#![allow(unused_imports)]

use std::time::Instant;
use std::time::Duration;

use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use transposition::TranspositionTable;

use std::i32;
use std::sync::Arc;
use std::atomic::AtomicBool;

use Search;

/// This struct does a parallel minimax search using the Jamboree algorithm.
struct JamboreeSearch {
    /// Transposition table
    tt  : Box<TranspositionTable>,
    /// Search abort flag
    ab  : Arc<AtomicBool>,
    /// Start flag
    st  : Instant,
    /// heuristic
    hr  : Box<Heuristic>,
}

impl Search for JamboreeSearch {
    fn search(&mut self, board : Board, ms_left : u64) -> Move {
        //do some setup of state.
        self.st = Instant::now();
        self.ab = false;
        self.tt.clear();

        //call the search
        self.jamboree(board.copy(), i32::MIN+1, i32::MAX-1).1
    }

    fn set_heuristic(&mut self, hr : Box<Heuristic>) {
        self.hr = hr;
    }
}

impl JamboreeSearcher {
    fn jamboree(
        &mut self,
        b   : Board,
        α   : i32,
        β   : i32,

    ) -> (i32, Move) {

        if self.ab {
            return (Move::null, i32::MIN);
        }

        if b.is_done() {
            return (Move::null, self.hr.eval(b));
        }

        let mut mvl = empty_movelist();
        let n = b.get_moves(&mut mvl);

        let r = {
            let cb = b.copy();
            cb.do_move(mvl[0]);
            -self.jamboree(cb, -β, -α);
        };

        if r >= β { return (mvl[0], r); }
        if r >  α { α = r; }

        let bs = mvl[1..n].par_iter()
            .map(|m| {
                let mut cb = b.copy();
                cb.do_move(m);
                let s = -self.jamboree(cb, -α - 1, -α);

                if s > r {r = s};
                if s >= β {
                    self.ab = true;
                }
            }).collect< Vec<_> >();

        let ss = bs.par_iter()
            .map(|(m, cb)| {
                
            }).collect< Vec<_> >();

        ss.

    }

}