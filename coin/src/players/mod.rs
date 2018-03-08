use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use mcts;
use mcts::*;


use std::path::Path;
use std::time::Instant;

mod mcts_player;
mod pondering_mcts_players;
mod mtdf_player;
mod ab_player;

pub use self::mcts_player::*;
pub use self::pondering_mcts_players::*;
pub use self::mtdf_player::*;
pub use self::ab_player::*;

pub trait Player {
    fn do_move(&mut self, b : Board, ms_left : u64) -> Move;
}

const TIME_ALLOC : [f32; 64] = [
    0.00,   0.00,   0.00,   0.00,   0.01,   0.05,   0.05,   0.05,
    0.05,   0.05,   0.05,   0.05,   0.05,   0.05,   0.05,   0.05,
    0.03,   0.03,   0.03,   0.03,   0.03,   0.03,   0.03,   0.03,
    0.10,   0.10,   0.10,   0.10,   0.10,   0.10,   0.10,   0.10,
    0.12,   0.12,   0.12,   0.12,   0.12,   0.12,   0.12,   0.12,
    0.14,   0.14,   0.14,   0.14,   0.14,   0.14,   0.14,   0.14,
    0.15,   0.15,   0.15,   0.15,   0.30,   0.30,   0.30,   0.30,
    0.70,   0.70,   0.70,   0.70,   0.95,   0.95,   0.95,   0.95,
];
