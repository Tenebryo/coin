use bitboard::Board;
use bitboard::Move;

use transposition::TranspositionTable;

use heuristic::Heuristic;

use std::time::Instant;
use rand::Rng;
use rand::ThreadRng;
use rand;

pub trait Search<H : Heuristic> {
    fn search(&mut self, board : Board, ms_left : u64, start : Instant) -> Move;

    fn set_heuristic(&mut self, hr : Box<H>);
}

pub struct SearchInfo {
    pub tt  : TranspositionTable,
    pub to  : bool,
    pub st  : Instant,
    pub sr  : u64, 
    pub rn  : ThreadRng,
}

impl SearchInfo {
    pub fn new() -> SearchInfo {
        SearchInfo {
            tt: TranspositionTable::new(20_000_000),
            to: false,
            st: Instant::now(),
            sr: 0,
            rn: rand::thread_rng(),
        }
    }

    pub fn set_start(&mut self) {
        self.st = Instant::now();
    }

    #[inline]
    pub fn check_timeout(&mut self, msleft : u64) {
        use std::time::Duration;
        self.to = self.st.elapsed() >= Duration::from_millis(msleft);
    }
}