use rand;
use rand::Rng;

use std::time::Instant;
use std::i32;
use std::fs::File;
use std::io::prelude::*;

use heuristic::*;

use bson;
use serde_json;

use PatternSet;
use bitboard::*;

const NUM_STAGES : usize = 15;
const EMPTY_LEVEL : usize = 60/NUM_STAGES;
const SEARCH_DEPTH : u8 = 4;
const SGD_BATCH_SIZE : usize = 32;
const SGD_NUM_BATCHES : usize = 64_000; //250_000;
const POSITION_PER_LEVEL : usize = SGD_BATCH_SIZE * SGD_NUM_BATCHES;
const TRAINING_EPOCHS : usize = 64;
const TRAINING_ETA : f32 = 0.025 / SGD_BATCH_SIZE as f32;

const PATTERN_MASKS : [u64; 12] = [
    0b00010000_00100000_01000000_10000000_00000000_00000000_00000000_00000000,
    0b00001000_00010000_00100000_01000000_10000000_00000000_00000000_00000000,
    0b00000100_00001000_00010000_00100000_01000000_10000000_00000000_00000000,
    0b00000010_00000100_00001000_00010000_00100000_01000000_10000000_00000000,
    0b00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000,
    0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
    0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000,
    0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000,
    0b11111111_01000010_00000000_00000000_00000000_00000000_00000000_00000000,
    0b11111000_11111000_00000000_00000000_00000000_00000000_00000000_00000000,
    0b11100000_11100000_11100000_00000000_00000000_00000000_00000000_00000000,
    0b11100000_11100000_11100000_00000000_00000000_00000000_00000000_00000000
];

#[derive(Serialize, Deserialize, Clone)]
pub struct StagedHeuristic {
    heuristics  : Vec<[PatternSet; 2]>
}

impl StagedHeuristic {
    pub fn new() -> StagedHeuristic {
        let mut hs : Vec<[PatternSet;2]> = Vec::with_capacity(NUM_STAGES);
        
        for _ in 0..NUM_STAGES {
            hs.push([
                PatternSet::from_masks(&PATTERN_MASKS),
                PatternSet::from_masks(&PATTERN_MASKS)
            ]);
        }

        StagedHeuristic{
            heuristics : hs,
        }
    }

    pub fn eval(&self, mut b : Board) -> i32 {
        let empty = b.total_empty() as usize;

        if empty == 0 {
            use std::i32;
            return (b.piece_diff() as i32).signum();
        } else if empty == 1 {
            let mut mvs = empty_movelist();
            b.get_moves(&mut mvs);
            b.f_do_move(mvs[0]);
            
            return (b.piece_diff() as i32).signum();
        }

        let (ps, os) = b.pieces();

        //eval_raw, and then scale and round to an int.
        (self.heuristics[empty/NUM_STAGES][empty & 1].eval_raw(ps, os).0 * 128.0).floor() as i32
    }

    pub fn eval_raw(&self, b : Board) -> (f32, [(u64,u64);8]) {
        let empty = b.total_empty() as usize;

        let (ps, os) = b.pieces();

        self.heuristics[empty/NUM_STAGES][empty & 1].eval_raw(ps, os)
    }

    pub fn adjust_fn<F>(&mut self, b : Board, syms : [(u64,u64);8], grad_fn : &mut F) 
        where F: FnMut(f32) -> f32 
    {
        let empty = b.total_empty() as usize;

        self.heuristics[empty/NUM_STAGES][empty & 1].adjust_fn(syms, grad_fn)
    }

