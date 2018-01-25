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

extern crate tensorflow as tf;

extern crate bitboard;

pub mod eval;
pub mod mcts;

pub use eval::*;
pub use mcts::*;

#[cfg(test)]
mod tests {

    use rand::{self, Rng};
    use std::time::{Instant, Duration};
    use std::path::Path;

    use bitboard::{Board, Turn};
    use eval;
    use eval::Evaluator;

    #[test]
    fn restore_test() {

        let mut net = eval::CoinNet::new("./data/CoinNet_model.pb").unwrap();

        net.load(Path::new("./data/iter000/CoinNet-checkpoint.best.index")).unwrap();
    }

    #[test]
    fn tensorflow_throughput_test() {
        let mut net = eval::CoinNet::new("./data/CoinNet_model.pb").unwrap();

        // net.load(Path::new("./data/iter014/CoinNet-checkpoint.best.index")).unwrap();


        let mut rng = rand::thread_rng();

        let bs = [
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK),
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK)
        ];
        let res = net.evaluate_batch(&bs[..2]);

        let r0 = {
            let start = Instant::now();

            let res = net.evaluate_batch(&bs);

            let elapsed = start.elapsed();
            println!("TF Parallel thru-put: {:?}", elapsed);

            res
        };

        let r1 = {
            let start = Instant::now();

            let mut ret = vec![];
            for b in bs.iter() {
                ret.push(net.evaluate(&b));
            }

            let elapsed = start.elapsed();
            println!("TF Serial thru-put: {:?}", elapsed);

            ret
        };

        assert!(r0.iter().zip(r1.iter()).all(|(a,b)| a.1 == b.1 ));
    }
}