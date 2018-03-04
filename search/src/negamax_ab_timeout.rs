use std::time::Instant;
use std::time::Duration;

use rand;
use rand::Rng;
use rand::ThreadRng;

use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;
use bitboard::MAX_MOVES;

use transposition::TranspositionTable;

use std::i32;

use Search;

pub struct NegamaxSearch<H: Heuristic> {
    tt  : TranspositionTable,
    hr  : Box<H>,
    to  : bool,
pub st  : Instant,
    dc  : u8,
    sr  : u64, 
    rn  : ThreadRng,
}

impl<H : Heuristic> Search<H> for NegamaxSearch<H> {
    fn search(&mut self, board : Board, ms_left : u64, start : Instant) -> Move {
        unimplemented!();
    }

    fn set_heuristic(&mut self, hr : Box<H>) {
        self.hr = hr;
    }
}

impl<H: Heuristic> NegamaxSearch<H> {
    pub fn new(hr : Box<H>, start : Instant) -> NegamaxSearch<H> {
        NegamaxSearch {
            tt  : TranspositionTable::new(10_000_000),
            hr  : hr,
            to  : false,
            st  : start,
            dc  : 3,
            sr  : 0,
            rn  : rand::thread_rng(),
        }
    }

    /// Clears the transposition table.
    pub fn clear_transpositions(&mut self) {
        self.tt.clear();
    }

    /// Gets the number of transpositions stored in the transposition table.
    pub fn get_transpositions(&self) -> u64 {
        self.tt.size() as u64
    }

    /// Gets the number of nodes searched
    pub fn get_searched(&self) -> u64 {
        self.sr
    }

    /// Returns a bool of whether the last call timed out.
    pub fn timeout(&self) -> bool {
        self.to
    }

    ///Negamax, fixed-depth, alpha-beta, with timeout
    pub fn negamax (
        &mut self,
        bb          : Board,
        mut alpha   : i32,
        mut beta    : i32,
        d           : u8,
        ms_left     : u64,
        out_move    : &mut Move
    ) -> i32 {
        self.sr += 1;

        {
            let (l,h) = self.tt.fetch(bb);

            if l >= beta  { return l; }
            alpha = if l > alpha {l} else {alpha};

            if h <= alpha { return h; }
            beta = if h < beta {h} else {beta};
        }

        if bb.is_done() || d == 0 {
            *out_move = Move::pass();
            return self.hr.evaluate(bb.copy(), Turn::BLACK);
        }

        let mut rmvs : MoveList = [Move::null(); MAX_MOVES];
        let mut omvs : MoveOrder = [(0i32, 0); MAX_MOVES];
        

        let n = bb.get_moves(&mut rmvs);
        
        //shuffle move list for better characteristics:
        self.rn.shuffle(&mut rmvs[0..(n as usize)]);

        //negamax step
        let mut g = i32::MIN;

        //loop through all the moves
        for i in 0..n {
            let mut bc = bb.copy();

            let m = rmvs[i as usize];
            bc.f_do_move(m);

            //recurse, updating alpha and beta appropriately.
            let v = if d > self.dc {
                //continue checking timeout
                -self.negamax(bc, -beta, -alpha, d-1, ms_left, &mut Move::null())
            } else {
                //switch to cheaper negamax to avoid checking timout often
                -self.negamax_opt( bc, -beta, -alpha, d-1 )
            };

            //break out if we have used all of our time
            if self.to || self.st.elapsed() >= Duration::from_millis(ms_left) {
                self.to = true;
                return 0;
            }

            //update best move
            if g < v {
                g = v;
                *out_move = m; //rmvs[omvs[i as usize].1];
            }

            if alpha < g {
                alpha = g;
            }

            if alpha >= beta {
                return g;
                //break;
            }
        }

        let mut low = i32::MIN;
        let mut high = i32::MAX;
        
        if g <= alpha               { high = g; }
        if g > alpha && g < beta    { high = g; low = g; }
        if g >= beta                { low = g; }
        
        self.tt.update(bb, low, high);
        
        g
    }

    ///Negamax, fixed-depth, alpha-beta, without timeout for lower depth searches
    ///also can be used to ignore transposition table when it is slower than just
    ///searching everything
    pub fn negamax_opt (
        &mut self,
        bb          : Board,
        mut alpha   : i32,
        mut beta    : i32,
        d           : u8,
    ) -> i32 {

        // increment the number of nodes searched
        self.sr += 1;

        if bb.is_done() || d == 0 {
            return self.hr.evaluate(bb.copy(), Turn::BLACK);
        }

        let mut rmvs : MoveList = [Move::null(); MAX_MOVES];
        
        let n = bb.get_moves(&mut rmvs);

        //negamax step
        let mut g = i32::MIN;

        //loop through all the moves
        for i in 0..n {
            let mut bc = bb.copy();
            let m = rmvs[i as usize];
            bc.f_do_move(m);

            //recurse, updating alpha and beta appropriately.
            let v = -self.negamax_opt( bc, -beta, -alpha, d-1 );

            //update best move
            if g < v { g = v; }

            if alpha < g { alpha = g; }

            if alpha >= beta { break; }
        }

        //transposition table code is left out because this version of negamax
        //is optimized for searching leaf nodes and branches near leaf nodes
        
        g
    }
}

// negamax ID code:
// cerrln!("[COIN]: Starting AlphaBeta Search:");
// cerrln!("[COIN]: | {: <5} | {: <9} | {: <13} | {: <14} | {: <14} | {: <12} |",
//         "Depth", "Best Move", "Minimax Value", "Transpositions", "Nodes Searched", "Time Elapsed");
// cerrln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");
// let mut out_move = Move::null();
// let mut d = 7;

// while d < 60 {
//     let empty = (empty as i32 - d as i32)/3;
//     let empty = if empty < 0 {0} else {empty};

//     let mut mv = Move::null();

//     let (s, tr, sr, to) = if empty == 0 {
//         let mut ng = NegamaxSearch::new(Box::new(ScaledBasicHeuristic::new(10)), start);

//         use std::i32;
//         let s = ng.negamax_ab_timeout(b.copy(), i32::MIN+1, i32::MAX-1, d as u8, 
//                                 alloc_time, &mut mv);

//         (s, ng.get_transpositions(), ng.get_searched(), ng.timeout())
//     } else {
//         let mut ng = NegamaxSearch::new(self.phs[empty as usize].clone(), start);

//         use std::i32;
//         let s = ng.negamax_ab_timeout(b.copy(), i32::MIN+1, i32::MAX-1, d as u8, 
//                                 alloc_time, &mut mv);

//         (s, ng.get_transpositions(), ng.get_searched(), ng.timeout())
//     };

//     if to {
//         cerrln!("[COIN]: TIMEOUT");
//         break;
//     }


//     cerrln!("[COIN]: | {: >5} | {: >9} | {: >13} | {: >14} | {: >14} | {: >12} |",
//             d, format!("{}",mv), s, tr, sr, 0);

//     out_move = mv;

//     d += 2;
// }