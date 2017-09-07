#![allow(unused_imports)]
#![allow(dead_code)]

use std::time::Instant;
use std::time::Duration;

use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use TranspositionTable;

use std::i32;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::AtomicIsize;

use std::borrow::Cow;
use rand;
use rand::Rng;

use rayon::prelude::*;

use SearchInfo;
use pvs;

/// This struct does a parallel minimax search using the Jamboree algorithm.
pub struct JamboreeSearchInfo {
    abort       : AtomicBool,
    /// Time out flag
    tm_out      : AtomicBool,
    /// Start time
    stime       : Instant,
    /// Number of nodes searched
    snodes      : AtomicUsize,
    /// The depth at which the abort occurred.
    abort_d     : AtomicUsize,
}

impl JamboreeSearchInfo {
    fn new () -> JamboreeSearchInfo {
        JamboreeSearchInfo {
            tm_out      : AtomicBool::new(false),
            stime       : Instant::now(),
            abort       : AtomicBool::new(false),
            abort_d     : AtomicUsize::new(0),
            snodes      : AtomicUsize::new(0),
        }
    }

    fn from_start(start : Instant) -> JamboreeSearchInfo {
        let mut tmp = JamboreeSearchInfo::new();
        tmp.stime = start;
        tmp
    }
}

const PAR_LEVELS : u8 = 3;
const PAR_CUTOFF : u8 = 10;

/// helper function to update atomic variables, may be highly unoptimal
fn atomic_set_if_greater(u : &AtomicUsize, n : usize) {
    loop {
        let ut = u.load(Ordering::SeqCst);
        if ut > n {
            if u.compare_and_swap(ut, n, Ordering::SeqCst) == n {
                break;
            }
        } else {
            break;
        }
    }
}

/// Jamboree search algorithm that falls back on PVS when searching serially becomes more efficient
pub fn jamboree<H: Heuristic>(info : & JamboreeSearchInfo, tt : &TranspositionTable, b : Board, h : &H, mut alpha : i32, mut beta : i32, d : u8, lvls : u8, msleft : u64) -> (Move, i32) {

    if lvls > PAR_LEVELS || d < PAR_CUTOFF {
        //if we reach the point at which it becomes less efficient to use the concurrency primitives, switch to PVS
        //this happens in a couple cases: when there are too many simultaneous options and when the remaining search 
        //tree is small enough
        let mut sinfo = SearchInfo::from_start(info.stime);
        let tmp = pvs(&mut sinfo, tt, h, b, alpha, beta, d, 0, msleft);
        info.tm_out.store(sinfo.to, Ordering::SeqCst);
        info.snodes.fetch_add(sinfo.sr as usize, Ordering::SeqCst);
        return tmp;
    }

    //check timeout status
    if info.tm_out.load(Ordering::SeqCst) {
        return (Move::null(), i32::MAX-3);
    }
    //check parent abort status and return early
    if info.abort.load(Ordering::SeqCst) && info.abort_d.load(Ordering::SeqCst) >= d as usize {
        return (Move::null(), i32::MAX-4);
    }

    {//transposition table code
        let (l,h) = tt.fetch(b);

        if l >= beta  { return (Move::null(), l); }
        alpha = if l > alpha {l} else {alpha};

        if h <= alpha { return (Move::null(), h); }
        beta = if h < beta {h} else {beta};
    }

    if b.is_done() {
        return (Move::null(), h.evaluate(b, Turn::BLACK));
    }

    let mut mvl = empty_movelist();
    let n = b.get_moves(&mut mvl) as usize;

    rand::thread_rng().shuffle(&mut mvl[0..n]);

    let mut g = {
        let mut cb = b.copy();
        cb.f_do_move(mvl[0]);
        -jamboree(info, tt, cb, h, -beta, -alpha, d-1, lvls+1, msleft).1
    };

    if g >= beta { return (mvl[0], g); }
    if g > alpha { alpha = g; }

    //let atom_alpha = AtomicIsize::new(alpha as isize);
    let atom_score = AtomicIsize::new(i32::MIN as isize);
    let atom_abort = AtomicBool::new(false);

    let mut bm = mvl[1..n]
        .par_iter()
        .map(|&m| {
            let mut cb = b.copy();
            cb.f_do_move(m);
            let s = -jamboree(info, tt, cb,  h, -alpha - 1, -alpha, d - 1, lvls+1, msleft).1;

            if info.tm_out.load(Ordering::SeqCst) {
                return (cb, 0, Move::null());
            }

            if s >= beta {
                info.abort.store(true, Ordering::SeqCst);
                atomic_set_if_greater(&info.abort_d, d as usize);
                atom_score.store(s as isize, Ordering::SeqCst);
                return (cb, s, m);
            }

            (cb, s, m)
        }).collect::<Vec<_>>();

    let mut best_score = i32::MIN;
    let mut best_move = Move::null();

    for (cb, mut s, m) in bm {
        // re-search if necessary
        if s > best_score {
            best_move = m;
            best_score = s;
        }
        if s > alpha {
            
            s = -jamboree(info, tt, cb, h, -beta, -alpha, d-1, lvls+1, msleft).1;

            if info.tm_out.load(Ordering::SeqCst) {
                return (Move::null(), 0);
            }

            if info.abort.load(Ordering::SeqCst) && info.abort_d.load(Ordering::SeqCst) >= d as usize {
                return (Move::null(), 0);
            }

            if s >= beta {
                best_move = m;
                best_score = s;
                break;
            }
            if alpha < s {
                alpha = s;
            }
            if s > best_score {
                best_move = m;
                best_score = s;
            }
        }
    }
    
    if info.tm_out.load(Ordering::SeqCst) {
        return (Move::null(), i32::MAX-5);
    }

    //check if we have aborted
    if info.abort.load(Ordering::SeqCst) {
        let abort_d = info.abort_d.load(Ordering::SeqCst) as u8;
        if abort_d > d {
            // if the search was aborted in a lower level, we just return, since our result will be thrown out
            return (Move::null(), 0);
        } else if abort_d == d {
            // if the search was aborted in this search level
            // This should never happen in the first level, so the return value should always have a move.
            best_score = atom_score.load(Ordering::SeqCst) as i32;
            best_move = Move::null();
            info.abort_d.store(0, Ordering::SeqCst);
            info.abort.store(false, Ordering::SeqCst);
        }
    }

    let mut low = i32::MIN;
    let mut high = i32::MAX;
    
    if best_score <= alpha                      { high = best_score; }
    if best_score > alpha && best_score < beta  { high = best_score; low = best_score; }
    if best_score >= beta                       { low = best_score; }
    
    tt.update(b, low, high);
    
    (best_move, best_score)
}

