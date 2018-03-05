use serde::Serialize;
use serde::Deserialize;

use serde_json;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub enum Algorithm {
    MCTS,
    MTDF,
    PVS,
    BNS,
}

fn solve_depth() -> u8 {18}

#[derive(Serialize, Deserialize)]
pub struct CoinCfg {
    // mode : Algorithm,
    pub model_file : String,
    pub heuristic_directory : String,
    #[serde(default = "solve_depth")]
    pub solve_depth : u8,
}

impl CoinCfg {
    pub fn from_bytes(buf : &[u8]) -> CoinCfg {
        let config : CoinCfg = serde_json::from_slice(buf).unwrap();

        config
    }
    pub fn from_path(path : &Path) -> CoinCfg {
        let mut buf = vec![];

        File::open(path).unwrap().read_to_end(&mut buf).unwrap();

        let config : CoinCfg = serde_json::from_slice(&buf).unwrap();

        config
    }
}