#![feature(swap_with_slice)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bincode;

extern crate rand;
extern crate rayon;

extern crate tensorflow as tf;

extern crate bitboard;

mod eval;
mod mcts;
mod train;

use eval::*;
use mcts::*;
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

    let coinnet = CoinNet::new("./data/CoinNet_model.pb")?;

    let mut trainer = MctsTrainer::<CoinNet>::new(6, coinnet);

    let mut i = 0;
    loop {
        println!("[COIN] Starting training iteration {}", i);
        let eta = match i {
              0 ... 400 => 0.0001,
            400 ... 600 => 0.00005,
            _           => 0.000025,
        };
        trainer.iteration(i, eta);

        i += 1;
    }

    Ok(())
}