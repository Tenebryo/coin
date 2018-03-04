use std::time::Instant;
use std::time::Duration;

use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;

use transposition::TranspositionTable;

use std::i32;

macro_rules! cerrln(
    ($($arg:tt)*) => { {
        use std::io::Write;
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);


use NegamaxSearch;

///Implements the MTD(f) algorithm
///
/// * `ng` The negamax search object
/// * `bb` The root position to search from
/// * `g` A guess of the true minimax score.
/// * `d` The maximum depth to search to
/// * `ms_left` The number of milliseconds allocated to make the move
pub fn mtdf_timeout<H : Heuristic>(
    ng      : &mut NegamaxSearch<H>,
    bb      : Board, 
    mut g   : i32,
    d       : u8,
    ms_left : u64,
) -> (Move, i32) {

    let mut best_move = Move::pass();
    let mut low = i32::MIN;
    let mut high = i32::MAX;

    let mut bounds = [i32::MIN, i32::MAX];
    
    while bounds[0] < bounds[1] {
        
        ////////////////////////////////////////////////////////////////////
        //do an alpha-beta search here
        ////////////////////////////////////////////////////////////////////

        let mut test = if g == bounds[0] { g+1 } else { g };
        
        let mut tmp_mv = Move::null();

        g = ng.negamax(bb, test-1, test, d, ms_left, &mut tmp_mv);
        
        ////////////////////////////////////////////////////////////////////
        //end alpha-beta search, get result. The result must not be a timeout
        ////////////////////////////////////////////////////////////////////
        
        //update MTD(f) bounds
        bounds[(g < test) as usize] = g;

        if ng.timeout() {
            return (Move::null(), 0); 
        }

        best_move = tmp_mv;
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
pub fn mtdf_id_timeout<H : Heuristic + Clone>(
    bb      : Board,
    hs      : Box<H>,
    md      : u8,
    ms_left : u64
) -> Move {

    let start = Instant::now();

    //check if I have any moves
    if !bb.has_move().0 {
        cerrln!("[COIN]: Passing.");
        return Move::pass();
    }

    let empty = bb.total_empty();

    cerrln!("[COIN]: Starting MTD(f) search!");
    // cerrln!("[COIN]: Starting MTD(f) search for {}!", bb.get_turn());
    
    let mut mv = Move::null();
    let mut f = 0;
    let mut d = 5;

    cerrln!("[COIN]: | {: <5} | {: <9} | {: <13} | {: <14} | {: <14} | {: <12} |",
            "Depth", "Best Move", "Minimax Value", "Transpositions", "Nodes Searched", "Time Elapsed");
    cerrln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");

    while d <= md && start.elapsed() < Duration::from_millis(ms_left) {
        
        //clears history heuristic for move ordering to match the current
        //depth of search
        //self.reinit_history();


        let mut ng = NegamaxSearch::new(hs.clone(), start);
        let (m,v) = mtdf_timeout(&mut ng, bb.copy(), f, d, ms_left);
        let mut tr = ng.get_transpositions();
        let mut sr = ng.get_searched();
        let mut to = ng.timeout();


        
        if to {
            cerrln!("[COIN]: TIMEOUT");
            break;
        }
        f = v;
        mv = m;

        let elapsed = {
            let d = start.elapsed();
            (d.as_secs() as f32) + (d.subsec_nanos() as f32)/1_000_000_000f32
        };
        
        cerrln!("[COIN]: | {: >5} | {: >9} | {: >13} | {: >14} | {: >14} | {: >12.2} |",
                d, format!("{}",mv), f, tr, sr, elapsed);
        
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