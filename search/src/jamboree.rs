#![allow(unused_imports)]
#![allow(dead_code)]

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
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::AtomicIsize;

use std::borrow::Cow;

use rayon::prelude::*;

use SearchInfo;
use pvs;

/// This struct does a parallel minimax search using the Jamboree algorithm.
pub struct JamboreeSearchInfo {
    abort       : AtomicBool,
    /// Time out flag
    tm_out      : AtomicBool,
    /// Transposition table
    ttable      : Mutex<TranspositionTable>,
    /// Start time
    stime       : Instant,
    /// Number of nodes searched
    snodes      : AtomicUsize,
}

impl JamboreeSearchInfo {
    fn new () -> JamboreeSearchInfo {
        JamboreeSearchInfo {
            tm_out      : AtomicBool::new(false),
            ttable      : Mutex::new(TranspositionTable::new(2_000_000)),
            stime       : Instant::now(),
            abort       : AtomicBool::new(false),
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

/// Jamboree search algorithm that falls back on PVS when searching serially becomes more efficient
pub fn jamboree<H: Heuristic>(info : & JamboreeSearchInfo, b : Board, h : &H, mut alpha : i32, mut beta : i32, d : u8, lvls : u8, msleft : u64) -> (Move, i32) {

    if lvls > PAR_LEVELS || d < PAR_CUTOFF {
        //if we reach the point at which it becomes less efficient to use the concurrency primitives, switch to PVS
        //this happens in a couple cases: when there are too many simultaneous options and when the remaining search 
        //tree is small enough
        let mut sinfo = SearchInfo::from_start(info.stime);
        let tmp = pvs(&mut sinfo, h, b, alpha, beta, d, 0, msleft);
        info.tm_out.store(sinfo.to, Ordering::SeqCst);
        info.snodes.fetch_add(sinfo.sr as usize, Ordering::SeqCst);
        return tmp;
    }

    //check parent abort status and return early
    if info.abort.load(Ordering::SeqCst) {
        return (Move::null(), i32::MIN + 2);
    }

    {//transposition table code
        let (l,h) = info.ttable.lock().unwrap().fetch(b);

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

    let mut g = {
        let mut cb = b.copy();
        cb.f_do_move(mvl[0]);
        -jamboree(info, cb, h, -beta, -alpha, d-1, lvls+1, msleft).1
    };

    if g >= beta { return (mvl[0], g); }
    if g > alpha { alpha = g; }

    let atom_alpha = AtomicIsize::new(alpha as isize);
    let atom_score = AtomicIsize::new(i32::MIN as isize);
    let atom_abort = AtomicBool::new(false);

    let mut bm = mvl[1..n]
        .par_iter()
        .map(|&m| {
            let mut cb = b.copy();
            cb.f_do_move(m);
            let mut s = -jamboree(info, cb,  h, -alpha - 1, -alpha, d - 1, lvls+1, msleft).1;

            if info.tm_out.load(Ordering::SeqCst) {
                return (i32::MIN+2, Move::null());
            }

            if s >= beta {
                info.abort.store(true, Ordering::SeqCst);
                atom_abort.store(true, Ordering::SeqCst);
                atom_score.store(s as isize, Ordering::SeqCst);
                return (s,m)
            }

            // re-search if necessary
            if s > alpha {
                
                s = -jamboree(info, cb, h, -beta, atom_alpha.load(Ordering::SeqCst) as i32, d-1, lvls+1, msleft).1;

                if info.tm_out.load(Ordering::SeqCst) {
                    return (i32::MIN+2, Move::null());
                }

                if info.abort.load(Ordering::SeqCst) {
                    return (s,m);
                }

                if s >= beta {
                    info.abort.store(true, Ordering::SeqCst);
                    atom_abort.store(true, Ordering::SeqCst);
                    atom_score.store(s as isize, Ordering::SeqCst);
                    return (s,m);
                }
                // make sure alpha is updated atomically
                let mut a = atom_alpha.load(Ordering::SeqCst);
                loop {
                    if s > a as i32 {
                        let ta = atom_alpha.compare_and_swap(a, s as isize, Ordering::SeqCst);
                        if ta == a {
                            break;
                        }
                        a = ta;
                    } else {
                        break;
                    }
                }
            }
            
            (s, m)
        })
        .collect::<Vec<_>>()
        .iter()
        .fold( (i32::MIN+2, Move::null()), |acc, &val| {
            // find max element of returned items
            if val.0 > acc.0 {
                val
            } else {
                acc
            }
        }).1;

    
    if info.tm_out.load(Ordering::SeqCst) {
        return (Move::null(), i32::MAX-2);
    }

    //check if we have aborted
    if atom_abort.load(Ordering::SeqCst) {
        // if the search was aborted in this search level
        // This should never happen in the first level, so the return value should always have a move.
        g = atom_score.load(Ordering::SeqCst) as i32;
        bm = Move::null();
        info.abort.store(false, Ordering::SeqCst);
    } else if atom_abort.load(Ordering::SeqCst) {
        // if the search was aborted in a different search level
        return (Move::null(), i32::MAX-2);
    }

    let mut low = i32::MIN;
    let mut high = i32::MAX;
    
    if g <= alpha               { high = g; }
    if g > alpha && g < beta    { high = g; low = g; }
    if g >= beta                { low = g; }
    
    {
        info.ttable.lock().unwrap().update(b, low, high);
    }
    
    (bm, g)
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

    while d <= max_depth && d <= empty + 2 {

        let info = JamboreeSearchInfo::from_start(start);

        //select heuristic
        let hi = empty as i32 - d as i32;

        let (m, s) = if hi <= 0 {
            jamboree(&info, bb.copy(), hz, i32::MIN+1, i32::MAX-1, empty, 0, msleft)
        } else {
            jamboree(&info, bb.copy(), &(*hr[(hi/3) as usize].clone()), i32::MIN+1, i32::MAX-1, d, 0, msleft)
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
                d, format!("{}",best_move), s, 0, sr, elapsed);
        
        d += 2;
    }
    eprintln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");

    best_move
}