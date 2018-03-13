use bitboard::*;
use std::time::*;
use std::i32;
use solver::ttable::*;
use std::cmp;

const OPT_CUTOFF : usize = 6;

#[derive(Clone)]
pub struct MoveOrderInfo {
    pub killers     : [[u32; 64]; 60],
    pub history     : [u32; 64],
    pub butterfly   : [u32; 64],
    pub kmoves      : [(usize, [usize; 4]); 64],
    pub ttable      : TTable,
    pub ttable_hits : usize,
    pub leaf_nodes  : usize,
}

impl MoveOrderInfo {
    pub fn new() -> MoveOrderInfo {
        MoveOrderInfo {
            killers : [[0; 64];60],
            history : [0; 64],
            butterfly : [1; 64],
            kmoves : [(0, [0;4]); 64],
            ttable : TTable::new(1_000_000),
            ttable_hits : 0,
            leaf_nodes : 0,
        }
    }
}

pub fn quick_board_score(
    b           : &Board,
    o           : usize, 
    e           : usize, 
    info        : &mut MoveOrderInfo
) -> i32 {
    let mut score = 0;
    const CORNERS : u64 = 0x81_00_00_00_00_00_00_81;

    const MOBILITY_SCORE : i32 = 31;
    const FRONTIER_SCORE : i32 = 1;
    const HISTORY_SCORE : i32 = 9;
    const CORNER_SCORE : i32 = 16;
    const KILLER_SCORE : i32 = 4;
    const PPIECES_SCORE : i32 = 4;

    let (om,_) = b.mobility();
    let ps = b.pieces().1;
    let (pf, of) = b.calculate_exposed();

    let mut mvs = empty_movelist();
    let n = b.get_moves(&mut mvs) as usize;
    let mut min_mob = 64i32;
    for i in 0..n {
        let mut bc = b.copy();
        bc.f_do_move(mvs[i]);

        let (pm,_) = bc.mobility();
        let pm = pm.count_ones() as i32;
        if pm < min_mob {
            min_mob = pm;
        }
    }

    if min_mob == 0 {
        min_mob = -10;
    }

    score -= MOBILITY_SCORE * (om.count_ones() as i32);
    score += MOBILITY_SCORE * (min_mob as i32);
    score -= CORNER_SCORE   * ((om & CORNERS).count_ones() as i32);
    score += CORNER_SCORE   * ((ps & CORNERS).count_ones() as i32);
    // score += HISTORY_SCORE  * (info.history[o] as i32);
    // score += FRONTIER_SCORE * (pf.count_ones() as i32);
    // score -= FRONTIER_SCORE * (of.count_ones() as i32);
    // score -= PPIECES_SCORE  * (ps.count_ones() as i32);
    // score += KILLER_SCORE   * (info.killers[e][o] as i32) << 6;
    // score += ((info.history[o] as i32) << 5) /  info.butterfly[o] as i32;
    // score += if info.kmoves[e].1[0] == o 
    //     || info.kmoves[e].1[1] == o 
    //     || info.kmoves[e].1[2] == o 
    //     || info.kmoves[e].1[3] == o {1 << 8} 
    //     else {0};
    
    score
}

pub fn order_moves_extras(
    b : Board,
    idx : &mut [usize],
    mvs : &[Move],
    info        : &mut MoveOrderInfo,
    extra : &[i32; 64]
) {
    use std::mem;
    const CORNERS : u64 = 0x81_00_00_00_00_00_00_81;
    
    let mut scores : [i32; 64] = unsafe{mem::uninitialized()};
   
    let empty = b.total_empty() as usize;
    for &m in mvs {
        let mut bc = b.copy();
        bc.f_do_move(m);
        
        let o = m.offset() as usize;
        
        scores[o] = (extra[o] << 6) + quick_board_score(&bc, o, empty, info);

    }
    
    idx.sort_unstable_by_key(|&i| -scores[mvs[i].offset() as usize]);
}

