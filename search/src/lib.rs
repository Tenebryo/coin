#![allow(unused_imports)]
extern crate bitboard;
extern crate heuristic;
extern crate rand;
extern crate rayon;

mod negamax_ab_timeout;
mod mtdf_id_timeout;
mod transposition;
mod search;
mod monte_carlo;
mod par_monte_carlo;
mod pvs;
mod jamboree;

pub use negamax_ab_timeout::NegamaxSearch;
pub use mtdf_id_timeout::{mtdf_timeout, mtdf_id_timeout};
pub use transposition::TranspositionTable;
pub use pvs::pvs;
pub use pvs::pvs_id;
pub use search::Search;
pub use search::SearchInfo;
pub use monte_carlo::MonteCarloSearch;
pub use par_monte_carlo::ParMonteCarloSearch;

pub use jamboree::jamboree_id;
pub use jamboree::JamboreeSearchInfo;

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
    use ::MonteCarloSearch;

    use std::time::Instant;

    use self::rand::Rng;

    #[test]
    fn pvs_test() {
        use heuristic::HandmadeHeuristic;
        use heuristic::ScaledBasicHeuristic;

        let tmp = Box::new(ScaledBasicHeuristic::new(10));
        let tmp_hr = [
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone()
        ];

        let b1 = Board::from_string(
            // Vec::from("BWWWWBBB\nBWBBWBBB\nBBBWWB B\nBWBBWB  \nBWBBWB  \nBWWWWBBB\nBWWWWBBB\nBBBBBBB \n")
            Vec::from(" WWWWWW \nWWWBWW W\nWWWWBWWW\nWWWWBWWW\nWWWWWWWW\nWWWWBWWW\n BBBWW W\n WWWWW  \n")
        );


        let out_move = pvs_id(b1, &tmp_hr, &ScaledBasicHeuristic::new(10_000), 9, 1000);

        println!("BEST MOVE: {} FOR \n{}", out_move, b1);
    }

    #[test]
    fn monte_carlo_test() {
        
        let mut r = rand::thread_rng();
        let mut mcts = MonteCarloSearch::new();

        let mut mvs = empty_movelist();
        let mut b = Board::new();

        for _ in 0..45 {
            let n = b.get_moves(&mut mvs);
            b.f_do_move(mvs[(r.gen::<u8>() % n) as usize]);
        }

        let res = mcts.search_for_millis(b, 10000);

        eprintln!("{}", b);
        eprintln!("{}", res);
    }

    #[test]
    fn par_monte_carlo_test() {
        use ::ParMonteCarloSearch;
        let mut r = rand::thread_rng();
        let mut mcts = ParMonteCarloSearch::new();

        let mut mvs = empty_movelist();
        let mut b = Board::new();

        for _ in 0..40 {
            let n = b.get_moves(&mut mvs);
            b.f_do_move(mvs[(r.gen::<u8>() % n) as usize]);
        }

        let res = mcts.search_for_millis(b, 10_000);

        eprintln!("{}", b);
        eprintln!("{}", res);
    }

    #[test]
    fn jamboree_test() {
        use ::jamboree_id;
        use heuristic::ScaledBasicHeuristic;
        use heuristic::WLDHeuristic;

        let tmp = Box::new(WLDHeuristic::new());
        let tmp_hr = [
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), 
            tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone(), tmp.clone()
        ];

        let mut r = rand::thread_rng();

        let mut mvs = empty_movelist();
        let mut b = Board::new();

        for _ in 0..40 {
            let n = b.get_moves(&mut mvs);
            b.f_do_move(mvs[(r.gen::<u8>() % n) as usize]);
        }

        eprintln!("{}", b);

        let res = jamboree_id(b, &tmp_hr, &WLDHeuristic::new(), 60, 15_000);

        eprintln!("{}", b);
        eprintln!("{}", res);
    }
}
