use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;
use bitboard::MAX_MOVES;

use transposition::TranspositionTable;

use std::i32;
use std::time::Instant;
use std::time::Duration;

pub struct AlphaBetaData<H: Heuristic> {
    trans   : TranspositionTable,
    hrstc   : 
    start   : Instant,
    nodes   : u64,
    depth   : u8,
}


fn alpha_beta_timeout<H: Heuristic>(
    o       : &mut AlphaBetaData,
    n       : Board,
    d       : u8,
    a       : i32,
    b       : i32,
    c       : i32,
) -> i32 {




    if b.is_done() || d == 0 {
        return h.evaluate(b, Turn::BLACK);
    }


    if 
}