pub fn order_moves(
    b : Board, 
    mvs : &mut [Move],
    test_move : Move,
    info        : &mut MoveOrderInfo
) {
    use std::mem;
        
    
    let mut scores : [i32; 64] = unsafe{mem::uninitialized()};
   
    let empty = b.total_empty() as usize;
    for &m in mvs.iter() {
        let mut bc = b.copy();
        bc.f_do_move(m);
        
        let o = m.offset() as usize;
        
        scores[o] = quick_board_score(&bc, o, empty, info);
    }

    if !test_move.is_null() && !test_move.is_pass() {
        scores[test_move.offset() as usize] += 4096;
    }
    
    mvs.sort_unstable_by_key(|m| -scores[m.offset() as usize]);
}

//simple negamax implementation.
pub fn negamax_ordering (
    mut bb      : Board,
    mut alpha   : i32,
    mut beta        : i32,
    info        : &mut MoveOrderInfo,
    start       : Instant,
    ms_left     : Duration,
    timeout     : &mut bool
) -> i32 {
    let oa = alpha;

    if bb.is_done() {
        info.leaf_nodes += 1;
        return (bb.piece_diff() as i32).signum();
    }

    let tt = info.ttable.check(bb.pieces());
    let test_move = match tt {
        Some(tte) => {
            info.ttable_hits += 1;
            let value = match tte.bound {
                TBound::Exact(bound) => {return bound;},
                TBound::Lower(bound) => {alpha = cmp::max(alpha, bound); alpha},
                TBound::Upper(bound) => {beta = cmp::min(beta, bound); beta},
            };
            if alpha >= beta { return value; }
            tte.best_move
        },
        _ => Move::null(),
    };

    let mut rmvs : MoveList = empty_movelist();

    let empty = bb.total_empty() as usize;

    let n = bb.get_moves(&mut rmvs) as usize;
    
    if n == 0 {
        bb.f_do_move(Move::pass());
        return - if empty <= OPT_CUTOFF {
            negamax_opt(bb, -beta, -alpha, &mut info.leaf_nodes)
        } else {
            negamax_ordering(bb, -beta, -alpha, info, start, ms_left, timeout)
        };
    }

    order_moves(bb, &mut rmvs[0..n], test_move, info);

    //negamax step
    let mut g = i32::MIN;
    let mut bm = Move::null();

    //loop through all the moves
    for i in 0..n {
        let mut bc = bb.copy();
        let m = rmvs[i as usize];
        bc.f_do_move(m);

        //recurse, updating alpha and beta appropriately.
        let v = - if empty <= OPT_CUTOFF {
            negamax_opt(bc, -beta, -alpha, &mut info.leaf_nodes)
        } else {
            negamax_ordering(bc, -beta, -alpha, info, start, ms_left, timeout)
        };
        
        if *timeout || start.elapsed() >= ms_left {
            *timeout = true;
            return 1064;
        }
        
        //update best move
        if g < v { g = v; bm = m; }

        if alpha < g { alpha = g; }

        let o = m.offset() as usize;
        info.butterfly[o] += 1;
        if alpha >= beta { 
            info.killers[empty][o] += 1;
            info.history[o] += 1;
            info.kmoves[empty].1[info.kmoves[o].0] = o;
            info.kmoves[empty].0 = (info.kmoves[empty].0 + 1) & 0b11;
            break; 
        }
    }

    let new_entry = if g < oa {
        TEntry::upper(g, bm)
    } else if g > beta {
        TEntry::lower(g, bm)
    } else {
        TEntry::exact(g, bm)
    };

    info.ttable.store(bb.pieces(), new_entry);

    g
}


//simple negamax implementation.
pub fn negamax_opt (
    mut bb      : Board,
    mut alpha   : i32,
    beta        : i32,
    nodes       : &mut usize
) -> i32 {

    if bb.is_done() {
        *nodes += 1;
        return (bb.piece_diff() as i32).signum();
    }
    
    //eprintln!("{:?} {:?}", bb.pieces(), bb.mobility());

    let mut rmvs : MoveList = empty_movelist();

    let n = bb.get_moves(&mut rmvs);

    //negamax step
    let mut g = i32::MIN;

    //loop through all the moves
    for i in 0..n {
        let mut bc = bb.copy();
        let m = rmvs[i as usize];
        bc.f_do_move(m);

        //recurse, updating alpha and beta appropriately.
        let v = -negamax_opt(bc, -beta, -alpha, nodes);

        //update best move
        if g < v { g = v; }

        if alpha < g { alpha = g; }

        if alpha >= beta { break; }
    }

    g
}