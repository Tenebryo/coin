use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;

use bitboard::bit_ops::popcount_64;

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
}

impl Heuristic for HPattern {
    fn evaluate(&mut self, b : Board, t : Turn) -> i32 {
        const crn : u64 = 0x81_00_00_00_00_00_00_81u64;
        const cac : u64 = 0xC3_C3_00_00_00_00_C3_C3u64;
        
        let mut score = 0;
        
        score += b.count_pieces(Turn::BLACK) as i32 - b.count_pieces(Turn::WHITE) as i32;
        
        if b.is_done() {
            //prevent wipeouts
            return score * 1000;
        }
        
        score += (popcount_64(b.stability(Turn::BLACK)) as i32 - popcount_64(b.stability(Turn::WHITE)) as i32);
        score += (popcount_64(b.mobility(Turn::BLACK)) as i32 - popcount_64(b.mobility(Turn::WHITE)) as i32);
        
        score
    }
    
    fn order_moves(&mut self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}
