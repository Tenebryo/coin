//STL imports
use std::io::prelude::*;
use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;
use std::fs::create_dir;
use std::time::Instant;
use std::time::Duration;

//Crate imports
use bincode::{serialize, deserialize, Infinite};
use serde_json;

//Coin imports
use pattern_engine::PatternSet;
use heuristic::ScaledBasicHeuristic;
use heuristic::PatternHeuristic;

//Training imports
use train_pattern::train_pattern;
use generate_positions::generate_positions;
use TrainingPosition;
use time_func;

pub fn autotrain_20(
    dir         : &Path, 
    data_set    : usize, 
    cost_bound  : f32, 
    epoch_cap   : usize, 
    masks       : &Vec<u64>
) -> [PatternSet; 20] {

    if create_dir(dir).is_err() {
        println!("Directory '{:?}' already exists.", dir);
    }

    let mut pss = [
        PatternSet::new(), PatternSet::new(), PatternSet::new(), PatternSet::new(),
        PatternSet::new(), PatternSet::new(), PatternSet::new(), PatternSet::new(),
        PatternSet::new(), PatternSet::new(), PatternSet::new(), PatternSet::new(),
        PatternSet::new(), PatternSet::new(), PatternSet::new(), PatternSet::new(),
        PatternSet::new(), PatternSet::new(), PatternSet::new(), PatternSet::new()
    ];

    for pi in 0..20 {

        let empty = 3*pi + 3;

        println!("Creating Pattern for range [{:02}, {:02}]", empty-2, empty);

        let mut training_set : Vec<TrainingPosition> = vec![];

        //generate positions...
        let ts_path = dir.join(Path::new(&format!("tset_e{:02}-{:02}.ts", empty-2, empty)));
        match File::open(&ts_path) {
            //the training set already exists, so use this one 
            //(pick up where we left off)
            Ok(mut f) => {
                println!("Loading previously generated positions...");
                let mut fbuf : Vec<u8> = vec![];
                f.read_to_end(&mut fbuf)
                    .unwrap_or_else(|_| panic!(format!("LINE: {}",line!())));

                training_set = deserialize(&fbuf[..])
                    .unwrap_or_else(|_| panic!(format!("LINE: {}",line!())));
            },
            //the training set didn't alread exist
            Err(r) => {
                println!("Generating new positions...");
                if pi == 0 {
                    generate_positions(5, data_set/5, &mut training_set, 
                                    &mut ScaledBasicHeuristic::new(10),
                                    empty, 3);
                } else {
                    let mut ph = PatternHeuristic::from_pattern_set(Box::new(pss[pi as usize -1].clone()));
                    generate_positions(5, data_set/5, &mut training_set, &mut ph,
                                    empty, 3);
                }

                //write file to disk
                let serial = serialize(&training_set, Infinite)
                    .unwrap_or_else(|_| panic!(format!("LINE: {}",line!())));
                File::create(ts_path).unwrap().write_all(&serial[..])
                    .unwrap_or_else(|_| panic!(format!("LINE: {}",line!())));
            }
        }

        pss[pi as usize] = {
            //train on positions...
            let ps_path = dir.join(Path::new(&format!("pdesc_e{:02}-{:02}.json", empty-2, empty)));
            match File::open(&ps_path) {
                //the pattern file already exists, so use this one 
                //(pick up where we left off)
                Ok(mut f) => {
                    println!("Loading previously trained pattern...");
                    let mut fbuf : Vec<u8> = vec![];
                    f.read_to_end(&mut fbuf).unwrap();

                    let ps : PatternSet = serde_json::from_slice(&fbuf[..])
                        .unwrap_or_else(|e| panic!(format!("LINE: {} => {}",line!(), e)));
                    ps
                },
                //the training set didn't alread exist
                Err(r) => {
                    println!("Training new pattern...");

                    let mut ps = PatternSet::from_masks(&masks[..]);

                    let mut epochs = 0;
                    while epochs < epoch_cap && 
                        train_pattern(&mut ps, 5, epochs, 100, &training_set, 
                                    0.0005) > cost_bound {

                        epochs += 5;
                    }

                    //write file to disk
                    let serial = serde_json::to_vec(&ps).unwrap();
                    File::create(ps_path).unwrap().write_all(&serial[..])
                        .unwrap_or_else(|_| panic!(format!("LINE: {}",line!())));
                    ps
                }
            }
            //trained new pattern;
        }
    }

    pss
}