use players::*;
use std::path::Path;

use std::time::*;

pub struct MctsPlayer {
    mcts_m  : mcts::MctsTree<CoinNet>,
}

impl MctsPlayer {
    pub fn new(_s : Turn, model_path : &Path, params_path : &Path) -> MctsPlayer {
        let mut net = CoinNet::new(model_path).unwrap();
        net.load(params_path).unwrap();

        let mut mcts_m = MctsTree::new(net);
        mcts_m.set_temp(1.0);
        MctsPlayer {
            mcts_m,
        }
    }
}

impl Player for MctsPlayer {
    
    fn do_move(&mut self, b : Board, mut ms_left : u64) -> Move {
        let pieces = b.count_pieces();
        let total = pieces.0 + pieces.1;
        let empty = (64 - total) as u64;
        
        let solve_depth = 20;
        

        let ttime = Instant::now();
        let mut start = Instant::now();

        let mut moves = empty_movelist();
        let n = b.get_moves(&mut moves) as usize;

        eprintln!("[COIN] {} nodes in the tree.", self.mcts_m.count_sims());
        
        eprintln!("[COIN] {} ms remaining.", ms_left);
        
        let pruned = self.mcts_m.prune_board(b.clone());

        eprintln!("[COIN] Saved {} Nodes.", pruned);

        let mut timeout = false;
        
        let mut out_move = Move::null();
        
        if empty < solve_depth {
            eprintln!("[COIN] Attempting to solve the game.");
            let (m,s) = self.mcts_m.solve_endgame(start, Duration::from_millis(ms_left/4), &mut timeout);
        
            ms_left *= 3;
            ms_left /= 4;
            
            if timeout {
                eprintln!("[COIN] Timeout on endgame solver, researching with MCTS.");
                start = Instant::now();
            } else {
                out_move = m;
                eprintln!("[COIN] Solved game! Result: {} {}", s, m);
            }
        }
        
        let alloc_time = (ms_left as f32 * TIME_ALLOC[total as usize]) as u64;

        if timeout || empty >= solve_depth {
            eprintln!("[COIN] Searching...");
            //let expansions = self.mcts_m.time_rounds(alloc_time);
            let expansions = 100; self.mcts_m.n_rounds(200);
            eprintln!("[COIN] Done!");
            eprintln!("[COIN] Generated {} Nodes. ({} n/s)", expansions, expansions as f32 * 1000.0 / alloc_time as f32);

            let EvalOutput(output, score) = self.mcts_m.evaluate(&Board::new());
            eprintln!("[COIN] Score={:.3}", score);
            
            eprint!("[COIN] Main Line:");
            let main_line = self.mcts_m.scan(4);
            
            for mv in main_line {
                eprint!(" {}", mv); 
            }
            eprintln!("...");

            let mut mi = 0;
            let mut mx = output[moves[0].offset() as usize];

            for i in 1..n {
                let tmp = output[moves[i].offset() as usize];
                if mx < tmp {
                    mx = tmp;
                    mi = i;
                }
            }
            
            out_move = moves[mi];
        }


        if out_move.is_null() {
            eprintln!("[COIN] Something went wrong. Choosing random move.");
            let mut ml = empty_movelist();
            let n = b.get_moves(&mut ml) as usize;

            use rand;
            use rand::Rng;
            out_move = ml[rand::thread_rng().gen::<usize>()%n];
        }

        let self_pruned = self.mcts_m.prune(out_move);
        
        eprintln!("[COIN] Playing {} ({} nodes remaining)", out_move, self_pruned);
        
        let elapsed_time = {
            let e = ttime.elapsed();
            let s = e.as_secs();
            let ns = e.subsec_nanos();
            
            s * 1000 + ns as u64 / 1_000_000
        };
        
        eprintln!("[COIN] Turn took {} ms.", elapsed_time);

        out_move
    }
}
