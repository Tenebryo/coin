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

use transposition::TranspositionTable;

use std::i32;

use heuristic;

#[macro_use]
use common;

pub trait Search {
    /// Search for good move starting at the given position for the given player
    fn search(&mut self, bb : Board, max_ms : u32, t : Turn) -> Move;
}

pub struct SearchEngine {
    killers     : [[Move; 8]; 60],
    trns        : TranspositionTable,
    side        : Turn,
    history     : [[[i32;8];8];2],
}

#[inline]
fn min(a : i32, b : i32) -> i32 {if a < b {a} else {b}}
#[inline]
fn max(a : i32, b : i32) -> i32 {if a > b {a} else {b}}
#[inline]
fn choose(t : Turn, a : i32, b : i32) -> i32 {
    match t {
        Turn::BLACK => max(a,b),
        Turn::WHITE => min(a,b),
    }
}

impl SearchEngine {

    /// Returns a new SearchEngine object
    pub fn new(t : Turn) -> SearchEngine {
    
        const a1 : i32 = 256;
        const b1 : i32 = -32;
        const c1 : i32 = 16;
        const d1 : i32 = 8;
        const b2 : i32 = -32;
        const c2 : i32 = 4;
        const d2 : i32 = -4;
        const c3 : i32 = 8;
        const d3 : i32 = 2;
        const d4 : i32 = 1;

        SearchEngine {
            killers     : [[Move::null(); 8];60],
            trns        : TranspositionTable::new(1_000_000),
            side        : t,
            history     : [[
                [a1, b1, c1, d1, d1, c1, b1, a1],
                [b1, b2, c2, d2, d2, c2, b2, b1],
                [c1, c2, c3, d3, d3, c3, c2, c1],
                [d1, d2, d3, d4, d4, d3, d2, d1],
                [d1, d2, d3, d4, d4, d3, d2, d1],
                [c1, c2, c3, d3, d3, c3, c2, c1],
                [b1, b2, c2, d2, d2, c2, b2, b1],
                [a1, b1, c1, d1, d1, c1, b1, a1]
            ];2],
        }
    }
    
    
    //re-initializes the history heuristic
    pub fn reinit_history(&mut self) {
    
        const a1 : i32 = 256;
        const b1 : i32 = -32;
        const c1 : i32 = 16;
        const d1 : i32 = 8;
        const b2 : i32 = -32;
        const c2 : i32 = 4;
        const d2 : i32 = -4;
        const c3 : i32 = 8;
        const d3 : i32 = 2;
        const d4 : i32 = 1;
        
        self.history = [[
            [a1, b1, c1, d1, d1, c1, b1, a1],
            [b1, b2, c2, d2, d2, c2, b2, b1],
            [c1, c2, c3, d3, d3, c3, c2, c1],
            [d1, d2, d3, d4, d4, d3, d2, d1],
            [d1, d2, d3, d4, d4, d3, d2, d1],
            [c1, c2, c3, d3, d3, c3, c2, c1],
            [b1, b2, c2, d2, d2, c2, b2, b1],
            [a1, b1, c1, d1, d1, c1, b1, a1]
        ];2];
    }
    
    
    ////////////////////////////////////////////////////////////////////////////
    /// Uses alpha-beta search with a time limit, which will force the search to
    /// stop after a certain amount of time.
    ///
    /// * `bb` The root position to search from
    /// * `h` The heuristic to use for searches
    /// * `t` The current turn
    /// * `alpha` The lower bound of the minimax score
    /// * `beta` The upper bound of the minimax score
    /// * `d` The maximum depth to search to
    /// * `ms_left` The number of milliseconds allocated to make the move
    /// * `start` The time the search was started
    /// * `to_flag` An out-parameter to signal when a timeout occured
    ////////////////////////////////////////////////////////////////////////////
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
        to_flag     : &mut bool,
        out_move    : &mut Move
    ) -> i32 {
        //transposition table code
        // * TODO: ADD memory management
        {
            let bounds = self.trns.fetch(bb);
            
            match bounds.0 {
                Some(l) => {
                    if l >= beta  { return l; }
                    alpha = max(alpha, l);
                },
                None => {}
            }

            match bounds.1 {
                Some(h) => {
                    if h <= alpha { return h; }
                    beta = min(beta, h);
                },
                None => {}
            }
        }
        // */
    
        //check if we've reached the max depth/the game has ended
        if bb.is_done() || d == 0 {
            *out_move = Move::pass();
            return h.evaluate(bb, t);
        }
        
        
        //fetch the available moves from the board
        let mut rmvs : MoveList = [Move::null(); MAX_MOVES];
        let mut omvs : MoveOrder = [(0i32, 0); MAX_MOVES];
        
        let n = bb.get_moves(t, &mut rmvs);
        
        //initialize history heuristic move ordering
        const history_cell_weight : i32 = 2;
        const history_cutoff_weight : i32 = 1;
        let history_index = match t {Turn::BLACK => 0, Turn::WHITE => 1};
        for i in 0..(n as usize) {
            omvs[i] = (
                history_cell_weight * self.history[history_index][rmvs[i].x() as usize][rmvs[i].y() as usize], 
                i
            );
        }
        
        //check for killer moves, and modify move ordering as appropriate 
        //(possibly deprecated)
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
            //Simulate BLACK's turn
            Turn::BLACK => {
                //pre-order moves to allow more agressive pruning
                omvs[0..(n as usize)].sort_by(|a,b| b.partial_cmp(a).unwrap_or(cmp::Ordering::Equal));
                
                let mut g = i32::MIN;
                let mut a = alpha;
                
                for i in 0..n {
                    let mut bc = bb.copy();
                    bc.do_move(t, rmvs[omvs[i as usize].1]);
                    
                    let v = self.alpha_beta_with_timeout(bc, h, !t, a, beta, d-1, ms_left, start, to_flag, &mut Move::null());
                    
                    //break out if we have used all of our time
                    if *to_flag || start.elapsed() >= Duration::from_millis(ms_left) {
                        *to_flag = true;
                        return 0;
                    }
                    
                    //update best move
                    if v > g {
                        g = v;
                        //cerrln!("g: {}", g);
                        *out_move = rmvs[omvs[i as usize].1];
                    }
                    
                    //update alpha value
                    a = max(a,g);
                    
                    //prune branch
                    if a >= beta {
                        //move caused a beta cutoff (Thus inc history moves)
                        let m = rmvs[omvs[i as usize].1];
                        self.history[0][m.x() as usize][m.y() as usize] += history_cutoff_weight;
                        break;
                    }
                }
                g
            },
            //Simulate WHITE's turn
            Turn::WHITE => {
                //pre-order moves to allow more agressive pruning
                omvs[0..(n as usize)].sort_by(|a,b| a.partial_cmp(b).unwrap_or(cmp::Ordering::Equal));
                
                let mut g = i32::MAX;
                let mut b = beta;
                
                for i in 0..n {
                    let mut bc = bb.copy();
                    bc.do_move(t, rmvs[omvs[i as usize].1]);
                    
                    let v = self.alpha_beta_with_timeout(bc, h, !t, alpha, b, d-1, ms_left, start, to_flag, &mut Move::null());
                    
                    //break out if we have used all of our time
                    if *to_flag || start.elapsed() >= Duration::from_millis(ms_left) {
                        *to_flag = true;
                        return 0;
                    }
                    
                    //update best move
                    if v < g {
                        g = v;
                        *out_move = rmvs[omvs[i as usize].1];
                    }
                    
                    //update beta value
                    b = min(b,g);
                    
                    //prune branch
                    if alpha >= b {
                        //move caused a alpha cutoff (Thus dec history moves)
                        let m = rmvs[omvs[i as usize].1];
                        self.history[1][m.x() as usize][m.y() as usize] -= history_cutoff_weight;
                        break;
                    }
                    
                }
                g
            }
        };
        
        //update transposition table:
        
        let mut low = None;
        let mut high = None;
        
        if r <= alpha {high = Some(r);}
        if r > alpha && r < beta {high = Some(r); low = Some(r);}
        if r >= beta {low = Some(r);}
        
        self.trns.update(bb, low, high);
        
        r
    }
    
    
    ///The alpha beta algorithm implemented with iterative deepening to ensure
    ///constant time use.
    ///
    /// * `bb` The root position to search from
    /// * `h` The heuristic to use for searches
    /// * `t` The current turn
    /// * `alpha` The lower bound of the minimax score
    /// * `beta` The upper bound of the minimax score
    /// * `ms_left` The number of milliseconds allocated to make the move
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
                            bc, h, !t, a, b, d, ms_left, start, &mut to, &mut Move::null()
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
                            bc, h, !t, a, b, d, ms_left, start, &mut to, &mut Move::null()
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
    ///
    /// * `bb` The root position to search from
    /// * `h` The heuristic to use for searches
    /// * `t` The current turn
    /// * `g` A guess of the true minimax score.
    /// * `d` The maximum depth to search to
    /// * `ms_left` The number of milliseconds allocated to make the move
    /// * `start` The time the search was started
    /// * `to_flag` An out-parameter to signal when a timeout occured
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
    
        let mut best_move = Move::pass();
        let mut low = i32::MIN;
        let mut high = i32::MAX;
        let mut mvs : MoveList = [Move::null(); MAX_MOVES];
        
        let n = bb.get_moves(t, &mut mvs);
        
        while low < high {
            
            ////////////////////////////////////////////////////////////////////
            //do an alpha-beta search here
            ////////////////////////////////////////////////////////////////////
            let mut beta = g;
            
            if g == low {
                beta += 1;
            }
            
            g = self.alpha_beta_with_timeout(
                bb, h, t, beta-1, beta, d, ms_left, start, to_flag, &mut best_move
            );
                
            if *to_flag {
                return (Move::null(), 0); 
            }
            
            ////////////////////////////////////////////////////////////////////
            //end alpha-beta search, get result. The result must not be a timeout
            ////////////////////////////////////////////////////////////////////
            
            //update MTD(f) bounds
            if g < beta {
                high = g;
            } else {
                low = g;
            }
        
        }
        
        (best_move, g)
    }
    
    
    ///Iterative deepening implementation of MTD(f).
    /// * `bb` The root position to search from
    /// * `h` The heuristic to use for searches
    /// * `t` The current turn
    /// * `alpha` The lower bound of the minimax score
    /// * `beta` The upper bound of the minimax score
    /// * `d` The maximum depth to search to
    /// * `ms_left` The number of milliseconds allocated to make the move
    /// * `start` The time the search was started
    /// * `to_flag` An out-parameter to signal when a timeout occured
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
            
            //This allows us to search deeper as well as compare scores between
            //iterations more accurately
            d += 2;
        }
        
        let d = start.elapsed();
        cerrln!(
            "[COIN]: Done searching ({} s). Best move is {}", 
            (d.as_secs() as f32) + (d.subsec_nanos() as f32)/1_000_000_000f32,
            mv
        );
        
        return mv;
    }
    
}






















