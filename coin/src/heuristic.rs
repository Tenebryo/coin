use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;

use bitboard::bit_ops::popcount_64;
use bitboard::bit_ops::propagate;

pub trait Heuristic : Sized {
    /// Evaluates the value of the given board on the turn of a given player.
    /// A board equal to a draw should evaluate to 0, positive for positions 
    /// that favor Black, and negative for positions that favor white. The
    /// magnitude of the score should reflect how favorable the position is.
    fn evaluate(&mut self, b : Board, t : Turn) -> i32;
    
    /// Orders moves based on how likely they are to be good moves, in order to
    /// speed up search algorithms.
    fn order_moves(&mut self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder);
}

///Very basic hueristic only counts discs
pub struct HBasic {
}

impl Heuristic for HBasic {
    fn evaluate(&mut self, b : Board, t : Turn) -> i32 {
        b.count_pieces(Turn::BLACK) as i32 - b.count_pieces(Turn::WHITE) as i32
    }
    
    fn order_moves(&mut self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}


//Slightly more advanced heuristic weights different patterns
pub struct HPattern {
    pub piece_diff  : i32,
}

impl HPattern {
    pub fn new() -> HPattern {
        HPattern {
            piece_diff  : 1,
        }
    }
}

impl Heuristic for HPattern {

    fn evaluate(&mut self, bb : Board, t : Turn) -> i32 {
    
        /*
           A   B   C   D   E   F   G   H
         +---+---+---+---+---+---+---+---+
        1| a1| b1| c1| d1| d1| c1| b1| a1|
         +---+---+---+---+---+---+---+---+
        2| b1| b2| c2| d2| d2| c2| b2| b1|
         +---+---+---+---+---+---+---+---+
        3| c1| c2| c3| d3| d3| c3| c2| c1|
         +---+---+---+---+---+---+---+---+
        4| d1| d2| d3| d4| d4| d3| d2| d1|
         +---+---+---+---+---+---+---+---+
        5| d1| d2| d3| d4| d4| d3| d2| d1|
         +---+---+---+---+---+---+---+---+
        6| c1| c2| c3| d3| d3| c3| c2| c1|
         +---+---+---+---+---+---+---+---+
        7| b1| b2| c2| d2| d2| c2| b2| b1|
         +---+---+---+---+---+---+---+---+
        8| a1| b1| c1| d1| d1| c1| b1| a1|
         +---+---+---+---+---+---+---+---+
        */
        
        const a1 : u64 = 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001u64;
        const b1 : u64 = 0b01000010_10000001_00000000_00000000_00000000_00000000_10000001_01000010u64;
        const c1 : u64 = 0b00100100_00000000_10000001_00000000_00000000_10000001_00000000_00100100u64;
        const d1 : u64 = 0b00011000_00000000_00000000_10000001_10000001_00000000_00000000_00011000u64;
        const b2 : u64 = 0b00000000_01000010_00000000_00000000_00000000_00000000_01000010_00000000u64;
        const c2 : u64 = 0b00000000_00100100_01000010_00000000_00000000_01000010_00100100_00000000u64;
        const d2 : u64 = 0b00000000_00011000_00000000_01000010_01000010_00000000_00011000_00000000u64;
        const c3 : u64 = 0b00000000_00000000_00100100_00000000_00000000_00100100_00000000_00000000u64;
        const d3 : u64 = 0b00000000_00000000_00011000_00100100_00100100_00011000_00000000_00000000u64;
        const d4 : u64 = 0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000u64;
        
        const a1_w : i32 = 256;
        const b1_w : i32 = -32;
        const c1_w : i32 = 16;
        const d1_w : i32 = 8;
        const b2_w : i32 = -32;
        const c2_w : i32 = 4;
        const d2_w : i32 = -4;
        const c3_w : i32 = 8;
        const d3_w : i32 = 2;
        const d4_w : i32 = 1;
        
        //compensate for corners that are already used.
        const crn_occ_cmp : i32 = 48;
        
        let b = bb.pieces(Turn::BLACK);
        let w = bb.pieces(Turn::WHITE);
        
        //get the corners occupied by each player
        let crn_occ = (b | w) & a1;
        
        //get the corner access 
        let crn_occ_msk = propagate(crn_occ) & (!a1);
        
        let mut score = 0;
        
        //the numn
        //score += (b.count_pieces(Turn::BLACK) as i32 - b.count_pieces(Turn::WHITE) as i32)*1;
        
        score += ((popcount_64(b) as i32) - (popcount_64(w) as i32));
        
        if bb.is_done() {
            //prevent wipeouts
            return score * 4096;
        }
        
        score *= self.piece_diff;

        score += a1_w * ((popcount_64(b & a1) as i32) - (popcount_64(w & a1) as i32));
        score += b1_w * ((popcount_64(b & b1) as i32) - (popcount_64(w & b1) as i32));
        score += c1_w * ((popcount_64(b & c1) as i32) - (popcount_64(w & c1) as i32));
        score += d1_w * ((popcount_64(b & d1) as i32) - (popcount_64(w & d1) as i32));
        score += b2_w * ((popcount_64(b & b2) as i32) - (popcount_64(w & b2) as i32));
        score += c2_w * ((popcount_64(b & c2) as i32) - (popcount_64(w & c2) as i32));
        score += d2_w * ((popcount_64(b & d2) as i32) - (popcount_64(w & d2) as i32));
        score += c3_w * ((popcount_64(b & c3) as i32) - (popcount_64(w & c3) as i32));
        score += d3_w * ((popcount_64(b & d3) as i32) - (popcount_64(w & d3) as i32));
        score += d4_w * ((popcount_64(b & d4) as i32) - (popcount_64(w & d4) as i32));
        
        //this ensures that if the corners are already taken, the corner access
        //squares are no longer counted as negative.
        score += crn_occ_cmp * ((popcount_64(b & crn_occ_msk) as i32) - (popcount_64(w & crn_occ_msk) as i32));
    
        /*        
        //count corners and uncapture corner access
        score += (popcount_64(bcrn) as i32 - popcount_64(wcrn) as i32) * 50;
        score += (popcount_64(bcac) as i32 - popcount_64(wcac) as i32) * -20;
        score += (popcount_64(bedg) as i32 - popcount_64(wedg) as i32) * 18;
        
        score += (
            popcount_64(b.stability(Turn::BLACK)) as i32 - 
            popcount_64(b.stability(Turn::WHITE)) as i32
        ) * 40;
        // */
        //count mobility
        score += (
            popcount_64(bb.mobility(Turn::BLACK)) as i32 - 
            popcount_64(bb.mobility(Turn::WHITE)) as i32
        ) * 8;
        
        
        score
    }
    
    fn order_moves(&mut self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}

pub struct HWLD {

}

impl Heuristic for HWLD {

    fn evaluate(&mut self, b : Board, t : Turn) -> i32 {
        let df = (b.count_pieces(Turn::BLACK) as i8) - (b.count_pieces(Turn::WHITE) as i8);
        if df > 0 {
            1
        } else if df == 0 {
            0
        } else {
            -1
        }
    }
    
    
    
    fn order_moves(&mut self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}









