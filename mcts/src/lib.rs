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

pub mod eval;
pub mod mcts;

pub use eval::*;
pub use mcts::*;

#[cfg(test)]
mod tests {

    #[test]
    fn restore_test() {
        use eval;
        use eval::Evaluator;
        use std::path::Path;

        let mut net = eval::CoinNet::new("./data/CoinNet_model.pb").unwrap();

        net.load(Path::new("./data/iter5/CoinNet-checkpoint.best.5")).unwrap();
    }
}