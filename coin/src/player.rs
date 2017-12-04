use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use mcts;
use mcts::*;

use std::path::Path;
use std::time::Instant;

pub struct Player {
    mcts_m  : mcts::MctsTree<CoinNet>,
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
        let mut net = CoinNet::new("./mcts/data/CoinNet_model.pb").unwrap();
        net.load(Path::new("./mcts/data/iter5/CoinNet-checkpoint.best.5")).unwrap();

        let mut mcts_m = MctsTree::new(net);
        mcts_m.set_temp(1.0);
        Player {
            mcts_m,
        }
    }
    
    pub fn do_move(&mut self, b : Board, ms_left : u64) -> Move {
        let pieces = b.count_pieces();
        let total = pieces.0 + pieces.1;
        let empty = (64 - total) as u64;
        

        let start = Instant::now();
        let alloc_time = (ms_left as f32 * time_alloc[total as usize]) as u64;

        let mut moves = empty_movelist();
        let n = b.get_moves(&mut moves) as usize;

        self.mcts_m.prune_board(b.clone());

        eprintln!("[COIN] Searching...");
        self.mcts_m.time_rounds(alloc_time);
        eprintln!("[COIN] Done!");

        let EvalOutput(output, score) = self.mcts_m.evaluate(&[Board::new()]);
        eprintln!("[COIN] Score={:.3}", score);


        let mut mi = 0;
        let mut mx = output[moves[0].offset() as usize];

        for i in 1..n {
            let tmp = output[moves[i].offset() as usize];
            if mx < tmp {
                mx = tmp;
                mi = i;
            }
        }

        let mut out_move = moves[mi];

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
