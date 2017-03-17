use std::time::Instant;
use std::time::Duration;
use std::cmp;
use std::collections::HashMap;

use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;
use bitboard::MAX_MOVES;
use std::i32;

use heuristic;

#[macro_use]
use common;

pub trait Search {
    /// Search for good move starting at the given position for the given player
    fn search(&mut self, bb : Board, max_ms : u32, t : Turn) -> Move;
}


struct TTEntry {
    lower   : i32,
    upper   : i32,
}

impl TTEntry {
    fn new(lower : i32, upper : i32) -> TTEntry {
        TTEntry {
            lower   : lower,
            upper   : upper,
        }
    }
}

pub struct SearchEngine {
    killers     : [[Move; 8]; 30],
    trns        : HashMap<Board, TTEntry>,
    side        : Turn,
}

fn min(a : i32, b : i32) -> i32 {if a < b {a} else {b}}
fn max(a : i32, b : i32) -> i32 {if a > b {a} else {b}}

impl SearchEngine {

    /// Returns a new SearchEngine object
    pub fn new(t : Turn) -> SearchEngine {
        SearchEngine {
            killers     : [[Move::null(); 8];30],
            trns        : HashMap::new(),
            side        : t,
        }
    }

    /// Uses alpha-beta search with a time limit, which will force the search to
    /// stop after a certain amount of time.
    pub fn alpha_beta_with_timeout<H: Heuristic>(
        &mut self,
        bb          : Board, 
        h           : &mut H,
        t           : Turn, 
        mut alpha   : i32, 
        mut beta    : i32,
        d           : u8,
        ms_left     : u64,
        start       : Instant,
        to_flag     : &mut bool
    ) -> i32 {
        //transposition table code
        /* TODO: ADD memory management
        {
            let mut tt = self.trns.entry(bb.copy()).or_insert(TTEntry::new(i32::MIN, i32::MAX));
            if tt.lower >= beta { return tt.lower; }
            if tt.upper <= alpha { return tt.upper; }
            alpha = max(alpha, tt.lower);
            beta = min(beta, tt.upper);
        }
        // */
    
        //check if we've reached the max depth/the game has ended
        if bb.is_done() || d == 0 {
            return h.evaluate(bb, t);
        }
        
        let mut rmvs : MoveList = [Move::null(); MAX_MOVES];
        let mut omvs : MoveOrder = [(0i32, 0); MAX_MOVES];
        
        let n = bb.get_moves(t, &mut rmvs);
        
        for i in 0..n {
            omvs[i as usize] = (0, i as usize);
        }
        
        for i in 0..8 {
            if self.killers[d as usize][i].is_null() {
                break;
            }
            if !self.killers[d as usize][i].is_pass() && bb.check_move(t, self.killers[d as usize][i]) {
                let ind = bb.get_move_index(t, self.killers[d as usize][i]);
                //TODO: perhaps make this more intelligent, currently just
                //adds a score to the move pre-order score
                omvs[ind].0 += 32;
            }
        }
        
        let r = match t {
            Turn::BLACK => {
                //pre-order moves to allow more agressive pruning
                omvs[0..(n as usize)].sort_by(|a,b| b.partial_cmp(a).unwrap_or(cmp::Ordering::Equal));
                
                let mut g = i32::MIN;
                let mut a = alpha;
                
                for i in 0..n {
                    let mut bc = bb.copy();
                    bc.do_move(t, rmvs[omvs[i as usize].1]);
                    
                    g = max(
                        g, self.alpha_beta_with_timeout(bc, h, !t, a, beta, d-1, ms_left, start, to_flag)
                    );
                    
                    a = max(a,g);
                    
                    //prune branch
                    if a >= beta {
                        break;
                    }
                    
                    //break out if we have used all of our time
                    if *to_flag || start.elapsed() >= Duration::from_millis(ms_left) {
                        *to_flag = true;
                        return 0;
                    }
                }
                g
            },
            Turn::WHITE => {
                //pre-order moves to allow more agressive pruning
                omvs[0..(n as usize)].sort_by(|a,b| a.partial_cmp(b).unwrap_or(cmp::Ordering::Equal));
                
                let mut g = i32::MAX;
                let mut b = beta;
                
                for i in 0..n {
                    let mut bc = bb.copy();
                    bc.do_move(t, rmvs[omvs[i as usize].1]);
                    
                    g = min(
                        g, self.alpha_beta_with_timeout(bc, h, !t, alpha, b, d-1, ms_left, start, to_flag)
                    );
                    
                    b = min(b,g);
                    
                    //prune branch
                    if alpha >= b {
                        break;
                    }
                    
                    //break out if we have used all of our time
                    if *to_flag || start.elapsed() >= Duration::from_millis(ms_left) {
                        *to_flag = true;
                        return 0;
                    }
                }
                g
            }
        };
        
        /*TODO: add memory management
        
        let mut tt = self.trns.entry(bb.copy()).or_insert(TTEntry::new(i32::MIN, i32::MAX));
        
        //update Transposition table
        if r <= alpha {tt.upper = r;}
        if r > alpha && r < beta {tt.upper = r; tt.lower = r;}
        if r >= beta {tt.lower = r;}
        // */
        r
    }
    
    
    pub fn alpha_beta_id<H: Heuristic>(
        &mut self,
        bb      : Board, 
        h       : &mut H,
        t       : Turn, 
        alpha   : i32, 
        beta    : i32,
        ms_left : u64
    ) -> Move {
    
        let mut d = 5;
        let mut bmove : usize = 0;
        let mut bscore: i32 = match t {
            Turn::BLACK => alpha,
            Turn::WHITE => beta,
        };
        
        let mut mvs : MoveList = [Move::pass(); MAX_MOVES];
        let n = bb.get_moves(t, &mut mvs);
        
        
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(ms_left-500) {
            let mut a = alpha;
            let mut b = beta;
            let mut best = 0;
            
            match t {
                Turn::BLACK => {
                    let mut g = a;
                    for i in 0..(n as usize) {
                        let mut bc = bb.copy();
                        bc.do_move(t, mvs[i]);
                        
                        let mut to = false;
                        
                        let v = self.alpha_beta_with_timeout(
                            bc, h, !t, a, b, d, ms_left, start, &mut to
                        );
                        
                        if to || start.elapsed() >= Duration::from_millis(ms_left-500) {
                            //break out before we update g, so we don't use half-
                            //searched score
                            break;
                        }
                        
                        if v > g {
                            g = v;
                            best = i;
                        }
                        
                        a = max(a,g);
                        
                        if a >= b {
                            break;
                        }
                    }
                    
                    if g > bscore {
                        bmove = best;
                        bscore = g;
                    }
                },
                Turn::WHITE => {
                    let mut g = a;
                    
                    for i in 0..(n as usize) {
                        let mut bc = bb.copy();
                        bc.do_move(t, mvs[i]);
                        
                        let mut to = false;
                        let v = self.alpha_beta_with_timeout(
                            bc, h, !t, a, b, d, ms_left, start, &mut to
                        );
                        
                        if to || start.elapsed() >= Duration::from_millis(ms_left-500) {
                            //break out before we update g, so we don't use half-
                            //searched score
                            break;
                        }
                        
                        if v < g {
                            g = v;
                            best = i;
                        }
                        
                        b = min(b,g);
                        
                        if a >= b {
                            break;
                        }
                    }
                    
                    if g < bscore {
                        bmove = best;
                        bscore = g;
                    }
                }
            }
            
            d += 2;
        }
        
        return mvs[bmove];
    }
    
