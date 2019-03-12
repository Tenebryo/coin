use std::sync::atomic::AtomicUsize;
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::{Instant, Duration};

use std::path::Path;
use std::result::Result;
use std::error::Error;

use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io;
use std::i32;

use std::sync::Arc;

use bitboard::*;
use eval::*;
use solver::*;

use parking_lot::RwLock;
use parking_lot::Mutex;

use scoped_threadpool::Pool;

use rand;
use rand::distributions::{Distribution, Gamma};

use pcoinnet::ParallelCoinNet;
use pcoinnet::ParallelCoinNetWorker;

const EXPLORATION_CONSTANT : f32 = 0.25;
const FIRST_PLAY_URGENCY_CONSTANT : f32 = 0.25;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum WipeoutState {
    Win,
    Unknown,
    Loss,
}
// a quick minimax check to ensure that a wipeout is not imminent. Intended as
// a safeguard to prevent MCTS from completely ignoring paths that lead to
// wipeouts and losing as a result
fn check_wipeout(b : Board, d : usize) -> WipeoutState {
    if b.is_done() {
        // should only be possible for this to be a loss, since it is a
        // wipeout and the other player just went
        return WipeoutState::Loss;
    }

    if d == 0 {
        // if we reach the depth of search and are not a wipeout, we can't
        // conclude
        return WipeoutState::Unknown;
    }

    let mut mvs = empty_movelist();
    let n = b.get_moves(&mut mvs);

    // start with the worst case, a win for the opponent
    // if all options are wins for the opponent, then the that is our best
    // if any are losses for the opponent, that is our best
    // otherwise, if any are unknown, that is better than 
    let mut best = WipeoutState::Win;
    for i in 0..n {
        let mut bb = b;
        bb.f_do_move(mvs[i as usize]);
        let r = check_wipeout(bb, d-1);
        
        if r > best {
            best = r;
        }
    }

    // flip best value.
    match best {
        WipeoutState::Win => WipeoutState::Loss,
        WipeoutState::Unknown => WipeoutState::Unknown,
        WipeoutState::Loss => WipeoutState::Win,
    }
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
enum ParaMctsNodeState {
    Invalid,        // not a valid node.
    ProvenWin(f32), // there is at least one child that is a proven loss for the other player
    ProvenDraw,     // current player can force a draw
    Branch,         // already expanded
    Leaf,           // hasn't been expanded yet
    ProvenLoss(f32),// every move leads to a loss for the current player
}

/*  A node in the monte carlo tree search tree. */
// #[derive(Clone)]
struct ParaMctsNode {
    position: Position,
    state   : Mutex<ParaMctsNodeState>,
    value   : f32,
    edges   : Vec<ParaMctsEdge>,
}

impl ParaMctsNode {
    /// Returns a new Paramcts node. Automatically detects the solution state of the
    /// board and sets it appropriately.
    fn new(position : &Board) -> ParaMctsNode{
        let mut state = ParaMctsNodeState::Leaf;
        let mut score = 0;
        if position.is_done() {
            let (ps,os) = position.count_pieces();
            score = ps as i8 - os as i8;
            if score == 0 {
                state = ParaMctsNodeState::ProvenDraw;
            } else if score > 0 {
                state = ParaMctsNodeState::ProvenWin(score as f32 / 64.0);
            } else {
                state = ParaMctsNodeState::ProvenLoss(score as f32 / 64.0);
            }
        }

        ParaMctsNode {
            position    : Position::from_board(*position),
            state       : Mutex::new(state),
            value       : score as f32,
            edges       : vec![],
        }
    }

    /// Returns an invalid node that is only used as a placeholder
    fn empty() -> ParaMctsNode {
        ParaMctsNode {
            position    : Position::from_board(Board::new()),
            state       : Mutex::new(ParaMctsNodeState::Invalid),
            value       : 0.0,
            edges       : vec![],
        }
    }

    fn get_state(&self) -> ParaMctsNodeState {
        *(self.state.lock())
    }
    fn set_state(&self, s : ParaMctsNodeState ) {
        *(self.state.lock()) = s;
    }

    /// This function takes an unevaluated leaf node, evaluates it, populates
    /// its edges and scores, and propagates the results.
    fn expand_and_eval<E : Evaluator>(&mut self, e : &mut E) -> f32 {
        let b = self.position.to_board();

        let val = e.evaluate(&b);

        let mut moves = empty_movelist();
        let n = b.get_moves(&mut moves) as usize;

        self.edges.reserve_exact(n);
        for i in 0..n {
            self.edges.push(
                ParaMctsEdge::new(&b, moves[i],val.0[moves[i].offset() as usize], val.1)
            );
        }
        if n==0 {
            self.edges.push(ParaMctsEdge::new(&b, Move::pass(), 1.0, val.1));
        }


        self.set_state(ParaMctsNodeState::Branch);

        self.update_proven_state();

        let wipeout = check_wipeout(b, 4);
        // wipeouts are pretty extreme, so weight strongly
        match wipeout {
            WipeoutState::Win => {
                self.set_state(ParaMctsNodeState::ProvenWin(1000.0));
            },
            WipeoutState::Loss => {
                self.set_state(ParaMctsNodeState::ProvenLoss(-1000.0));
            },
            _ => ()
        }

        self.value = val.1;

        self.value
    }

    /// Scan through edges from this node and find the one with the highest qu
    /// score. Leaf nodes always have higher scores than branch nodes. If we
    /// picked a leaf node, we evaluate it, otherwise we recurse. Afterwards, we
    /// backpropagate scores and solved paths.
    fn select_and_backprop<E : Evaluator>(&self, e : &mut E) -> f32 {
        /*  find the max. */
        let n = self.edges.len();
        let mut ntot = 0;
        let mut ptot : f32 = 0.0;
        for i in 0..n {
            ntot += self.edges[i].get_sims();
            if self.edges[i].get_sims() > 0 {
                ptot += self.edges[i].get_sum();
            }
        }
        let sqrt_n = (ntot as f32).sqrt();
        let sqrt_p = FIRST_PLAY_URGENCY_CONSTANT * ptot.sqrt();

        let mut max_edge = n;
        let mut max_qu = 0.0;
        for i in 0..n {
            match self.edges[i].state() {
                ParaMctsNodeState::Branch => {
                    let qu = self.edges[i].qu(sqrt_n);
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                ParaMctsNodeState::Leaf => {
                    let qu = self.edges[i].qu(sqrt_n);// - sqrt_p;
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                _ => ()
            }
        }

        // ensure we found a max edge
        assert!(max_edge != n);

        /*  increment the number of simulations of this edge. */
        self.edges[max_edge].add_sim();

        // add virtual loss when we recurse and remove it after we are
        // done. This prevents parallel MCTS iterations from exploring the same
        // path too much.
        self.edges[max_edge].do_virtual_loss();
        let delta = - match self.edges[max_edge].state() {
            ParaMctsNodeState::Branch 
                => self.edges[max_edge].to.read().select_and_backprop(e),
            ParaMctsNodeState::Leaf
                => self.edges[max_edge].to.write().expand_and_eval(e),
            _   => panic!()
        };
        self.edges[max_edge].undo_virtual_loss();

        self.update_proven_state();

        self.edges[max_edge].add_delta(delta);

        delta
    }

    /// This function propagates solved paths up the tree.
    /// TODO: could remove the looping to improve performance, but its probably
    /// not a huge inprovement.
    fn update_proven_state(&self) {
        let mut best_state = ParaMctsNodeState::Invalid;
        for i in 0..(self.edges.len()) {
            if best_state < self.edges[i].state() {
                best_state = self.edges[i].state();
            }
        }

        let mut state = self.state.lock();

        match best_state {
            ParaMctsNodeState::ProvenLoss(s) => {
                // if there is a proven loss for the opponent, this move is a proven win
                *state = ParaMctsNodeState::ProvenWin(-s);
            },
            ParaMctsNodeState::ProvenDraw => {
                // if the best is a proven draw (this implies that there are no unproven branches left)
                // then this node is also a proven draw
                *state = ParaMctsNodeState::ProvenDraw;
            },
            ParaMctsNodeState::ProvenWin(s) => {
                // if the best state is a proven win, then this node is a proven loss for the current player
                *state = ParaMctsNodeState::ProvenLoss(-s);
            },
            //no other states matter:
            _ => {}
        }
    }

    /// Outputs the MCTS tree as a DOT graph for debug purposes
    fn render<W : Write>(&self, w : &mut W, name : String) -> Result<(),io::Error> {

        let mut edge_list = String::new();
        writeln!(w, "\"{}\" -> \"{}\" [label=\"[V={}]\", loop left];", name, name ,self.value)?;
        let state = *(self.state.lock());
        match state {
            ParaMctsNodeState::ProvenLoss(s) => {
                writeln!(w, "\"{}\" -> \"{}\" [label=\"[L: {}]\", loop right];", name, name , s)?;
            },
            ParaMctsNodeState::ProvenDraw => {
                writeln!(w, "\"{}\" -> \"{}\" [label=\"[D]\", loop right];", name, name)?;
            },
            ParaMctsNodeState::ProvenWin(s) => {
                writeln!(w, "\"{}\" -> \"{}\" [label=\"[W: {}]\", loop right];", name, name , s)?;
            },
            _ => ()
        }

        let ntot : usize = self.edges.iter().map(|e| e.get_sims()).sum();

        for (i, e) in self.edges.iter().enumerate() {
            let next_name = format!("{}-{}", name, e.action);
            writeln!(w, "\"{}\" -> \"{}\" [label=\"[M={},P={},N={},QU={}]\"];", name, next_name,e.action,e.prior,e.get_sims(),e.qu((ntot as f32).sqrt()))?;
            edge_list = format!("{} \"{}\"", edge_list, next_name);
            e.to.read().render(w, next_name)?;
        }

        writeln!(w, "{{ rank = same;{} }};", edge_list)
    }
    
    /// Populates a vector with the most likely line
    fn scan(&self, depth : usize, mvs : &mut Vec<Move>) {
        
        if depth == 0 {
            return;
        }
    
        let n = self.edges.len();
        let mut ntot = 0;
        let mut ptot :f32 = 0.0;
        for i in 0..n {
            ntot += self.edges[i].get_sims();
            if self.edges[i].get_sims() > 0 {
                ptot += self.edges[i].get_sum();
            }
        }
        let sqrt_n = (ntot as f32).sqrt();
        let sqrt_p = FIRST_PLAY_URGENCY_CONSTANT * ptot.sqrt();

        let mut max_edge = n;
        let mut max_qu = 0.0;
        for i in 0..n {
            match self.edges[i].state() {
                ParaMctsNodeState::Branch => {
                    let qu = self.edges[i].qu(sqrt_n);
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                ParaMctsNodeState::Leaf => {
                    let qu = self.edges[i].qu(sqrt_n);// - sqrt_p;
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                _ => ()
            }
        }

        if max_edge == n {
            // this means it is solved
            eprintln!("[COIN] Solved Line.");
            return;
        }

        /*  increment the number of simulations of this edge. */
        mvs.push(self.edges[max_edge].action);
        
        match self.edges[max_edge].state() {
            ParaMctsNodeState::Branch 
                => self.edges[max_edge].to.read().scan(depth - 1, mvs),
            _   => (),
        };
    }
    
    /// tries to solve the endgame using information from the MCTS searches to
    /// imform the search.
    fn solve_endgame(
        &self,
        mut alpha   : i32,
        beta        : i32,
        info        : &mut MoveOrderInfo,
        start       : Instant,
        ms_left     : Duration,
        timeout     : &mut bool,
        first       : bool
    ) -> (Move, i32) {

        match self.get_state() {
            ParaMctsNodeState::Invalid | ParaMctsNodeState::Leaf => {
                return (Move::null(), negamax_ordering(self.position.to_board(), alpha, beta, info, start, ms_left, timeout));
            },
            ParaMctsNodeState::ProvenWin(s)  if !first => {return (Move::null(), s.signum() as i32);},
            ParaMctsNodeState::ProvenDraw    if !first => {return (Move::null(), 0);},
            ParaMctsNodeState::ProvenLoss(s) if !first => {return (Move::null(), s.signum() as i32);},
            _ => (),
        }

        use std::mem;

        let bb = self.position.to_board();

        let mvs = self.edges.iter().map(|c| c.action).collect::<Vec<Move>>();
        let n = mvs.len();
        
        let mut idx = (0..n).collect::<Vec<usize>>();
        
        let mut extra : [i32; 64] = unsafe{mem::uninitialized()};
        for i in 0..n {
            extra[self.edges[i].action.offset() as usize] = self.edges[i].get_sims() as i32;
        }

        let empty = bb.total_empty() as usize;

        order_moves_extras(bb, &mut idx, &mvs[0..n], info, &extra);

        //negamax step
        let mut g = i32::MIN;
        let mut bm = Move::pass();

        //loop through all the moves
        for i in 0..n {
        
            let (_, v) = self.edges[idx[i]].to.read().solve_endgame(-beta, -alpha, info, start, ms_left, timeout, false);
            let v = -v;
            
            
            if *timeout || start.elapsed() >= ms_left {
                *timeout = true;
                return (Move::null(), 1064);
            }
            
            //update best move
            if g < v { g = v; bm = mvs[idx[i]]; }

            if alpha < g { alpha = g; }

            let o = bm.offset() as usize;
            info.butterfly[o] += 1;
            if alpha >= beta { 
                info.killers[empty][o] += 1;
                info.history[o] += 1;
                info.kmoves[empty].1[info.kmoves[o].0] = o;
                info.kmoves[empty].0 = (info.kmoves[empty].0 + 1) & 0b11;
                break; 
            }
        }

        (bm, g)
    }
}

const VIRTUAL_LOSS_CONST : isize = 3;

/*  An edge from one node to the next given an action in the monte carlo tree
    search tree. */
// #[derive(Clone)]
struct ParaMctsEdge {
    action  : Move,
    sims    : AtomicUsize,
    prior   : f32,
    sum     : Mutex<f32>,
    init    : f32,
    virt_sum: AtomicIsize,
    to      : RwLock<ParaMctsNode>,
}

impl ParaMctsEdge {
    fn new(position : &Board, action : Move, prior : f32, init : f32) -> ParaMctsEdge {
        let mut pos = *position;
        pos.f_do_move(action);
        ParaMctsEdge {
            action  : action,
            sims    : AtomicUsize::new(0),
            prior   : prior,
            sum     : Mutex::new(0.0),
            init    : init,
            virt_sum: AtomicIsize::new(0),
            to      : RwLock::new(ParaMctsNode::new(&pos)),
        }
    }

    /// adds a simulation to the record.
    fn add_sim(&self) {
        self.sims.fetch_add(1, Ordering::SeqCst);
    }

    /// gets the simulation count of the record.
    fn get_sims(&self) -> usize {
        self.sims.load(Ordering::SeqCst)
    }

    /// gets the simulation count of the record.
    fn get_sum(&self) -> f32 {
        *(self.sum.lock())
    }

    /// adds a delta to the record.
    fn add_delta(&self, delta : f32) {
        let mut sum = self.sum.lock();
        *sum += delta;
    }

    /// Applies virtual loss to this edge
    fn do_virtual_loss(&self) {
        &self.virt_sum.fetch_sub(VIRTUAL_LOSS_CONST, Ordering::SeqCst);
    }

    /// Removes virtual loss from this edge
    fn undo_virtual_loss(&self) {
        &self.virt_sum.fetch_add(VIRTUAL_LOSS_CONST, Ordering::SeqCst);
    }

    /// Computes the action value for this edge.
    fn q(&self) -> f32 {
        /*  we add a small term to blow up the q value if this edge hasn't been
            explored yet. */
        let sims = self.sims.load(Ordering::SeqCst);
        if sims == 0 {
            self.init
        } else {
            let virt_loss = self.virt_sum.load(Ordering::SeqCst) as f32;
            let sum = self.sum.lock();
            //
            (*sum + virt_loss) / sims as f32
        }
    }

    /// Computes the upper confidence bound for this edge.
    fn u(&self, n : f32) -> f32 {
        let sims = self.sims.load(Ordering::SeqCst);
        self.prior * n / (sims as f32 + 1.0)
    }

    /// Computes the tree exploration factor of the edges.
    fn qu(&self, n : f32) -> f32 {
        self.q() + EXPLORATION_CONSTANT * self.u(n)
    }

    /// returns the state of the node this edge points to.
    fn state(&self) -> ParaMctsNodeState {
        let to = self.to.read();
        let state = to.state.lock();
        *state
    }
}

struct ParaMctsTreeRoot {
    root : RwLock<ParaMctsNode>,
}

impl ParaMctsTreeRoot {
    fn new() -> ParaMctsTreeRoot {
        ParaMctsTreeRoot {
            root    : RwLock::new(ParaMctsNode::new(&Board::new())),
        }
    }

    /// returns true when the root node is a solved node.
    pub fn single_round<F: Evaluator>(&self, eval : &mut F) -> bool {
        match self.root.read().get_state() {
            ParaMctsNodeState::Branch => {
                self.root.read().select_and_backprop(eval);
                false
            },
            ParaMctsNodeState::Leaf => {
                self.root.write().expand_and_eval(eval);
                false
            },
            _ => {true}
        }
    }

    pub fn apply_dirichlet_noise<F: Evaluator>(
        &mut self, theta : f64, eval : &mut F
    ) {
        self.single_round(eval);

        let n = self.root.read().edges.len();

        let d_noise = dirichlet_dist(theta, n);
        for i in 0..n {
            self.root.write().edges[i].prior *= 0.75;
            self.root.write().edges[i].prior += (0.25 * d_noise[i]) as f32;
        }
    }

    pub fn solve_endgame(&mut self, start : Instant, ms_left : Duration, timeout : &mut bool, solver_info : &mut MoveOrderInfo) -> (Move, i32) {
        
        solver_info.leaf_nodes = 0;
        solver_info.ttable_hits = 0;

        
        let (bm, v) = self.root.read().solve_endgame(-1, 2, solver_info, start, ms_left, timeout, true);
        
        eprintln!("[COIN] Solver Nodes: {} TTable Hits: {}", 
            solver_info.leaf_nodes, solver_info.ttable_hits); 
        // if v < 0 {
        //     (Move::null(), -1064)
        // } else {
        //     (bm, v)
        // }
        (bm, v)
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune(&mut self, action : Move) -> usize {
        use std::mem;

        let i = self.root.read().position.to_board().get_move_index(action);

        let tmp = mem::replace(&mut self.root.write().edges[i].to, RwLock::new(ParaMctsNode::empty()));

        let n = self.root.read().edges[i].get_sims();

        self.root = tmp;
        
        n
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune_board(&mut self, board : Board) -> usize {
        use std::mem;

        let mut i = 32;

        for j in 0..(self.root.read().edges.len()) {
            let root = self.root.read();
            let bb = root.edges[j].to.read().position.to_board();
            if bb == board {
                i = j;
                break;
            }
        }


        let mut n = 0;
        let tmp = if i != 32 {
            n = self.root.read().edges[i].get_sims();
            mem::replace(&mut self.root.write().edges[i].to, RwLock::new(ParaMctsNode::empty()))
        } else {
            RwLock::new(ParaMctsNode::new(&board))
        };

        self.root = tmp;
        
        n
    }

    pub fn set_position(&mut self, position : Board) {
        self.root = RwLock::new(ParaMctsNode::new(&position));
    }

    pub fn count_sims(&self) -> usize {
        let mut nodes = 0;
        for e in &self.root.read().edges {
            nodes += e.get_sims();
        }
        nodes
    }

    fn render_tree<W: Write>(&self, w : &mut W) -> Result<(),io::Error>{
        writeln!(w, "digraph Paramcts_tree {{")?;
        self.root.read().render(w, "root".to_string())?;
        writeln!(w, "}}")
    }

    pub fn scan(&self, depth : usize) -> Vec<Move> {
        let mut mvs = vec![];
        
        self.root.read().scan(depth, &mut mvs);
        
        mvs
    }
}

/*  A struct containing and managing an instance of a monte carlo tree search */
pub struct ParaMctsTree {
    root        : Arc<ParaMctsTreeRoot>,
    pub eval_c  : ParallelCoinNet,
    pub eval_w  : ParallelCoinNetWorker,
    temp        : f32,
    solver_info : MoveOrderInfo,
    variation   : bool,
}

impl ParaMctsTree {
    pub fn new(model : &Path, vars : Option<&Path>) -> ParaMctsTree {
        let (net, workers) = ParallelCoinNet::new_worker_group(model, vars).unwrap();

        ParaMctsTree {
            root        : Arc::new(ParaMctsTreeRoot::new()),
            eval_c      : net,
            eval_w      : workers,
            temp        : 5.0e-2,
            solver_info : MoveOrderInfo::new(),
            variation   : false,
        }
    }

    /// Runs a fixed number of simulations or until the game is solved.
    pub fn n_rounds<F: Evaluator>(&self, n : usize, eval : &mut F) {
        for _ in 0..n {
            if self.root.single_round(eval) {
                break;
            }
        }
    }

    /// Runs simulations until a certain amount of time has passed or the game
    /// is solved.
    pub fn time_rounds<F: Evaluator>(&self, millis : u64, eval : &mut F) -> usize{
        let start = Instant::now();
        
        let mut count = 0;
        while start.elapsed() < Duration::from_millis(millis) {
            count += 1;
            if self.root.single_round(eval) {
                break;
            }
        }
        
        count
    }

    pub fn single_round(&mut self) -> bool {
        self.root.single_round(&mut self.eval_w.net)
    }

    pub fn para_time_rounds(&mut self, millis : u64, n_para : usize) -> usize {
        let start = Instant::now();
        
        // may create a lot of workers, but they will be mostly idle.
        let mut tp = Pool::new(n_para as u32);

        // count how many node were expanded this turn
        let count = Arc::new(AtomicUsize::new(0));

        // variable to signal workers when the
        let done = Arc::new(AtomicBool::new(false));
        tp.scoped(|scoped| {
            for id in 0..n_para {
                let mut eval = self.eval_c.clone();
                let tree = self.root.clone();
                let done = done.clone();
                let count = count.clone();
                scoped.execute(move ||{
                    // loop running MCTS rounds in parallel. Because the parallel
                    // mcts worker batches calls, each of these will be forced to
                    // run phase-locked. The loop continues while 
                    while done.load(Ordering::SeqCst) {
                        if tree.single_round(&mut eval) {
                            // we've reached a solved state, so we should exit
                            done.fetch_or(true, Ordering::SeqCst);
                        }
                        count.fetch_add(1, Ordering::SeqCst);
                    }
                })
            }
        });

        // loop while time allows
        while start.elapsed() < Duration::from_millis(millis) {
            // check if something has flagged for the end of sampling.
            if done.load(Ordering::SeqCst) {
                break;
            }
            // do a round of batched calls to tensorflow.
            // deliberately left timeout long so that it will be obvious if
            // if ever becomes an issue.
            self.eval_w.do_a_work(n_para, Duration::from_millis(1000));
        }
        // stop all the worker threads.
        done.fetch_or(true, Ordering::SeqCst);
        // do one more batch to make sure the threads finish.
        self.eval_w.do_a_work(n_para, Duration::from_millis(10));
        
        count.load(Ordering::SeqCst)
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune(&mut self, action : Move) -> usize {
        self.single_round();
        Arc::get_mut(&mut self.root).unwrap().prune(action)
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune_board(&mut self, board : Board) -> usize {
        self.single_round();
        Arc::get_mut(&mut self.root).unwrap().prune_board(board)
    }

    pub fn set_position(&mut self, position : Board) {
        Arc::get_mut(&mut self.root).unwrap().set_position(position);
    }

    pub fn set_temp(&mut self, temp : f32) {
        self.temp = temp;
    }

    pub fn set_variation(&mut self, ena : bool) {
        self.variation = ena;
    }

    pub fn apply_dirichlet_noise(&mut self, theta : f64) {
        Arc::get_mut(&mut self.root).unwrap().apply_dirichlet_noise(theta, &mut self.eval_w.net)
    }
    
    pub fn count_sims(&self) -> usize {
        self.root.count_sims()
    }

    fn render_tree<W: Write>(&self, w : &mut W) -> Result<(),io::Error>{
        self.root.render_tree(w)
    }
    
    pub fn scan(&self, depth : usize) -> Vec<Move> {
        self.root.scan(depth)
    }
    
    pub fn solve_endgame(&mut self, start : Instant, ms_left : Duration, timeout : &mut bool) -> (Move, i32) {
        Arc::get_mut(&mut self.root).unwrap().single_round(&mut self.eval_w.net);
        Arc::get_mut(&mut self.root).unwrap().solve_endgame(start, ms_left, timeout, &mut self.solver_info)
    }
}

fn dirichlet_dist(theta : f64, n : usize) -> Vec<f64> {
    let g_dist = Gamma::new(theta, 1.0);
    let mut rng = rand::thread_rng();

    let mut data = (0..n).map(|_| g_dist.sample(&mut rng)).collect::<Vec<_>>();
    let sum = data.iter().sum::<f64>();
    
    // normalize distribution
    data.iter().map(|x| x / sum).collect::<Vec<_>>()
}

/// The ParaMctsTree is an Evaluator itself (since it is an improvement operator on
/// whatever evaluator is given to it).
impl Evaluator for ParaMctsTree {
    fn evaluate(&mut self, _input : &EvalInput) -> EvalOutput {
        let mut res = EvalOutput::new();
        let mut res64 = ([0.0f64; 64], 0.0);

        let solved = match self.root.root.read().get_state() {
            ParaMctsNodeState::ProvenLoss(_) |
            ParaMctsNodeState::ProvenWin(_) |
            ParaMctsNodeState::ProvenDraw => true,
            _   => false,
        };

        if solved {
            /*  If the node is solved, choose the appropriate values. */
            let n = self.root.root.read().edges.len();
            let mut best_state = ParaMctsNodeState::Invalid;
            let mut best_i = n;
            for i in 0..n {
                if best_state < self.root.root.read().edges[i].state() {
                    best_state = self.root.root.read().edges[i].state();
                    best_i = i;
                }
            }

            match best_state {
                ParaMctsNodeState::ProvenLoss(s) => {
                    // if there is a proven loss for the opponent, this move is a proven win
                    res.0[self.root.root.read().edges[best_i].action.offset() as usize] = 1.0;
                    res.1 = -s;
                },
                ParaMctsNodeState::ProvenDraw => {
                    // if the best is a proven draw (this implies that there are no unproven branches left)
                    // then this node is also a proven draw
                    res.0[self.root.root.read().edges[best_i].action.offset() as usize] = 1.0;
                    res.1 = 0.0;
                },
                ParaMctsNodeState::ProvenWin(s) => {
                    // if the best state is a proven win, then this node is a proven loss for the current player
                    res.0[self.root.root.read().edges[best_i].action.offset() as usize] = 1.0;
                    res.1 = -s;
                },
                //no other states matter:
                _ => {}
            }
        } else {
            let mut nsum = 0;
            let mut ntsum = 0.0;
            let mut wsum = 0.0;
            for e in &self.root.root.read().edges {
                match e.state() {
                    ParaMctsNodeState::ProvenWin(_s) => {
                        // if the best state is a proven win, then this node is a proven loss for the current player
                        res64.0[e.action.offset() as usize] = 0.0;
                        //don't go down this path
                        continue;
                    },
                    _ => ()
                }
                let tmp = (e.get_sims() as f64).powf(1.0/self.temp as f64);
                nsum += e.get_sims();
                ntsum += tmp;
                wsum += e.get_sum();

                res64.0[e.action.offset() as usize] = tmp;
            }

            for i in 0..64 {
                res.0[i] = (res64.0[i] / ntsum) as f32;
            }

            if self.variation {
                let dchlet_noise = dirichlet_dist(0.03, 64);
                for i in 0..64 {
                    res.0[i] = (0.75 * res.0[i]) + (0.25 * dchlet_noise[i]) as f32;
                }
            }

            res.1 = wsum / (nsum as f32);
        }

        res
    }

    fn evaluate_batch(&mut self, input : &[EvalInput]) -> Vec<EvalOutput> {
        input.iter().map(|i| self.evaluate(i)).collect::<Vec<_>>()
    }

    fn train(&mut self, input : &[EvalInput], target : &[EvalOutput], eta : f32) -> f32 {
        self.eval_w.train(input, target, eta)
    }

    fn save(&mut self, filename : &Path) -> Result<(), Box<Error>> {
        self.eval_w.save(filename)
    }
    
    fn load(&mut self, filename : &Path) -> Result<(), Box<Error>>{
        self.eval_w.load(filename)
    }
}

#[test]
fn visualize_mcts_tree() {

    let mut evals = ParaMctsTree::new(&Path::new("../params/CoinNet_model.pb"), Some(&Path::new("../params/CoinNet-170")));

    let dir = Path::new("./data/graphviz4/");

    fs::create_dir_all(&dir);

    // let b = Board::from_string(b"________\n________\n_W_WWWW_\n__WBBW__\n_WWWWW__\n_WWWWW__\n__W__W__\n________");
    let b = Board::new();

    evals.prune_board(b);

    for i in 0..100 {
        eprintln!("Step {:3}", i);
        evals.single_round();

        let mut f = File::create(dir.join(&Path::new(&format!("step-{:03}.dot",i)))).unwrap();

        evals.render_tree(&mut f).unwrap();
    }
}