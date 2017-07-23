extern crate bitboard;
extern crate heuristic;
extern crate rand;

mod negamax_ab_timeout;
mod mtdf_id_timeout;
mod transposition;
mod search;
//mod monte_carlo;
mod pvs;

pub use negamax_ab_timeout::NegamaxSearch;
pub use mtdf_id_timeout::{mtdf_timeout, mtdf_id_timeout};
pub use transposition::TranspositionTable;
pub use pvs::pvs;
pub use pvs::pvs_id;
pub use search::Search;
pub use search::SearchInfo;

#[cfg(test)]
mod tests {
    extern crate rand;
    use heuristic::BasicHeuristic;
    use bitboard::Board;
    use bitboard::Move;
    use bitboard::empty_movelist;

    use ::mtdf_id_timeout;
    use ::TranspositionTable;
    use std::time::Instant;

    use self::rand::Rng;
    #[test]
    fn it_works() {
    }

    #[test]
    fn negamax_mtdf_consistency() {
        let mut b = Board::new();

        let m = mtdf_id_timeout(b, Box::new(BasicHeuristic::new()), 34, 10000);

        println!("Move: {}", m);
    }

    #[test]
    fn monte_carlo_test() {

    }
}
