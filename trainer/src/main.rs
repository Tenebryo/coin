extern crate bitboard;
extern crate heuristic;
extern crate pattern_engine;
extern crate search;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate bincode;
#[macro_use]
extern crate clap;

// mod generate_positions;
// mod train_pattern;
// mod autotrain20;
mod wthor;
mod supervised;

// use train_pattern::train_pattern;
// use generate_positions::generate_positions;
// use autotrain20::autotrain_20;

use bitboard::Board;
use bitboard::Move;
use heuristic::BasicHeuristic;
use pattern_engine::PatternSet;

use bincode::{serialize, deserialize, Infinite};
use clap::{Arg,App};

use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use std::time::Duration;

#[derive(Serialize, Deserialize, PartialEq)]
pub struct TrainingPosition {
    ps : u64,
    os : u64,
    sc : i16,
    mv : Move,
}

impl TrainingPosition {
    fn new(b : Board, mv : Move, sc : i16) -> TrainingPosition {
        let (ps, os) = b.pieces();

        TrainingPosition {
            ps, os, sc, mv
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TrainingSet(Vec<TrainingPosition>);

type TrainingDataType = (Board, i16, Move);

///currently set to generate 10,000,000 possitions
const POSITIONS_PER_EPOCH : usize = 2_000_000;
const NUM_THREADS : usize = 5;

fn time_func<F : FnMut()> (mut func : F) -> u64 {
    let st = Instant::now();

    func();

    let e = st.elapsed();
    1_000_000_000 * e.as_secs() + e.subsec_nanos() as u64
}

fn main() {

    let masks = vec![
        //diag4
        //0b00010000_00100000_01000000_10000000_00000000_00000000_00000000_00000000,
        1161999622361579520,
        //diag5
        //0b00001000_00010000_00100000_01000000_10000000_00000000_00000000_00000000,
        580999813328273408,
        //diag6
        //0b00000100_00001000_00010000_00100000_01000000_10000000_00000000_00000000,
        290499906672525312,
        //diag7
        //0b00000010_00000100_00001000_00010000_00100000_01000000_10000000_00000000,
        145249953336295424,
        //diag8
        //0b00000001_00000010_00000100_00001000_00010000_00100000_01000000_10000000,
        72624976668147840,

        //ordinate2
        //0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
        71776119061217280,
        //ordinate3
        //0b00000000_00000000_11111111_00000000_00000000_00000000_00000000_00000000,
        280375465082880,
        //ordinate4
        //0b00000000_00000000_00000000_11111111_00000000_00000000_00000000_00000000,
        1095216660480,

        //edge+2
        //0b11111111_01000010_00000000_00000000_00000000_00000000_00000000_00000000,
        18393263828134526976,
        //2x5 corner
        //0b11111000_11111000_00000000_00000000_00000000_00000000_00000000_00000000,
        17940089115630370816,
        //3x3 corner
        //0b11100000_11100000_11100000_00000000_00000000_00000000_00000000_00000000,
        16204197749883666432,
        //wide half diag
        //0b11000000_11100000_01110000_01100000_00000000_00000000_00000000_00000000,
        13898232007684521984,
    ];

    let matches = clap_app!(coin_trainer =>
            (version: "0.1.0")
            (author: "Sam Blazes <blazes.sam (at) gmail.com>")
            (about: "Generates training data and trains pattern heuristics on it.")
            (@arg POSITIONS: -p --positions +takes_value "Sets the number of training positions to generate per pattern set (Default 1000).")
            (@arg MAX_EPOCHS: -e --max-epochs +takes_value "Sets the maximum number of epochs to train a pattern for (Default 100).")
            (@arg LEARNING_RATE: -l --learning-rate +takes_value "Sets the learning rate (Default 0.001).")
            (@arg COST_CUTOFF: -c --cost-cutoff +takes_value "Sets the point for the average training cost below which training stops (Default 400.0).")
            (@arg OUTPUT_FOLDER: +required "The location pattern files and training data sets are written to (40 files will be written here, so an empty or non-existent directory is ideal).")
        ).get_matches();

    use std::usize;
    use std::f32;

    let output = matches.value_of("OUTPUT_FOLDER").unwrap_or_else(|| panic!("Must provide an output directory!"));
    let output = Path::new(&output);

    let positions = matches.value_of("POSITIONS").unwrap_or("1000");
    let positions = positions.parse::<usize>().unwrap_or_else(|_| panic!("Positions must be a positive integer!"));

    let max_epochs = matches.value_of("MAX_EPOCHS").unwrap_or("100");
    let max_epochs = max_epochs.parse::<usize>().unwrap_or_else(|_| panic!("Epoch cap must be a positive integer!"));

    let cost_cutoff = matches.value_of("COST_CUTOFF").unwrap_or("400.0");
    let cost_cutoff = cost_cutoff.parse::<f32>().unwrap_or_else(|_| panic!("Cost cutoff must be a positive number!"));

    let learning_rate = matches.value_of("LEARNING_RATE").unwrap_or("0.001");
    let learning_rate = learning_rate.parse::<f32>().unwrap_or_else(|_| panic!("Learning rate must be a positive number!"));

    // autotrain_20(&output, positions, cost_cutoff, max_epochs, learning_rate, &masks);
}
