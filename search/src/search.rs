use bitboard::Board;
use bitboard::Move;

use heuristic::Heuristic;

use std::time::Instant;

pub trait Search<H : Heuristic> {
    fn search(&mut self, board : Board, ms_left : u64, start : Instant) -> Move;

    fn set_heuristic(&mut self, hr : Box<H>);
}