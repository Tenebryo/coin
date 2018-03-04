use std::io::prelude::*;
use std::fs::File;

use players::*;
use search::*;
use pattern_engine::*;
use serde_json;

pub struct ABPlayer {
    sr  : NegamaxSearch<StagedHeuristic>,
}

impl ABPlayer {
    pub fn new(s : Turn) -> ABPlayer {
        let mut buf = vec![];

        File::open("./data/StagedHeuristic.json").unwrap().read_to_end(&mut buf).unwrap();

        let hs : StagedHeuristic = serde_json::from_slice(&buf).unwrap();

        ABPlayer {sr: NegamaxSearch::new(Box::new(hs),Instant::now())}
    }
}

impl Player for ABPlayer {
    
    fn do_move(&mut self, b : Board, ms_left : u64) -> Move {
        let pieces = b.count_pieces();
        let total = pieces.0 + pieces.1;
        let empty = (64 - total) as u64;
        

        let start = Instant::now();
        let alloc_time = (ms_left as f32 * TIME_ALLOC[total as usize]) as u64;

        eprintln!("[COIN] Searching...");

        let mut out_move = Move::null();

        self.sr.clear_transpositions();

        self.sr.st = Instant::now();
        let value = - self.sr.negamax(b, -1000, 1000, 5, alloc_time, &mut out_move);

        eprintln!("[COIN] Done! V={}", value);

        if out_move.is_null() {
            let mut ml = empty_movelist();
            let n = b.get_moves(&mut ml) as usize;

            use rand;
            use rand::Rng;
            out_move = ml[rand::thread_rng().gen::<usize>()%n];
        }

        out_move
    }
}
