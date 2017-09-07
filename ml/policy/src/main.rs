extern crate tensorflow;
extern crate bitboard;
extern crate rand;

use std::error::Error;
use std::result::Result;

//use rand::Rng;

use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use tensorflow::Code;
use tensorflow::Graph;
use tensorflow::ImportGraphDefOptions;
use tensorflow::Session;
use tensorflow::SessionOptions;
use tensorflow::Status;
use tensorflow::StepWithGraph;
use tensorflow::Tensor;

pub mod policy;
pub mod simulate;

fn main() {
    std::process::exit(match _main() {
        Ok(_) => {
            0
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            1
        }
    })
}

fn _main() -> Result<(), Box<Error>> {
    use bitboard::{Board, Move};
    let fname = "../../data/policy/policy-6x3.pb";

    let mut po = policy::Policy::new(&fname)?;

    let mut eta = 0.005;

    for e in 0..20 {

        eprintln!("Simulating!");

        //play 10000 games per test, for 600000 total positions
        let s = simulate::play_game_set(&mut po, 10_000)?;
        
        eprintln!("Training!");

        let cost = po.train(s, eta)?;

        eprintln!("Epoch #{} complete. Average cost: {}", e, cost);

        eta *= 0.97;
    }


    Ok(())
}

#[test]
fn perf_policy() {
    _perf_policy().unwrap();
}
fn _perf_policy() -> Result<(), Box<Error>> {
    use bitboard::Board;
    use std::time::Instant;

    let fname = "../../data/policy/policy-8x3.pb";
    let mut po = policy::Policy::new(&fname)?;

    let iters = 1000;
    let start = Instant::now();

    let mut tmp = [0.0; 64];
    for _ in 0..iters {
        po.eval(Board::new(), &mut tmp);
    }

    let tm = start.elapsed();
    eprintln!("Time for {} iters: {:?}", iters, tm);
    Ok(())
}