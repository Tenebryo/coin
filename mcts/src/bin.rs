#![feature(test)]
#![feature(swap_with_slice)]

#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bincode;

extern crate rand;
extern crate rayon;

extern crate threadpool;
extern crate scoped_threadpool;

extern crate tensorflow as tf;
extern crate indexmap;

extern crate bitboard;

mod eval;
mod mcts;
mod train;
mod game;
mod solver;

use train::*;

use std::error::Error;
use std::result::Result;

fn main() {
    std::process::exit(match run(){
        Ok(_) => {0},
        Err(e) => {
            println!("{:?}", e);

            1
        },
    })
}

fn run() -> Result<(), Box<Error>> {
    use std::path::Path;

    let mut trainer = MctsTrainer::new(3, &Path::new("./data/CoinNet_model.pb"), None);

    let n = trainer.load_files(&Path::new("./data"))?;

    eprintln!("[COIN] Seeding players with logistello opening book...");

    // trainer.seed_players(n, &Path::new("D:\\doc\\caltech\\othello\\databases\\"))?;

    let mut i = n+1;
    loop {
        println!("[COIN] Starting training iteration {}", i);
        let eta = match i {
              0 ... 400 => 0.01,
            400 ... 600 => 0.005,
            _           => 0.0025,
        };
        trainer.iteration(i, eta);

        i += 1;
    }

    Ok(())
}
