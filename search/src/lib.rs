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
    use ::pvs_id;
    use ::TranspositionTable;
    use std::time::Instant;

    use self::rand::Rng;
    #[test]
    fn it_works() {
    }

    #[test]
    fn negamax_mtdf_consistency() {
        let mut b = Board::new();

        //let m = mtdf_id_timeout(b, Box::new(BasicHeuristic::new()), 34, 10000);

        //println!("Move: {}", m);
    }

    #[test]
    fn monte_carlo_test() {

    }

    #[test]
    fn pvs_test() {
        use heuristic::HandmadeHeuristic;
        use heuristic::ScaledBasicHeuristic;

        let tmp = Box::new(ScaledBasicHeuristic::new(10));
        let mut tmp_hr = [
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone()
        ];

        let b1 = Board::from_string(
            // Vec::from("BWWWWBBB\nBWBBWBBB\nBBBWWB B\nBWBBWB  \nBWBBWB  \nBWWWWBBB\nBWWWWBBB\nBBBBBBB \n")
            Vec::from(" WWWWWW \nWWWBWW W\nWWWWBWWW\nWWWWBWWW\nWWWWWWWW\nWWWWBWWW\n BBBWW W\n WWWWW  \n")
        );


        let mut out_move = pvs_id(b1, &mut tmp_hr, &mut ScaledBasicHeuristic::new(10_000), 9, 1000);

        println!("BEST MOVE: {} FOR \n{}", out_move, b1);
    }
}
