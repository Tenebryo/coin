use heuristic::WLDHeuristic;
use heuristic::BasicHeuristic;
use heuristic::PatternHeuristic;
use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;
use search::mtdf_id_timeout;

use std::path::Path;
use std::time::Instant;



pub struct Player {
    phs     : [PatternHeuristic; 20],
}

const time_alloc : [f32; 64] = [
    0.00,   0.00,   0.00,   0.00,   0.01,   0.05,   0.05,   0.05,
    0.05,   0.05,   0.05,   0.05,   0.05,   0.05,   0.05,   0.05,
    0.03,   0.03,   0.03,   0.03,   0.03,   0.03,   0.03,   0.03,
    0.10,   0.10,   0.10,   0.10,   0.10,   0.10,   0.10,   0.10,
    0.12,   0.12,   0.12,   0.12,   0.12,   0.12,   0.12,   0.12,
    0.14,   0.14,   0.14,   0.14,   0.14,   0.14,   0.14,   0.14,
    0.15,   0.15,   0.15,   0.15,   0.30,   0.30,   0.30,   0.30,
    0.70,   0.70,   0.70,   0.70,   0.95,   0.95,   0.95,   0.95,
];

impl Player {
    pub fn new(s : Turn) -> Player {
        Player {
            phs     : [
                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(),

                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(), 
                PatternHeuristic::random(), PatternHeuristic::random(),
            ],
        }
    }
    
    pub fn do_move(&mut self, b : Board, ms_left : u64) -> Move {
        let pieces = b.count_pieces();
        let total = pieces.0 + pieces.1;
        let empty = (64 - total) as u64;
        

        let start = Instant::now();
        let alloc_time = (ms_left as f32 * time_alloc[total as usize]) as u64;

        
        let mut out_move = mtdf_id_timeout(b, &mut self.phs, Turn::BLACK, 30, alloc_time);


        if out_move.is_null() {
            let mut ml = empty_movelist();
            b.get_moves(&mut ml);

            out_move = ml[0];
        }

        out_move
        
        // if empty > 18 {
        //     //somehow, this makes it play well...
        //     //optimizing for few pieces
        //     self.h_mid.piece_diff = ((16-empty)>>1) as i32;
        //     // If we are still in the mid-game, use MTD(f)
        //     self.se.mtdf_id(
        //         b.copy(), &mut self.h_mid, self.t, 21, 2*ms_left/empty
        //     )
        // } else {
        //     // If we are close to the end, try solving the game
        //     let mut to = false;
        //     let mut mv = Move::null();
        //     cerrln!("[COIN]: Trying solver...");
        //     let start = Instant::now();
        //     let (al,bt) = match self.t {
        //         Turn::BLACK => (0,2),
        //         Turn::WHITE => (-2,0),
        //     };
            
        //     //we allot a quarter of the remaining time to solving
        //     let g = self.se.alpha_beta_with_timeout(
        //         b.copy(),&mut self.h_end,self.t,al,bt,2*empty as u8,(ms_left/4),start,&mut to, &mut mv
        //     );
            
        //     if to || g >= bt || g <= al || mv.is_null() {
        //         if to {
        //             cerrln!("[COIN]: Solver TIMEOUT, reverting to MTD(f)");
        //         } else if g >= bt || g <= al {
        //             cerrln!("[COIN]: Bounds miss, reverting to MTD(f)");
        //         } else {
        //             cerrln!("[COIN]: Other error, reverting to MTD(f)");
        //         }
        //         //change the value of having more pieces as we get closer to the
        //         //end of the game
        //         //self.h_mid.piece_diff = ((24-empty)>>1) as i32;
        //         //we either timed out or couldn't find a guaranteed win so we
        //         //switch back to MTD(f) and try again next time.
        //         self.se.mtdf_id(
        //             b.copy(), &mut self.h_mid, self.t, 21, 3*ms_left/empty/2
        //         )
        //     } else {
        //         cerrln!("[COIN]: Found Solution, score: {}, move: {}", g, mv);
        //         mv
        //     }
        // }
    }
}
