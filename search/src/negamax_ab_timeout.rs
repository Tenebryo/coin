use std::time::Instant;
use std::time::Duration;

use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;
use bitboard::MAX_MOVES;

use transposition::TranspositionTable;

use std::i32;

// use heuristic;

// #[macro_use]
// use common;

///Negamax, fixed-depth, alpha-beta, with timeout
pub fn negamax_ab_timeout<H: Heuristic> (
    trans_table : &mut TranspositionTable,
    bb          : Board,
    h           : &mut H,
    mut alpha   : i32,
    mut beta    : i32,
    color       : i8,
    d           : u8,
    d_cutoff    : u8,
    ms_left     : u64,
    start       : Instant,
    to_flag     : &mut bool,
    out_move    : &mut Move
) -> i32 {
    ///TODO: limit memory
    {
        let (l,h) = trans_table.fetch(bb);

        if l >= beta  { return l; }
        alpha = if l > alpha {l} else {alpha};

        if h <= alpha { return h; }
        beta = if h < beta {h} else {beta};
    }

    if bb.is_done() || d == 0 {
        *out_move = Move::pass();
        return color as i32 * h.evaluate(bb, Turn::BLACK);
    }

    let mut rmvs : MoveList = [Move::null(); MAX_MOVES];
    let mut omvs : MoveOrder = [(0i32, 0); MAX_MOVES];
    
    let n = bb.get_moves(&mut rmvs);

    //negamax step
    let mut g = i32::MIN;

    //loop through all the moves
    for i in 0..n {
        let mut bc = bb.copy();
        let m = rmvs[i as usize];
        bc.do_move(m);

        //recurse, updating alpha and beta appropriately.
        let v = if d > d_cutoff {
            //continue checking timeout
            -negamax_ab_timeout(
                trans_table, bc, h,     -beta, -alpha, -color, 
                d-1, d_cutoff,          ms_left, start, to_flag, &mut Move::null() 
            )
        } else {
            //switch to cheaper negamax to avoid checking timout often
            -negamax_ab(
                trans_table, bc, h, -beta, -alpha, -color, 
                d-1,                &mut Move::null()
            )
        };

        //break out if we have used all of our time
        if *to_flag || start.elapsed() >= Duration::from_millis(ms_left) {
            *to_flag = true;
            return 0;
        }

        //update best move
        if v > g {
            g = v;
            //cerrln!("g: {}", g);
            *out_move = m; //rmvs[omvs[i as usize].1];
        }

        alpha = if alpha > g {alpha} else {g};

        if alpha >= beta {
            break;
        }
    }

    let mut low = i32::MIN;
    let mut high = i32::MAX;
    
    if g <= alpha {high = g;}
    if g > alpha && g < beta {high = g; low = g;}
    if g >= beta {low = g;}
    
    trans_table.update(bb, low, high);
    
    g
}

///Negamax, fixed-depth, alpha-beta, without timeout for lower depth searches
///also can be used to ignore transposition table when it is slower than just
///searching everything
pub fn negamax_ab<H: Heuristic> (
    trans_table : &mut TranspositionTable,
    bb          : Board,
    h           : &mut H,
    mut alpha   : i32,
    mut beta    : i32,
    color       : i8,
    d           : u8,
    out_move    : &mut Move
) -> i32 {
    {
        let (l,h) = trans_table.fetch(bb);

        if l >= beta  { return l; }
        alpha = if l > alpha {l} else {alpha};

        if h <= alpha { return h; }
        beta = if h < beta {h} else {beta};
    }

    if bb.is_done() || d == 0 {
        *out_move = Move::pass();
        return color as i32 * h.evaluate(bb, Turn::BLACK);
    }

    let mut rmvs : MoveList = [Move::null(); MAX_MOVES];
    let mut omvs : MoveOrder = [(0i32, 0); MAX_MOVES];
    
    let n = bb.get_moves(&mut rmvs);

    //negamax step
    let mut g = i32::MIN;

    //loop through all the moves
    for i in 0..n {
        let mut bc = bb.copy();
        let m = rmvs[i as usize];
        bc.do_move(m);

        //recurse, updating alpha and beta appropriately.
        let v = -negamax_ab(
            trans_table, bc, h, -beta, -alpha, -color, 
            d-1, &mut Move::null()
        );

        //update best move
        if v > g {
            g = v;
            //cerrln!("g: {}", g);
            *out_move = m; //rmvs[omvs[i as usize].1];
        }

        alpha = if alpha > g {alpha} else {g};

        if alpha >= beta {
            break;
        }
    }

    let mut low = i32::MIN;
    let mut high = i32::MAX;
    
    if g <= alpha {high = g;}
    if g > alpha && g < beta {high = g; low = g;}
    if g >= beta {low = g;}
    
    trans_table.update(bb, low, high);
    
    g
}