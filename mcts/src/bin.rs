#![feature(test)]
#![feature(swap_with_slice)]

#![allow(dead_code)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate clap;

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

use clap::{App, Arg, SubCommand};

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

    let matches = App::new("mcts_train")
        .version("0.2")
        .about("Automated reinforcement learning for the COIN Othello bot.")
        .author("Sam Blazes")
        .arg(Arg::with_name("data")
            .short("d")
            .long("data")
            .value_name("FOLDER")
            .help("Set the folder to use for data.")
            .required(true))
        .arg(Arg::with_name("model")
            .short("m")
            .long("model")
            .value_name("MODEL")
            .required(true)
            .help("Set the model file to use, relative to the data folder"))
        .get_matches();

    //unwrap is safe because the arguments are required.
    let data_folder = Path::new(matches.value_of("data").unwrap());
    let model_file = Path::new(matches.value_of("model").unwrap());

    let mut trainer = MctsTrainer::new(3, &data_folder, &data_folder.join(model_file), None);

    let n = trainer.load_files(&data_folder)?;

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
