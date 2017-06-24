use std::time::Instant;
use std::time::Duration;

use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;

use transposition::TranspositionTable;

use std::i32;

// use heuristic;

// #[macro_use]
// use common;

use search::negamax_ab_timeout;

///Implements the MTD(f) algorithm
///
/// * `bb` The root position to search from
/// * `h` The heuristic to use for searches
/// * `t` The current turn
/// * `g` A guess of the true minimax score.
/// * `d` The maximum depth to search to
/// * `ms_left` The number of milliseconds allocated to make the move
/// * `start` The time the search was started
/// * `to_flag` An out-parameter to signal when a timeout occured
pub fn mtdf_timeout<H : Heuristic>(
    ttbl    : &mut TranspositionTable, 
    bb      : Board, 
    h       : &mut H,
    mut g   : i32,
    d       : u8,
    ms_left : u64,
    start   : Instant,
    to_flag : &mut bool
) -> (Move, i32) {

    let mut best_move = Move::pass();
    let mut low = i32::MIN;
    let mut high = i32::MAX;
    
    while low < high {
        
        ////////////////////////////////////////////////////////////////////
        //do an alpha-beta search here
        ////////////////////////////////////////////////////////////////////
        let mut beta = g;
        
        if g == low {
            beta += 1;
        }
        
        g = negamax_ab_timeout(ttbl, bb, h, beta-1, beta, -1, d, 5, 
                               ms_left, start, to_flag, &mut best_move);
            
        if *to_flag {
            return (Move::null(), 0); 
        }
        
        ////////////////////////////////////////////////////////////////////
        //end alpha-beta search, get result. The result must not be a timeout
        ////////////////////////////////////////////////////////////////////
        
        //update MTD(f) bounds
        if g < beta {
            high = g;
        } else {
            low = g;
        }
    
    }
    
    (best_move, g)
}


///Iterative deepening implementation of MTD(f).
/// * `bb` The root position to search from
/// * `h` The heuristic to use for searches
/// * `t` The current turn
/// * `alpha` The lower bound of the minimax score
/// * `beta` The upper bound of the minimax score
/// * `d` The maximum depth to search to
/// * `ms_left` The number of milliseconds allocated to make the move
/// * `start` The time the search was started
/// * `to_flag` An out-parameter to signal when a timeout occured
pub fn mtdf_id_timeout<H : Heuristic>(
    bb      : Board,
    hs      : &mut [H; 20],
    t       : Turn,
    md      : u8,
    ms_left : u64
) -> Move {

    let start = Instant::now();
    let mut trans_table = TranspositionTable::new(20_000_000);

    //check if I have any moves
    if !bb.has_move().0 {
        cerrln!("[COIN]: Passing.");
        return Move::pass();
    }

    let empty = bb.total_empty();

    cerrln!("[COIN]: Starting MTD(f) search for {}!", t);
    
    let mut mv = Move::null();
    let mut f = 0;
    let mut d = 5;
    while d <= md && start.elapsed() < Duration::from_millis(ms_left) {
        
        //clear old transposition table
        trans_table.clear();
        //clears history heuristic for move ordering to match the current
        //depth of search
        //self.reinit_history();

        let mut hi = (if (empty as i16 - d as i16) < 0 {0} else {empty as i16 - d as i16}) / 3;

        let mut to = false;
        let (m, v) = mtdf_timeout(&mut trans_table, bb.copy(), &mut hs[hi as usize], 
                                  f, d, ms_left, start, &mut to);
        
        if to {
            cerrln!("[COIN]: TIMEOUT");
            break;
        }
        f = v;
        mv = m;
        
        cerrln!("[COIN]: Depth {:3}. \tBest move found: {:10} \t\tEstimated score: {:5} \tTable Size: {:8}", 
                d, mv, f, trans_table.size());
        
        //This allows us to search deeper as well as compare scores between
        //iterations more accurately
        d += 2;
    }
    
    let d = start.elapsed();
    cerrln!(
        "[COIN]: Done searching ({} s). Best move is {}", 
        (d.as_secs() as f32) + (d.subsec_nanos() as f32)/1_000_000_000f32,
        mv
    );
    
    return mv;
}