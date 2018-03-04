#![allow(unused_imports)]

use std::io::prelude::*;
use std::fs::File;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;

use bitboard::bit_ops::popcount_64;
use bitboard::bit_ops::propagate;

// use pattern_engine::PatternSet;

use std::path::Path;
use serde_json;

use rand;
use rand::Rng;

pub trait Heuristic : Sized + Send + Sync + Clone {
    /// Evaluates the value of the given board on the turn of a given player.
    /// A board equal to a draw should evaluate to 0, positive for positions 
    /// that favor Black, and negative for positions that favor white. The
    /// magnitude of the score should reflect how favorable the position is.
    fn evaluate(&self, b : Board, t : Turn) -> i32;
    
    /// Orders moves based on how likely they are to be good moves, in order to
    /// speed up search algorithms.
    fn order_moves(&self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder);
}
///Very basic hueristic only counts discs
#[derive(Clone)]
pub struct BasicHeuristic {
}

impl BasicHeuristic {
    pub fn new() -> BasicHeuristic {
        BasicHeuristic{}
    }
}

impl Heuristic for BasicHeuristic {
    fn evaluate(&self, b : Board, t : Turn) -> i32 {
        let pieces = b.count_pieces();
        pieces.0 as i32 - pieces.1 as i32
    }
    
    fn order_moves(&self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}
///Very basic hueristic only counts discs
#[derive(Clone)]
pub struct ScaledBasicHeuristic {
    scale : i32,
}

impl ScaledBasicHeuristic {
    pub fn new(scale: i32) -> ScaledBasicHeuristic {
        ScaledBasicHeuristic{scale}
    }
}

impl Heuristic for ScaledBasicHeuristic {
    fn evaluate(&self, b : Board, t : Turn) -> i32 {
        let pieces = b.count_pieces();
        (pieces.0 as i32 - pieces.1 as i32) * self.scale
    }
    
    fn order_moves(&self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}

#[derive(Clone)]
pub struct WLDHeuristic {

}

impl WLDHeuristic {
    pub fn new() -> WLDHeuristic {
        WLDHeuristic{}
    }
}

impl Heuristic for WLDHeuristic {

    fn evaluate(&self, b : Board, t : Turn) -> i32 {
        let pieces = b.count_pieces();
        let df = (pieces.0 as i8) - (pieces.1 as i8);
        if df > 0 {
            1
        } else if df == 0 {
            0
        } else {
            -1
        }
    }
    
    
    
    fn order_moves(&self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}


// #[derive(Clone)]
// pub struct PatternHeuristic {
//     ps      : Box<PatternSet>,
// }

// impl PatternHeuristic {

//     pub fn from_pattern_set(ps : Box<PatternSet>) -> PatternHeuristic {
//         PatternHeuristic {
//             ps
//         }
//     }

//     pub fn file(filename : &Path) -> PatternHeuristic {
//         let mut f = File::open(filename).expect("Unable to read pattern file.");

//         let mut buf = String::new();
//         f.read_to_string(&mut buf).expect("Unable to read pattern file.");

//         let mut ps : PatternSet = serde_json::from_str(&buf).expect("Unable to parse pattern file.");
//         ps.trim(8*3);

//         PatternHeuristic {
//             ps : Box::new(ps),
//         }
//     }

//     pub fn random() -> PatternHeuristic {
//         let mut masks = vec![];

//         let mut r = rand::thread_rng();

//         for i in 0..12 {
//             let mut m : u64= 0;
//             while popcount_64(m) != 8 {
//                 m |= 1 << (r.gen::<u8>() % 64);
//             }

//             masks.push(m);
//         }

//         PatternHeuristic {
//             ps      : Box::new(PatternSet::from_masks(&masks)),
//         }
//     }
// }

// impl Heuristic for PatternHeuristic {
//     fn evaluate(&self, b : Board, t : Turn) -> i32 {
//         let p = b.pieces();
//         self.ps.eval(p.0, p.1) as i32
//     }
    
//     fn order_moves(&self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
//     }
// }



//Slightly more advanced heuristic weights different patterns
#[derive(Clone)]
pub struct HandmadeHeuristic {
}

impl HandmadeHeuristic {
    pub fn new() -> HandmadeHeuristic {
        HandmadeHeuristic {
        }
    }
}

impl Heuristic for HandmadeHeuristic {

    fn evaluate(&self, bb : Board, t : Turn) -> i32 {
    
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

        const piece_diff : [i32; 60] = [
            -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
            -1,-1,-1,-1,-1,-1,-1,-1,-1,-1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 2, 2, 3, 3, 4, 4, 5, 5, 1000,
        ];

        const mobility : [i32; 60] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            4, 4, 4, 5, 5, 5, 6, 6, 6, 7,
            7, 7, 8, 8, 8, 8, 8, 8, 8, 8,
            9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
            10,10,10,10,10,11,11,11,11,11,
            5, 5, 5, 5, 5, 0, 0, 0, 0, 0,
        ];
        
        //compensate for corners that are already used.
        const crn_occ_cmp : i32 = 48;
        
        let (b, w) = bb.pieces();

        let pieces = bb.total_pieces() as usize;
        
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
            use std::i32;
            return score.signum() * 1_000_000_000i32 + score;
        }
        
        score *= piece_diff[pieces - 4];

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
        let (bm, wm) = bb.mobility();
        score += (
            popcount_64(bm) as i32 - 
            popcount_64(wm) as i32
        ) * mobility[pieces - 4];
        
        
        score
    }
    
    fn order_moves(&self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
    
    }
}
// 








