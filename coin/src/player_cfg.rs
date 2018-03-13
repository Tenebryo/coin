use serde::Serialize;
use serde::Deserialize;

use serde_json;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::*;

pub enum Algorithm {
    MCTS,
    MTDF,
    PVS,
    BNS,
}

fn mcts_rounds() -> isize{-1}
fn solve_depth() -> u8 {21}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinCfg {
    // mode : Algorithm,
    pub model_file : String,
    pub heuristic_directory : String,
    #[serde(default = "solve_depth")]
    pub solve_depth : u8,
    #[serde(default = "mcts_rounds")]
    pub mcts_rounds : isize,
}

impl CoinCfg {
    pub fn from_bytes(buf : &[u8]) -> CoinCfg {
        let config : CoinCfg = serde_json::from_slice(buf).unwrap();

        config
    }
    pub fn from_path(path : &Path) -> CoinCfg {
        let mut buf = vec![];

        while let Err(_) = File::open(path).map(|mut fd| fd.read_to_end(&mut buf)) {
            thread::sleep(Duration::from_millis(100));
        };

        let config : CoinCfg = serde_json::from_slice(&buf).unwrap();

        config
    }
}