    ///Implements the MTD(f) algorithm
    pub fn mtdf_with_timeout<H : Heuristic>(
        &mut self, 
        bb      : Board, 
        h       : &mut H,
        t       : Turn, 
        mut g   : i32, 
        d       : u8,
        ms_left : u64,
        start   : Instant,
        to_flag : &mut bool
    ) -> (Move, i32) {
    
        let mut best = 0;
        let mut low = i32::MIN;
        let mut high = i32::MAX;
        let mut mvs : MoveList = [Move::null(); MAX_MOVES];
        
        let n = bb.get_moves(t, &mut mvs);
        
        while low < high {
            
            let mut old = g;
            //do an alpha-beta search here
            let mut alpha = g;
            let mut beta = g;
            
            if g == low {
                beta += 1;
                old += 1;
            } else {
                alpha -= 1;
            }
            
            let mut flag = false;
            let mut mx = match t {
                Turn::BLACK => i32::MIN,
                Turn::WHITE => i32::MAX,
            };
            
            for i in 0..(n as usize) {
                let mut bc = bb.copy();
                bc.do_move(t, mvs[i]);
                
                let v = self.alpha_beta_with_timeout(
                    bc, h, !t, alpha, beta, d, ms_left, start, to_flag
                );
                
                if start.elapsed() >= Duration::from_millis(ms_left) {
                    //return with timeout flag set if time runs out, as MTD(f)
                    //Results are not meaningful if not complete
                    *to_flag = true;
                    return (Move::null(),0);
                }
                
                //update bounds and best moves
                match t {
                    Turn::BLACK => {
                        if v > mx {
                            mx = v;
                            best = i;
                            flag = true;
                        }
                        
                        alpha = max(alpha, mx);
                    },
                    Turn::WHITE => {
                        if v < mx {
                            mx = v;
                            best = i;
                            flag = true;
                        }
                        
                        beta = min(beta, mx);
                    }
                }
            }
            //end alpha-beta search, get result. The result must not be a timeout
            g = mx;
            
            //update MTD(f) bounds
            if g < old {
                high = g;
            } else {
                low = g;
            }
            
        
        }
        
        (mvs[best], g)
    }
    
    
    ///Iterative deepening implementation of MTD(f).
    pub fn mtdf_id<H : Heuristic>(
        &mut self,
        bb      : Board,
        h       : &mut H,
        t       : Turn,
        md      : u8,
        ms_left : u64
    ) -> Move {
    
        //check if I have any moves
        if !bb.has_move(t) {
            cerrln!("[COIN]: Passing.");
            return Move::pass();
        }
    
        cerrln!("[COIN]: Starting MTD(f) search for {}!", t);
        let start = Instant::now();
        
        let mut mv = Move::null();
        let mut f = 0;
        let mut d = 5;
        while d <= md && start.elapsed() < Duration::from_millis(ms_left) {
            
            self.trns.clear();
            
            let mut to = false;
            let (m, v) = self.mtdf_with_timeout(bb.copy(), h, t, f, d, ms_left, start, &mut to);
            
            if to {
                cerrln!("[COIN]: TIMEOUT");
                break;
            }
            f = v;
            mv = m;
            
            cerrln!("[COIN]: Depth {}. Best move found: {} Estimated score: {}", d, mv, f);
            
            //This allows us to search deeper as well as compare scores better
            d += 2;
        }
        
        let d = start.elapsed();
        cerrln!(
            "[COIN]: Done searching ({} s).", 
            (d.as_secs() as f32) + (d.subsec_nanos() as f32)/1_000_000_000f32
        );
        
        return mv;
    }

}






















