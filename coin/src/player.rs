use search::SearchEngine;
use heuristic::Heuristic;
use heuristic::HBasic;
use heuristic::HPattern;
use heuristic::HWLD;
use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;

use std::time::Instant;


pub struct Player {
    t       : Turn,
    se      : SearchEngine,
    h_mid   : HPattern,
    h_end   : HWLD,
}

impl Player {
    pub fn new(s : Turn) -> Player {
        Player {
            t       : s,
            se      : SearchEngine::new(s),
            h_mid   : HPattern::new(),
//            h_mid   : MLHeuristic::new(
//                "../data/value_net_v1/value_net.index".to_string(),
//                "../data/policy_net_v1/policy_net.index".to_string()
//            ),
            h_end   : HWLD{},
        }
    }
    
    pub fn do_move(&mut self, b : Board, ms_left : u64) -> Move {
        let empty = (64 - b.count_pieces(Turn::BLACK) - b.count_pieces(Turn::WHITE)) as u64;
        
        
        if empty > 20 {
            self.h_mid.piece_diff = ((16-empty)>>1) as i32;
            // If we are still in the mid-game, use MTD(f)
            self.se.mtdf_id(
                b.copy(), &mut self.h_mid, self.t, 21, 2*ms_left/empty
            )
        } else {
            // If we are close to the end, try solving the game
            let mut to = false;
            let mut mv = Move::null();
            cerrln!("[COIN]: Trying solver...");
            let start = Instant::now();
            let (al,bt) = match self.t {
                Turn::BLACK => (0,2),
                Turn::WHITE => (-2,0),
            };
            
            //we allot a quarter of the remaining time to solving
            let g = self.se.alpha_beta_with_timeout(
                b.copy(),&mut self.h_end,self.t,al,bt,2*empty as u8,(ms_left/4),start,&mut to, &mut mv
            );
            
            if to || g >= bt || g <= al || mv.is_null() {
                cerrln!("[COIN]: TIMEOUT or bounds miss, reverting to MTD(f)");
                //we either timed out or couldn't find a guaranteed win so we
                //switch back to MTD(f) and try again next time.
                self.se.mtdf_id(
                    b.copy(), &mut self.h_mid, self.t, 21, 3*ms_left/empty/2
                )
            } else {
                cerrln!("[COIN]: Found Solution, score: {}, move: {}", g, mv);
                mv
            }
        }
    }
}
