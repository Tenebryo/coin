use players::*;
use std::path::Path;

use std::time::*;

use std::sync::mpsc::*;
use std::thread::*;

enum PonderingMessage {
    Prune(Board),
    PruneMove(Move),
    TrySolve(u64),
    SolverTimeout,
    SolverMove(Move),
    GetMove(Board),
    SendMove(Move),
}

pub struct PonderingMctsPlayer {
    b_queue_tx  : Sender<PonderingMessage>,
    m_queue_rx  : Receiver<PonderingMessage>,
}

impl PonderingMctsPlayer {
    pub fn new(_s : Turn, model_path : String, params_path : String) -> PonderingMctsPlayer {
        
        let (b_queue_tx, b_queue_rx) = channel();
        let (m_queue_tx, m_queue_rx) = channel();
        
        spawn(move ||{
            let mut net = CoinNet::new(&Path::new(&model_path)).unwrap();
            net.load(&Path::new(&params_path)).unwrap();

            let mut mcts_m = MctsTree::new(net);
            mcts_m.set_temp(1.0);
        
            let mut total_count = 0;
            let mut last_count = 0;
            let mut ms_left = 300_000;
            let mut solver_time = 0;
            let mut search_start = Instant::now();
            
            loop {
                mcts_m.single_round();
            
                total_count += 1;
                last_count += 1;
            
                match b_queue_rx.try_recv() {
                    Ok(PonderingMessage::Prune(b)) => {
                        let nodes = mcts_m.count_sims();
                        let pruned = mcts_m.prune_board(b);
                        eprintln!("[COIN] {} nodes remaining of {}.", pruned, nodes);
                    },
                    Ok(PonderingMessage::PruneMove(mv)) => {
                        let nodes = mcts_m.count_sims();
                        let pruned = mcts_m.prune(mv);
                        eprintln!("[COIN] {} nodes remaining of {}.", pruned, nodes);
                    },
                    Ok(PonderingMessage::TrySolve(alloc_time)) => {
                        let mut timeout = false;
                        let start = Instant::now();
                        let time = Duration::from_millis(alloc_time);
                        eprintln!("[COIN] Attempting to solve the game.");
                        let (out_move, score) = mcts_m.solve_endgame(start, time, &mut timeout);
                        
                        if timeout {
                            eprintln!("[COIN] Timeout on endgame solver, researching with MCTS.");
                            m_queue_tx.send(PonderingMessage::SolverTimeout).unwrap();
                        } else {
                            eprintln!("[COIN] Solved game! Result: {} {}", score, out_move);
                            m_queue_tx.send(PonderingMessage::SolverMove(out_move)).unwrap();
                        }
                        solver_time += {
                            let e = start.elapsed();
                            let s = e.as_secs();
                            let ns = e.subsec_nanos();
                            (s * 1000) + (ns as u64/ 1_000_000)
                        };
                    },
                    Ok(PonderingMessage::GetMove(b)) => {
                        
                        let EvalOutput(output, score) = mcts_m.evaluate(&Board::new());
                        
                        let mut moves = empty_movelist();
                        let n = b.get_moves(&mut moves) as usize;
                        
                        let mut mi = 0;
                        let mut mx = output[moves[0].offset() as usize];

                        for i in 1..n {
                            let tmp = output[moves[i].offset() as usize];
                            if mx < tmp {
                                mx = tmp;
                                mi = i;
                            }
                        }
                        
                        let search_millis = {
                            let e = search_start.elapsed();
                            let s = e.as_secs();
                            let ns = e.subsec_nanos();
                            (s * 1000) + (ns as u64 / 1_000_000) - solver_time
                        };
                        search_start = Instant::now();
                        solver_time = 0;
                        
                        eprintln!("[COIN] Generated {} Nodes. ({} n/s)", last_count, last_count as f32 * 1000.0 / search_millis as f32);
                        eprintln!("[COIN] Score: {:.3}", score);
                        eprint!("[COIN] Main Line:");
                        let main_line = mcts_m.scan(5);
                        for mv in main_line {
                            eprint!(" {}", mv); 
                        }
                        eprintln!("...");
                        
                        m_queue_tx.send(PonderingMessage::SendMove(moves[mi]),).unwrap();
                    },
                    _ => {},
                }
            }
        });
        
        PonderingMctsPlayer {
            b_queue_tx,m_queue_rx
        }
    }
}

impl Player for PonderingMctsPlayer {
    
    fn do_move(&mut self, b : Board, mut ms_left : u64) -> Move {
        let pieces = b.count_pieces();
        let total = pieces.0 + pieces.1;
        let empty = (64 - total) as u64;
        
        let solve_depth = 21;
       
        let ttime = Instant::now();
        let mut start = Instant::now();
        
        self.b_queue_tx.send(PonderingMessage::Prune(b)).unwrap();

        let mut timeout = false;
        
        let mut out_move = Move::null();
        
        if empty < solve_depth {
        
            self.b_queue_tx.send(PonderingMessage::TrySolve(ms_left/4)).unwrap();
        
            ms_left -= {
                let e = start.elapsed();
                let s = e.as_secs();
                let ns = e.subsec_nanos();
                (s * 1000) + (ns as u64/ 1_000_000)
            };
            
            match self.m_queue_rx.recv() {
                Ok(PonderingMessage::SolverMove(mv)) => {out_move = mv;}
                _ => {
                    start = Instant::now();
                    timeout = true;
                }
            }
        }
        
        let alloc_time = (ms_left as f32 * TIME_ALLOC[total as usize]) as u64;

        if timeout || empty >= solve_depth {
            eprintln!("[COIN] Searching...");
            sleep(Duration::from_millis(alloc_time));
            eprintln!("[COIN] Done!");
            self.b_queue_tx.send(PonderingMessage::GetMove(b)).unwrap();
            
            match self.m_queue_rx.recv() {
                Ok(PonderingMessage::SendMove(mv)) => {out_move = mv;}
                _ => {}
            }
        }

        if out_move.is_null() {
            eprintln!("[COIN] Something went wrong. Choosing random move.");
            let mut ml = empty_movelist();
            let n = b.get_moves(&mut ml) as usize;

            use rand;
            use rand::Rng;
            out_move = ml[rand::thread_rng().gen::<usize>()%n];
        }
        
        self.b_queue_tx.send(PonderingMessage::PruneMove(out_move)).unwrap();
        
        eprintln!("[COIN] Playing {}", out_move);
        
        let elapsed_time = {
            let e = ttime.elapsed();
            let s = e.as_secs();
            let ns = e.subsec_nanos();
            (s * 1000) + (ns as u64 / 1_000_000)
        };
        
        eprintln!("[COIN] Turn took {} ms.", elapsed_time);

        out_move
    }
}
