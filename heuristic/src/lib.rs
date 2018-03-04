extern crate bitboard;
// extern crate pattern_engine;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
//extern crate tensorflow;
extern crate rand;

mod heuristic;

pub use heuristic::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