    pub fn train(&mut self) {

        let mut r = rand::thread_rng();

        for level in 0..NUM_STAGES {
            eprintln!("[COIN] Training Heuristic Level {:3}", level);
            let mut pos = [vec![],vec![]];
            // generate a bunch of positions for each level, and estimate/
            // calculate the score

            eprintln!("[COIN]   Generating Positions...");
            let pos_start = Instant::now();

            for _ in 0..POSITION_PER_LEVEL {
                for k in 0..EMPTY_LEVEL {
                    let e = EMPTY_LEVEL*level + k;
                    pos[e & 1].push(self.random_position(e as u8));
                }
            }

            eprintln!("[COIN]     Time = {:?} ({} /s)", pos_start.elapsed(),
                    (pos[0].len() + pos[1].len()) as f32/pos_start.elapsed().as_secs() as f32);

            eprintln!("[COIN]   Training...");

            let sgd_start = Instant::now();

            for e in 0..TRAINING_EPOCHS {
                //shuffle data
                r.shuffle(&mut pos[0]);
                r.shuffle(&mut pos[1]);

                let mut even_err = 0.0;
                let mut odd_err = 0.0;

                let epoch_start = Instant::now();
                let mut eta = TRAINING_ETA;

                for b in (0..(pos[0].len())).step_by(SGD_BATCH_SIZE) {
                    let mut batch_data = Vec::with_capacity(SGD_BATCH_SIZE);
                    for i in 0..SGD_BATCH_SIZE {
                        let (b, est) = pos[0][b+i];
                        let (ps, os) = b.pieces();
                        let (score, syms) = self.heuristics[level][0].eval_raw(ps,os);

                        let diff = score - est;
                        even_err += diff * diff;
                        batch_data.push((diff, syms));
                    }

                    for (grad, syms) in batch_data {
                        self.heuristics[level][0].adjust(syms, eta * grad);
                    }

                    let mut batch_data = Vec::with_capacity(SGD_BATCH_SIZE);

                    for i in 0..SGD_BATCH_SIZE {
                        let (b, est) = pos[1][b+i];
                        let (ps, os) = b.pieces();
                        let (score, syms) = self.heuristics[level][1].eval_raw(ps,os);

                        let diff = score - est;
                        odd_err += diff * diff;
                        batch_data.push((diff, syms));
                    }

                    for (grad, syms) in batch_data {
                        self.heuristics[level][1].adjust(syms, eta * grad);
                    }

                    eta *= 0.975;
                }

                eprintln!("[COIN]     Epoch {:3}: Even Err = {:.4}   Odd Err = {:.4}", 
                        e, even_err/pos[0].len() as f32, odd_err/pos[1].len() as f32);
                eprintln!("[COIN]     Epoch Time: {:?}", epoch_start.elapsed());
            }

            eprintln!("[COIN]     Training Time: {:?}", sgd_start.elapsed());
        }
    }

    //generates a random position and evaluates it using negamax.
    fn random_position(&self, empty : u8) -> (Board, f32) {
        let mut b = Board::new();
        
        let mut mvs = empty_movelist();
        let mut r = rand::thread_rng();

        while empty < b.total_empty() {
            if b.is_done() {
                b = Board::new()
            }

            let n = b.get_moves(&mut mvs) as usize;

            if n == 0 {
                b.f_do_move(Move::pass());
            } else {
                let mv = *r.choose(&mvs[0..n]).unwrap();
                b.f_do_move(mv);
            }
        }

        assert!(b.total_empty() == empty);

        let score = self.negamax_opt(b, i32::MIN, i32::MAX, SEARCH_DEPTH);

        (b, score as f32)
    }

    //simple negamax implementation.
    fn negamax_opt (
        &self,
        bb          : Board,
        mut alpha   : i32,
        beta        : i32,
        d           : u8,
    ) -> i32 {

        if bb.is_done() {
            return (bb.piece_diff() as i32);
        } else if d == 0 {
            return self.eval(bb.copy()) as i32;
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
            let v = -self.negamax_opt( bc, -beta, -alpha, d-1 );

            //update best move
            if g < v { g = v; }

            if alpha < g { alpha = g; }

            if alpha >= beta { break; }
        }

        g
    }
}

impl Heuristic for StagedHeuristic {

    fn evaluate(&self, b : Board, t : Turn) -> i32 {
        self.eval(b) as i32
    }
    
    fn order_moves(&self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {

    }
}

#[test]
fn train_staged_heuristic() {
    eprintln!("[COIN] Creating Heuristic Set...");
    let mut h = StagedHeuristic::new();

    eprintln!("[COIN] Training Heuristic Set...");
    h.train();

    eprintln!("[COIN] Saving Heuristic Set...");
    let bin = serde_json::to_string(&h).unwrap();

    let mut file = File::create("heuristic.json").unwrap();
    file.write_all(bin.as_bytes()).unwrap();
}