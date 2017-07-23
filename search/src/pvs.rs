use SearchInfo;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use heuristic::Heuristic;

use std::i32;
use std::time::Instant;

use rand::Rng;

const DEPTH : u8 = 3;

pub fn pvs<H: Heuristic>(info : &mut SearchInfo, hr: &mut H, bb : Board, mut alpha : i32, mut beta : i32, depth : u8, msleft : u64) -> (Move, i32) {
    if depth <= DEPTH {
        info.check_timeout(msleft);
        if info.to {
            return (Move::null(), 0);
        } else {
            return (Move::null(), pvs_no_tt(info, hr, bb, alpha, beta, depth));
        }
    }

    info.sr += 1;

    {//transposition table code
        let (l,h) = info.tt.fetch(bb);

        if l >= beta  { return (Move::null(), l); }
        alpha = if l > alpha {l} else {alpha};

        if h <= alpha { return (Move::null(), h); }
        beta = if h < beta {h} else {beta};
    }

    if bb.is_done() {
        return (Move::null(), hr.evaluate(bb, Turn::BLACK));
    } else if depth == 0 {
        return (Move::null(), hr.evaluate(bb, Turn::BLACK));
    }

    let mut mvs = empty_movelist();
    let n = bb.get_moves(&mut mvs) as usize;

    //order moves (random for now TODO: make history heuristic, etc)
    info.rn.shuffle(&mut mvs[0..n]);

    //search
    let mut g = i32::MIN;
    let mut m = Move::null();
    {
        let mut bc = bb.copy();
        bc.f_do_move(mvs[0]);
        let score = -pvs(info, hr, bc, -beta, -alpha, depth-1, msleft).1;

        if info.to {
            return (Move::null(), 0);
        }

        g = if g < score {m = mvs[0]; score} else {g};
        alpha = if alpha < score {score} else {alpha};
    }

    for i in 1..n {
        let mut bc = bb.copy();
        bc.f_do_move(mvs[i]);

        let mut score = -pvs(info, hr, bc, -alpha-1, -alpha, depth-1, msleft).1;
        if alpha < score && score < beta {
            score = -pvs(info, hr, bc, -beta, -alpha, depth-1, msleft).1;
        }

        g = if g < score {m = mvs[i]; score} else {g};
        alpha = if alpha < score {score} else {alpha};
        if alpha >= beta {break;}
    }

    let mut low = i32::MIN;
    let mut high = i32::MAX;
    
    if g <= alpha               { high = g; }
    if g > alpha && g < beta    { high = g; low = g; }
    if g >= beta                { low = g; }
    
    info.tt.update(bb, low, high);
    
    (m, g)
}

fn pvs_no_tt<H: Heuristic>(info : &mut SearchInfo, hr: &mut H, bb : Board, mut alpha : i32, beta : i32, depth : u8) -> i32 {
    
    info.sr += 1;

    if bb.is_done() {
        return hr.evaluate(bb, Turn::BLACK);
    } else if depth == 0 {
        return hr.evaluate(bb, Turn::BLACK);
    }

    let mut mvs = empty_movelist();
    let n = bb.get_moves(&mut mvs) as usize;

    //order moves (random for now TODO: make history heuristic, etc)
    info.rn.shuffle(&mut mvs[0..n]);

    //search
    let mut g = i32::MIN;
    let mut m = Move::null();
    {
        let mut bc = bb.copy();
        bc.f_do_move(mvs[0]);
        let score = -pvs_no_tt(info, hr, bc, -beta, -alpha, depth-1);

        g = if g < score {score} else {g};
        alpha = if alpha < score {score} else {alpha};
    }

    for i in 1..n {
        let mut bc = bb.copy();
        bc.f_do_move(mvs[i]);

        let mut score = -pvs_no_tt(info, hr, bc, -alpha-1, -alpha, depth-1);
        if alpha < score && score < beta {
            score = -pvs_no_tt(info, hr, bc, -beta, -alpha, depth-1);
        }

        g = if g < score {score} else {g};
        alpha = if alpha < score {score} else {alpha};
        if alpha >= beta {break;}
    }

    g
}

pub fn pvs_id<H: Heuristic>(bb : Board, hr : &mut [H], hz : &mut H, max_depth : u8, msleft : u64) -> Move {
    use search::SearchInfo;

    let mut d = 5;
    let mut info = SearchInfo::new();
    let mut best_move = Move::null();
    let empty = bb.get_empty();

    eprintln!("Starting ID PVS...");
    eprintln!("[COIN]: | {: <5} | {: <9} | {: <13} | {: <14} | {: <14} | {: <12} |",
            "Depth", "Best Move", "Minimax Value", "Transpositions", "Nodes Searched", "Time Elapsed");
    eprintln!("[COIN]: |{0:-<7}|{0:-<11}|{0:-<15}|{0:-<16}|{0:-<16}|{0:-<14}|", "");
    while d < max_depth && d <= empty + 2 {

        info.tt.clear();

        //select heuristic
        let hi = empty as i32 - d as i32;

        let (m, s) = if hi <= 0 {
            pvs(&mut info, hz, bb.copy(), i32::MIN+1, i32::MAX-1, d, msleft);
        } else {
            pvs(&mut info, hr[(i/3) as usize], bb.copy(), i32::MIN+1, i32::MAX-1, d, msleft);
        }

        if info.to {
            return best_move;
        }

        best_move = m;
        
        let elapsed = {
            let d = info.st.elapsed();
            (d.as_secs() as f32) + (d.subsec_nanos() as f32)/1_000_000_000f32
        };
        
        eprintln!("[COIN]: | {: >5} | {: >9} | {: >13} | {: >14} | {: >14} | {: >12.2} |",
                d, format!("{}",best_move), s, info.tt.size(), info.sr, elapsed);
        
        d += 2;
    }

    best_move
}