use std::io::prelude::*;
use std::io::stdout;
use std::fs::File;
use std::path::Path;

use std::cmp::min;
use std::i32;

use serde_json;

use wthor::*;
use bitboard::*;

use rand;
use rand::Rng;

use pattern_engine::*;

const LAMBDA : f32 = 0.01;

fn logistic_loss(p : f32, y : f32) -> f32 {
    let r = (1.0 + (-y * p).exp()).ln();
    if !r.is_normal() {
        10000000.0 * r.signum()
    } else {
        r
    }
}

fn logistic_partial_grad(p : f32, y : f32) -> f32 {
    let denom = 1.0 + (p * y).exp();
    let numer = - y;

    numer/denom
}

#[allow(dead_code)]
fn train_staged_heuristic_supervised(
    // database: &Path, 
    positions : &mut Vec<(Board,f32)>,
    training_epochs : usize, 
    eta : f32,
    batch_size : usize
) -> StagedHeuristic {

    let mut hs = StagedHeuristic::new();

    let n = positions.len();
    let mut r = rand::thread_rng();

    eprintln!("[COIN] Starting training...");

    //do logistic regression:
    for epoch in 0..training_epochs {

        let batches = ((n-1) as f32 /batch_size as f32 + 1.0).floor() as usize;

        r.shuffle(positions);

        let mut mean_loss = 0.0;
        for b in 0..batches {
            let grads = positions[b..min(n, b+batch_size)].iter().map(|&(pos, y)| {
                let (p,syms) = hs.eval_raw(pos);

                let y = y.signum();


                let loss = logistic_loss(p, y);
                mean_loss += loss as f64 / n as f64;
                // eprintln!("[COIN] Mean Loss = {:8.4} Loss = {:8.4} P = {:8.4} Y = {}", mean_loss, loss, p, y);

                (pos, syms, logistic_partial_grad(p, y))
            }).collect::<Vec<_>>();

            for (pos, syms, gb) in grads {
                hs.adjust_fn(pos, syms, &mut |w| {eta * (LAMBDA * w + gb) / batch_size as f32});
            }
        }
        eprintln!("[COIN] Epoch {:3}: Mean Loss = {}", epoch, mean_loss);
    }

    hs
}

//simple negamax implementation.
fn negamax_opt (
    hs          : &mut StagedHeuristic,
    bb          : Board,
    mut alpha   : i32,
    beta        : i32,
    d           : u8,
) -> i32 {

    if bb.is_done() {
        return (bb.piece_diff() as i32);
    } else if d == 0 {
        return hs.eval(bb.copy()) as i32;
    }

    let mut rmvs : MoveList = empty_movelist();

    let n = bb.get_moves(&mut rmvs);

    //negamax step
    let mut g = i32::MIN;

    //loop through all the moves
    for i in 0..n {
        let mut bc = bb.copy();
        let m = rmvs[i as usize];
        bc.f_do_move(m);

        //recurse, updating alpha and beta appropriately.
        let v = -negamax_opt(hs, bc, -beta, -alpha, d-1 );

        //update best move
        if g < v { g = v; }

        if alpha < g { alpha = g; }

        if alpha >= beta { break; }
    }

    g
}

#[allow(dead_code)]
fn train_staged_heuristic_reinforcement(hs : &mut StagedHeuristic, positions : &mut Vec<(Board, f32)>) {

    let mut r = rand::thread_rng();

    for level in 0..15 {
        let empty = 4*level;

        let mut mvs = empty_movelist();
        eprintln!("[COIN] Generating new positions for e={}...", empty);
        for i in 0..1000 {
            eprint!("\r[COIN] Game {}", i);
            stdout().flush();
            let mut b = Board::new();

            while b.total_empty() > empty {
                if b.is_done() {
                    b = Board::new();
                }

                let mut n = b.get_moves(&mut mvs) as usize;
                if n == 0 {
                    mvs[0] = Move::pass();
                    n = 1;
                }
                let mv = if r.gen::<f32>() > 0.2 && b.total_empty() > 30 + empty/2 {
                    
                    let mut best_move = mvs[0];
                    let mut best_value = {
                        let mut bc = b.copy();
                        bc.f_do_move(mvs[0]);
                        -negamax_opt(hs, bc, -1000, 1000, 4)
                    };

                    for &mv in &mvs[1..n] {
                        let mut bc = b.copy();
                        bc.f_do_move(mv);
                        let value = -negamax_opt(hs, bc, -1000, 1000, 4);

                        if value > best_value {
                            best_value = value;
                            best_move = mv;
                        }
                    }


                    positions.push((b,best_value as f32));
                    best_move
                } else {
                    *r.choose(&mvs[0..n]).unwrap()
                };


                b.f_do_move(mv);
            }
        }
        eprintln!("");
        eprintln!("[COIN] Training on generated positions...");
        train_staged_heuristic_supervised(positions, 5, 0.0001, 32);
        let winp = measure_performance_vs_random(hs);
        eprintln!("[COIN] Random win P={}", winp);
    }
}

fn measure_performance_vs_random(hs: &mut StagedHeuristic) -> f32{
    let mut wins = 0;
    let mut mvs = empty_movelist();
    let mut r = rand::thread_rng();

    for g in 0..100 {

        let mut b = Board::new();

        let mut t = if g & 1 == 0 {1} else {-1};
        while !b.is_done() {

            let mut n = b.get_moves(&mut mvs) as usize;
            if n == 0 {
                mvs[0] = Move::pass();
                n = 1;
            }
            let mv = if t > 0 {
                
                let mut best_move = mvs[0];
                let mut best_value = {
                    let mut bc = b.copy();
                    bc.f_do_move(mvs[0]);
                    -negamax_opt(hs, bc, -1000, 1000, 4)
                };

                for &mv in &mvs[1..n] {
                    let mut bc = b.copy();
                    bc.f_do_move(mv);
                    let value = -negamax_opt(hs, bc, -1000, 1000, 4);

                    if value > best_value {
                        best_value = value;
                        best_move = mv;
                    }
                }

                best_move
            } else {
                *r.choose(&mvs[0..n]).unwrap()
            };


            b.f_do_move(mv);
            t = -t;
        }

        if t * b.piece_diff() > 0 {
            wins += 1;
        }
    }

    wins as f32 / 100.0
}

#[test]
fn logistello_book_training() {
    eprintln!("[COIN] Loading positions...");
    let mut games = if cfg!(target_os = "windows") {
        load_wthor_database(&Path::new("D:\\doc\\caltech\\othello\\databases\\Logistello_book_1999.wtb")).unwrap()
    } else {
        load_wthor_database(&Path::new("/mnt/d/doc/caltech/othello/databases/Logistello_book_1999.wtb")).unwrap()
    };

    let mut positions = vec![];
    for g in games {
        for s in g.states {
            positions.push(s);
        }
    }
    let mut hs = train_staged_heuristic_supervised(&mut positions, 5, 0.0001, 32);

    for i in 0..100 {
        train_staged_heuristic_reinforcement(&mut hs, &mut positions);

        let n = positions.len() * 2 / 3;
        rand::thread_rng().shuffle(&mut positions);
        positions.truncate(n);
    }

    let serialized = serde_json::to_string(&hs).unwrap();

    let mut file = File::create(&Path::new("../data/StagedHeuristic.json")).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}