/// iterative deepening version of jamboree search.
pub fn jamboree_id<Hf: Heuristic, Hz: Heuristic>(bb : Board, hr : &[Box<Hf>], hz : &Hz, max_depth : u8, msleft : u64) -> Move {

    let start = Instant::now();

    let mut d = 5;
    if max_depth < d {d = max_depth};
    let mut best_move = Move::null();
    let empty = bb.total_empty();

    eprintln!("Starting ID PVS...");
    eprintln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");
    eprintln!("[COIN]: | {: <5} | {: <9} | {: <13} | {: <14} | {: <14} | {: <12} |",
            "Depth", "Best Move", "Minimax Value", "Transpositions", "Nodes Searched", "Time Elapsed");
    eprintln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");

    let tt = TranspositionTable::new(20_000_000);

    while d <= max_depth && d <= empty + 2 {

        let info = JamboreeSearchInfo::from_start(start);
        tt.clear();

        //select heuristic
        let hi = empty as i32 - d as i32;

        let (m, s) = if hi <= 0 {
            jamboree(&info, &tt, bb.copy(), hz, i32::MIN+1, i32::MAX-1, empty, 0, msleft)
            // jamboree(&info, &tt, bb.copy(), hz, 1, 2, empty, 0, msleft)
        } else {
            jamboree(&info, &tt, bb.copy(), &(*hr[(hi/3) as usize].clone()), i32::MIN+1, i32::MAX-1, d, 0, msleft)
            // jamboree(&info, &tt, bb.copy(), &(*hr[(hi/3) as usize].clone()), 1, 2, d, 0, msleft)
        };

        if info.tm_out.load(Ordering::SeqCst) {
            eprintln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");
            eprintln!("[COIN]: TIMEOUT");
            return best_move;
        }

        best_move = m;
        
        let elapsed = {
            let d = info.stime.elapsed();
            (d.as_secs() as f32) + (d.subsec_nanos() as f32)/1_000_000_000f32
        };
        
        let sr = info.snodes.load(Ordering::SeqCst);

        eprintln!("[COIN]: | {: >5} | {: >9} | {: >13} | {: >14} | {: >14} | {: >12.2} |",
                d, format!("{}",best_move), s, tt.size(), sr, elapsed);
        
        d += 2;
    }
    eprintln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");

    best_move
}