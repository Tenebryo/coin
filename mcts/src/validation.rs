use rand::{self, Rng};
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::io::stdout;
use std::fs;
use std::fs::File;

use std::mem;
use std::error::Error;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::*;
use std::sync::atomic::*;

use std::time::Instant;
use std::time::Duration;

use bincode;
use serde_json;

use threadpool::*;
use scoped_threadpool::Pool;

use bitboard::*;
use eval::*;
use mcts::*;
use game::*;

const VALIDATION_GAMES = 32;

pub struct Validator {
    players      : Vec<(usize, Arc<ParallelCoinNetWorker>, MctsTree<ParallelCoinNet>)>,
}

impl Validator {
    pub fn new(interval : usize) -> Result<Validator,Error> {
        use std::cmp::max;

        // find all the training iteration files
        let mut p_iter_dirs = vec![];
        for dir in fs::read_dir(dir)? {
            let entry = dir?;
            if entry.file_name().to_str().unwrap().starts_with("iter") {
                p_iter_dirs.push(entry.path());
            }
        }

        let n = p_iter_dirs.len();

        // create an array of empty 
        let mut players = (0..n).map(|_| {

            let (evals, worker) = ParallelCoinNet::new_worker_group(model, vars).unwrap();

            let evals = MctsTree::new(evals);

            (0, worker, evals)
        }).collect::<Vec<_>>();


        let mut dirs = p_iter_dirs.iter()
            .enumerate()
            .skip((n-1) % interval)
            .step_by(interval)
            .enumerate();


        for (i, (j, dir)) in dirs {
            let prefix= format!("CoinNet-checkpoint.best");
            players[i].0 = j;
            players[i].1.load(&dir.join(prefix))?;
        }

        Validator{
            players
        }
    }

    pub fn validate_networks(&mut self) {
        
    }

    fn validation_match(&mut self, p1 : usize, p2 : usize) -> (usize, usize) {
        let start = Instant::now();
        println!("[COIN]   Validation Match: {} against {}", p1, p2);

        let mut tpool = Pool::new(TF_EVAL_BATCH_SIZE as u32 + 1);

        let wins = Arc::new(AtomicUsize::new(0));
        let running = Arc::new(AtomicUsize::new(0));

        tpool.scoped(|tpool| {

            let (ref worker1, ref eval1) = self.players[p1];
            let (ref worker2, ref eval2) = self.players[p2];
            
            let mut par = true;
            for _i in 0..TF_EVAL_BATCH_SIZE {
                let wins = wins.clone();
                let running = running.clone();
                running.fetch_add(1, Ordering::SeqCst);
                //schedule a game on the threadpool. # of games >>> # of threads,
                //since the bottleneck is the TF evaluations
                let (p1, p2) = (eval1.clone(), eval2.clone());

                if par {
                    tpool.execute(move || {
                        let r = Self::validation_game(p1, p2);
                        if r < 0.0 {
                            wins.fetch_add(1, Ordering::SeqCst);
                        }
                        running.fetch_sub(1, Ordering::SeqCst);
                    });
                } else {
                    tpool.execute(move || {
                        let r = Self::validation_game(p2, p1);
                        if r > 0.0 {
                            wins.fetch_add(1, Ordering::SeqCst);
                        }
                        running.fetch_sub(1, Ordering::SeqCst);
                    });
                }
                par = !par;
            }


            let mut p1_steps = 0;
            let mut p2_steps = 0;
            while running.load(Ordering::SeqCst) != 0 {
                p1_steps += worker1.do_a_work();
                p2_steps += worker2.do_a_work();
                print!("\r[COIN]         Total Evals: [P1,P2] = [{:8}, {:8}] ({:4.1}%)", 
                    p1_steps, p2_steps, (p1_steps + p2_steps) as f32 / (0.6 * (TF_EVAL_BATCH_SIZE * EVAL_ROUNDS) as f32));
                stdout().flush().unwrap();
            }
            println!("");
        });

        let wins = wins.load(Ordering::SeqCst);

        let elapsed = start.elapsed();
        println!("\r[COIN]     Result: {} to {}", wins, VALIDATION_GAMES  - wins);
        println!("\r[COIN]     Time elapsed: {:5}.{:09}", 
            elapsed.as_secs(), elapsed.subsec_nanos());

        (VALIDATION_GAMES  - wins, wins)
    }

    pub fn validation_game(mut p1 : MctsTree<ParallelCoinNet>, mut p2 : MctsTree<ParallelCoinNet>) {
        let mut rng = rand::thread_rng();
        let mut b = Board::new();

        let mut turn = 1.0;
        // let mut t = 1;

        /*  Initialize each player for the game. */
        p1.set_position(b);
        p2.set_position(b);
        p1.set_temp(0.25);
        p2.set_temp(0.25);

        /*  Simulate the game. */
        while !b.is_done() {

            let mut moves = empty_movelist();
            let n = b.get_moves(&mut moves);

            let mut selected_move = Move::pass();

            if n != 0 {
                let val = if turn > 0.0 {
                    p1.n_rounds(EVAL_ROUNDS);
                    p1.evaluate(&b)
                } else {
                    p2.n_rounds(EVAL_ROUNDS);
                    p2.evaluate(&b)
                };


                let sum : f32 = (0..(n as usize)).map(|i| val.0[moves[i].offset() as usize]).sum();

                assert!(sum >= 0.0);

                let rmove = rng.gen_range(0.0, sum);

                let mut mi = 0;
                let mut sm = 0.0;

                for i in 0..(n as usize) {
                    sm += val.0[moves[i].offset() as usize];
                    if rmove <= sm {
                        mi = i;
                        break;
                    }
                }

                selected_move = moves[mi];
            }

            /*  Apply the move to the board and each of the players. */
            b.f_do_move(selected_move);

            p1.prune(selected_move);
            p2.prune(selected_move);

            turn = -turn;
        }

        turn * b.piece_diff() as f32 / 64.0
    }
}
