use std::time::{Instant, Duration};

use std::path::Path;
use std::result::Result;
use std::error::Error;

use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io;
use std::i32;

use bitboard::*;
use eval::*;
use solver::*;

use rand;
use rand::distributions::{Distribution, Gamma};

const EXPLORATION_CONSTANT : f32 = 2.0;
const FIRST_PLAY_URGENCY_CONSTANT : f32 = 0.25;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
enum MctsNodeState {
    Invalid,        // not a valid node.
    ProvenWin(f32), // there is at least one child that is a proven loss for the other player
    ProvenDraw,     // current player can force a draw
    Branch,         // already expanded
    Leaf,           // hasn't been expanded yet
    ProvenLoss(f32),// every move leads to a loss for the current player
}

/*  A node in the monte carlo tree search tree. */
#[derive(Clone)]
struct MctsNode {
    position: Position,
    state   : MctsNodeState,
    value   : f32,
    edges   : Vec<MctsEdge>,
}

impl MctsNode {
    /// Returns a new mcts node. Automatically detects the solution state of the
    /// board and sets it appropriately.
    fn new(position : &Board) -> MctsNode{
        let mut state = MctsNodeState::Leaf;
        let mut score = 0;
        if position.is_done() {
            let (ps,os) = position.count_pieces();
            score = ps as i8 - os as i8;
            if score == 0 {
                state = MctsNodeState::ProvenDraw;
            } else if score > 0 {
                state = MctsNodeState::ProvenWin(score as f32 / 64.0);
            } else {
                state = MctsNodeState::ProvenLoss(score as f32 / 64.0);
            }
        }

        MctsNode {
            position    : Position::from_board(*position),
            state       : state,
            value       : score as f32,
            edges       : vec![],
        }
    }

    /// Returns an invalid node that is only used as a placeholder
    fn empty() -> MctsNode {
        MctsNode {
            position    : Position::from_board(Board::new()),
            state       : MctsNodeState::Invalid,
            value       : 0.0,
            edges       : vec![],
        }
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
                MctsEdge::new(&b, moves[i],val.0[moves[i].offset() as usize], val.1)
            );
        }
        if n==0 {
            self.edges.push(MctsEdge::new(&b, Move::pass(), 1.0, val.1));
        }

        self.state = MctsNodeState::Branch;

        self.update_proven_state();

        self.value = val.1;

        self.value
    }

    /// Scan through edges from this node and find the one with the highest qu
    /// score. Leaf nodes always have higher scores than branch nodes. If we
    /// picked a leaf node, we evaluate it, otherwise we recurse. Afterwards, we
    /// backpropagate scores and solved paths.
    fn select_and_backprop<E : Evaluator>(&mut self, e : &mut E) -> f32 {
        /*  find the max. */
        let n = self.edges.len();
        let mut ntot = 0;
        let mut ptot = 0.0;
        for i in 0..n {
            ntot += self.edges[i].sims;
            if self.edges[i].sims > 0 {
                ptot += self.edges[i].sum;
            }
        }
        let sqrt_n = (ntot as f32).sqrt();
        let sqrt_p = FIRST_PLAY_URGENCY_CONSTANT * ptot.sqrt();

        let mut max_edge = n;
        let mut max_qu = 0.0;
        for i in 0..n {
            match self.edges[i].state() {
                MctsNodeState::Branch => {
                    let qu = self.edges[i].qu(sqrt_n);
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                MctsNodeState::Leaf => {
                    let qu = self.edges[i].qu(sqrt_n);// - sqrt_p;
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                _ => ()
            }
        }

        assert!(max_edge != n);

        /*  increment the number of simulations of this edge. */
        self.edges[max_edge].sims += 1;

        let delta = - match self.edges[max_edge].state() {
            MctsNodeState::Branch 
                => self.edges[max_edge].to.select_and_backprop(e),
            MctsNodeState::Leaf
                => self.edges[max_edge].to.expand_and_eval(e),
            _   => panic!()
        };

        self.update_proven_state();

        self.edges[max_edge].sum += delta;

        delta
    }

    /// This function propagates solved paths up the tree.
    fn update_proven_state(&mut self) {
        let mut best_state = MctsNodeState::Invalid;
        for i in 0..(self.edges.len()) {
            if best_state < self.edges[i].state() {
                best_state = self.edges[i].state();
            }
        }

        match best_state {
            MctsNodeState::ProvenLoss(s) => {
                // if there is a proven loss for the opponent, this move is a proven win
                self.state = MctsNodeState::ProvenWin(-s);
            },
            MctsNodeState::ProvenDraw => {
                // if the best is a proven draw (this implies that there are no unproven branches left)
                // then this node is also a proven draw
                self.state = MctsNodeState::ProvenDraw;
            },
            MctsNodeState::ProvenWin(s) => {
                // if the best state is a proven win, then this node is a proven loss for the current player
                self.state = MctsNodeState::ProvenLoss(-s);
            },
            //no other states matter:
            _ => {}
        }
    }

    fn render<W : Write>(&self, w : &mut W, name : String) -> Result<(),io::Error> {

        let mut edge_list = String::new();
        writeln!(w, "\"{}\" -> \"{}\" [label=\"[V={}]\", loop left];", name, name ,self.value)?;
        match self.state {
            MctsNodeState::ProvenLoss(s) => {
                writeln!(w, "\"{}\" -> \"{}\" [label=\"[L: {}]\", loop right];", name, name , s)?;
            },
            MctsNodeState::ProvenDraw => {
                writeln!(w, "\"{}\" -> \"{}\" [label=\"[D]\", loop right];", name, name)?;
            },
            MctsNodeState::ProvenWin(s) => {
                writeln!(w, "\"{}\" -> \"{}\" [label=\"[W: {}]\", loop right];", name, name , s)?;
            },
            _ => ()
        }

        let ntot : usize = self.edges.iter().map(|e| e.sims).sum();

        for (i, e) in self.edges.iter().enumerate() {
            let next_name = format!("{}-{}", name, e.action);
            writeln!(w, "\"{}\" -> \"{}\" [label=\"[M={},P={},N={},QU={}]\"];", name, next_name,e.action,e.prior,e.sims,e.qu((ntot as f32).sqrt()))?;
            edge_list = format!("{} \"{}\"", edge_list, next_name);
            e.to.render(w, next_name)?;
        }

        writeln!(w, "{{ rank = same;{} }};", edge_list)
    }
    
    fn scan(&self, depth : usize, mvs : &mut Vec<Move>) {
        
        if depth == 0 {
            return;
        }
    
        let n = self.edges.len();
        let mut ntot = 0;
        let mut ptot = 0.0;
        for i in 0..n {
            ntot += self.edges[i].sims;
            if self.edges[i].sims > 0 {
                ptot += self.edges[i].sum;
            }
        }
        let sqrt_n = (ntot as f32).sqrt();
        let sqrt_p = FIRST_PLAY_URGENCY_CONSTANT * ptot.sqrt();

        let mut max_edge = n;
        let mut max_qu = 0.0;
        for i in 0..n {
            match self.edges[i].state() {
                MctsNodeState::Branch => {
                    let qu = self.edges[i].qu(sqrt_n);
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                MctsNodeState::Leaf => {
                    let qu = self.edges[i].qu(sqrt_n);// - sqrt_p;
                    if qu > max_qu || max_edge == n {
                        max_edge = i;
                        max_qu = qu;
                    }
                },
                _ => ()
            }
        }

        assert!(max_edge != n);

        /*  increment the number of simulations of this edge. */
        mvs.push(self.edges[max_edge].action);
        
        match self.edges[max_edge].state() {
            MctsNodeState::Branch 
                => self.edges[max_edge].to.scan(depth - 1, mvs),
            _   => (),
        };
    }
    
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

        match self.state {
            MctsNodeState::Invalid | MctsNodeState::Leaf => {
                return (Move::null(), negamax_ordering(self.position.to_board(), alpha, beta, info, start, ms_left, timeout));
            },
            MctsNodeState::ProvenWin(s)  if !first => {return (Move::null(), s.signum() as i32);},
            MctsNodeState::ProvenDraw    if !first => {return (Move::null(), 0);},
            MctsNodeState::ProvenLoss(s) if !first => {return (Move::null(), s.signum() as i32);},
            _ => (),
        }

        use std::mem;

        let bb = self.position.to_board();

        let mvs = self.edges.iter().map(|c| c.action).collect::<Vec<Move>>();
        let n = mvs.len();
        
        let mut idx = (0..n).collect::<Vec<usize>>();
        
        let mut extra : [i32; 64] = unsafe{mem::uninitialized()};
        for i in 0..n {
            extra[self.edges[i].action.offset() as usize] = self.edges[i].sims as i32;
        }

        let empty = bb.total_empty() as usize;

        order_moves_extras(bb, &mut idx, &mvs[0..n], info, &extra);

        //negamax step
        let mut g = i32::MIN;
        let mut bm = Move::pass();

        //loop through all the moves
        for i in 0..n {
        
            let (_, v) = self.edges[idx[i]].to.solve_endgame(-beta, -alpha, info, start, ms_left, timeout, false);
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

/*  An edge from one node to the next given an action in the monte carlo tree
    search tree. */
#[derive(Clone)]
struct MctsEdge {
    action  : Move,
    sims    : usize,
    prior   : f32,
    sum     : f32,
    init    : f32,
    to      : MctsNode,
}

impl MctsEdge {
    fn new(position : &Board, action : Move, prior : f32, init : f32) -> MctsEdge {
        let mut pos = *position;
        pos.f_do_move(action);
        MctsEdge {
            action  : action,
            sims    : 0,
            prior   : prior,
            sum     : 0.0,
            init    : init,
            to      : MctsNode::new(&pos),
        }
    }

    /// Computes the action value for this edge.
    fn q(&self) -> f32 {
        /*  we add a small term to blow up the q value if this edge hasn't been
            explored yet. */
        if self.sims == 0 {
            self.init
        } else {
            self.sum / self.sims as f32
        }
    }

    /// Computes the upper confidence bound for this edge.
    fn u(&self, n : f32) -> f32 {
        self.prior /* * n */ / (self.sims as f32 + 1.0)
    }

    /// Computes the tree exploration factor of the edges.
    fn qu(&self, n : f32) -> f32 {
        self.q() + EXPLORATION_CONSTANT * self.u(n)
    }

    /// returns the state of the node this edge points to.
    fn state(&self) -> MctsNodeState {
        self.to.state
    }
}

/*  A struct containing and managing an instance of a monte carlo tree search */
#[derive(Clone)]
pub struct MctsTree<E : Evaluator> {
    root    : MctsNode,
    pub eval    : E,
    temp    : f32,
    solver_info : MoveOrderInfo,
    variation : bool,
}

impl<E : Evaluator> MctsTree<E> {
    pub fn new(eval : E) -> MctsTree<E> {
        MctsTree {
            root    : MctsNode::new(&Board::new()),
            eval    : eval,
            temp    : 5.0e-2,
            solver_info : MoveOrderInfo::new(),
            variation : false,
        }
    }

    /// returns true when the root node is a solved node.
    pub fn single_round(&mut self) -> bool {
        match self.root.state {
            MctsNodeState::Branch => {
                self.root.select_and_backprop(&mut self.eval);
                false
            },
            MctsNodeState::Leaf => {
                self.root.expand_and_eval(&mut self.eval);
                false
            },
            _ => {true}
        }
    }

    /// Runs a fixed number of simulations or until the game is solved.
    pub fn n_rounds(&mut self, n : usize) {
        for _ in 0..n {
            if self.single_round() {
                break;
            }
        }
    }

    /// Runs simulations until a certain amount of time has passed or the game
    /// is solved.
    pub fn time_rounds(&mut self, millis : u64) -> usize{
        let start = Instant::now();
        
        let mut count = 0;
        while start.elapsed() < Duration::from_millis(millis) {
            count += 1;
            if self.single_round() {
                break;
            }
        }
        
        count
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune(&mut self, action : Move) -> usize {
        use std::mem;

        let i = self.root.position.to_board().get_move_index(action);

        if self.root.edges.len() == 0 {
            self.root.expand_and_eval(&mut self.eval);
        }

        let tmp = mem::replace(&mut self.root.edges[i].to, MctsNode::empty());

        let n = self.root.edges[i].sims;

        self.root = tmp;
        
        n
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune_board(&mut self, board : Board) -> usize {
        use std::mem;

        if self.root.edges.len() == 0 {
            self.root.expand_and_eval(&mut self.eval);
        }

        let mut i = 32;

        for j in 0..(self.root.edges.len()) {
            let bb = self.root.edges[j].to.position.to_board();
            if bb == board {
                i = j;
                break;
            }
        }


        let mut n = 0;
        let tmp = if i != 32 {
            n = self.root.edges[i].sims;
            mem::replace(&mut self.root.edges[i].to, MctsNode::empty())
        } else {
            MctsNode::new(&board)
        };

        self.root = tmp;
        
        n
    }

    pub fn set_position(&mut self, position : Board) {
        self.root = MctsNode::new(&position);
    }

    pub fn set_temp(&mut self, temp : f32) {
        self.temp = temp;
    }

    pub fn set_variation(&mut self, ena : bool) {
        self.variation = ena;
    }

    pub fn apply_dirichlet_noise(&mut self, theta : f64) {
        self.single_round();

        let n = self.root.edges.len();

        let d_noise = dirichlet_dist(theta, n);
        for i in 0..n {
            self.root.edges[i].prior *= 0.75;
            self.root.edges[i].prior += (0.25 * d_noise[i]) as f32;
        }
    }
    
    pub fn count_sims(&self) -> usize {
        let mut nodes = 0;
        for e in &self.root.edges {
            nodes += e.sims;
        }
        nodes
    }

    fn render_tree<W: Write>(&self, w : &mut W) -> Result<(),io::Error>{
        writeln!(w, "digraph mcts_tree {{")?;
        self.root.render(w, "root".to_string())?;
        writeln!(w, "}}")
    }
    
    pub fn scan(&self, depth : usize) -> Vec<Move> {
        let mut mvs = vec![];
        
        self.root.scan(depth, &mut mvs);
        
        mvs
    }
    
    pub fn solve_endgame(&mut self, start : Instant, ms_left : Duration, timeout : &mut bool) -> (Move, i32) {
        
        self.solver_info.leaf_nodes = 0;
        self.solver_info.ttable_hits = 0;

        // ensure the root node is always expanded.
        match self.root.state {
            MctsNodeState::Leaf | MctsNodeState::Invalid => {
                self.root.expand_and_eval(&mut self.eval);
            },
            _ => ()
        }
        
        let (bm, v) = self.root.solve_endgame(-1, 2, &mut self.solver_info, start, ms_left, timeout, true);
        
        eprintln!("[COIN] Solver Nodes: {} TTable Hits: {}", 
            self.solver_info.leaf_nodes, self.solver_info.ttable_hits); 
        // if v < 0 {
        //     (Move::null(), -1064)
        // } else {
        //     (bm, v)
        // }
        (bm, v)
    }
}

fn dirichlet_dist(theta : f64, n : usize) -> Vec<f64> {
    let mut g_dist = Gamma::new(theta, 1.0);
    let mut rng = rand::thread_rng();

    let mut data = (0..n).map(|_| g_dist.sample(&mut rng)).collect::<Vec<_>>();
    let sum = data.iter().sum::<f64>();
    
    // normalize distribution
    data.iter().map(|x| x / sum).collect::<Vec<_>>()
}

/// The MctsTree is an Evaluator itself (since it is an improvement operator on
/// whatever evaluator is given to it).
impl<E : Evaluator> Evaluator for MctsTree<E> {
    fn evaluate(&mut self, _input : &EvalInput) -> EvalOutput {
        let mut res = EvalOutput::new();
        let mut res64 = ([0.0f64; 64], 0.0);

        let solved = match self.root.state {
            MctsNodeState::ProvenLoss(_) |
            MctsNodeState::ProvenWin(_) |
            MctsNodeState::ProvenDraw => true,
            _   => false,
        };

        if solved {
            /*  If the node is solved, choose the appropriate values. */
            let n = self.root.edges.len();
            let mut best_state = MctsNodeState::Invalid;
            let mut best_i = n;
            for i in 0..n {
                if best_state < self.root.edges[i].state() {
                    best_state = self.root.edges[i].state();
                    best_i = i;
                }
            }

            match best_state {
                MctsNodeState::ProvenLoss(s) => {
                    // if there is a proven loss for the opponent, this move is a proven win
                    res.0[self.root.edges[best_i].action.offset() as usize] = 1.0;
                    res.1 = -s;
                },
                MctsNodeState::ProvenDraw => {
                    // if the best is a proven draw (this implies that there are no unproven branches left)
                    // then this node is also a proven draw
                    res.0[self.root.edges[best_i].action.offset() as usize] = 1.0;
                    res.1 = 0.0;
                },
                MctsNodeState::ProvenWin(s) => {
                    // if the best state is a proven win, then this node is a proven loss for the current player
                    res.0[self.root.edges[best_i].action.offset() as usize] = 1.0;
                    res.1 = -s;
                },
                //no other states matter:
                _ => {}
            }
        } else {
            let mut nsum = 0;
            let mut ntsum = 0.0;
            let mut wsum = 0.0;
            for e in &self.root.edges {
                match e.state() {
                    MctsNodeState::ProvenWin(s) => {
                        // if the best state is a proven win, then this node is a proven loss for the current player
                        res64.0[e.action.offset() as usize] = 0.0;
                        //don't go down this path
                        continue;
                    },
                    _ => ()
                }
                let tmp = (e.sims as f64).powf(1.0/self.temp as f64);
                nsum += e.sims;
                ntsum += tmp;
                wsum += e.sum;

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
        self.eval.train(input, target, eta)
    }

    fn save(&mut self, filename : &Path) -> Result<(), Box<Error>> {
        self.eval.save(filename)
    }
    
    fn load(&mut self, filename : &Path) -> Result<(), Box<Error>>{
        self.eval.load(filename)
    }
}

#[test]
fn visualize_mcts_tree() {

    let mut cnet = CoinNet::new(&Path::new("../params/CoinNet_model.pb")).unwrap();
    cnet.load(&Path::new("../params/CoinNet-170")).unwrap();

    let mut evals = MctsTree::new(cnet);

    let dir = Path::new("./data/graphviz2/");

    fs::create_dir_all(&dir);

    let b = Board::from_string(b"________\n________\n_W_WWWW_\n__WBBW__\n_WWWWW__\n_WWWWW__\n__W__W__\n________");

    evals.prune_board(b);

    for i in 0..100 {
        eprintln!("Step {:3}", i);
        evals.single_round();

        let mut f = File::create(dir.join(&Path::new(&format!("step-{:03}.dot",i)))).unwrap();

        evals.render_tree(&mut f).unwrap();
    }
}

#[test]
fn solve_endgame_test() {
    use rand;
    use rand::Rng;

    let mut cnet = CoinNet::new(&Path::new("../params/CoinNet_model.pb")).unwrap();
    cnet.load(&Path::new("../params/CoinNet-170")).unwrap();

    let mut evals = MctsTree::new(cnet);

    
    let mut mvs = empty_movelist();
    let mut r = rand::thread_rng();
    
    for d in 10..30 {
        let mut b = Board::new();
        while b.total_empty() > d {
            if b.is_done() {
                b = Board::new();
            }
            
            let n = b.get_moves(&mut mvs) as usize;
            
            let m = r.choose(&mvs[..n]).unwrap();
            
            b.f_do_move(*m);
        }
        
        evals.prune_board(b);
        let start = Instant::now();
        evals.single_round();
        // evals.n_rounds(400);
        let score = evals.solve_endgame(start, Duration::from_millis(30_000), &mut false);
        eprintln!("Empty: {:2} Score: {} Move: {} Elapsed: {:?}", d, score.1, score.0, start.elapsed());
    }
}

#[test]
fn ffo_test() {

    let mut cnet = CoinNet::new(&Path::new("../params/CoinNet_model.pb")).unwrap();
    cnet.load(&Path::new("../params/CoinNet-170")).unwrap();
    let mut evals = MctsTree::new(cnet);

    let ffo_positions = [
        Board::from_string(b"________\n________\n_W_WWWW_\n__WBBW__\n_WWWWW__\n_WWWWW__\n__W__W__\n________"),
        Board::from_string(b"___W____\n___W____\n_W_WWWW_\n__WWBW__\n_WWWWW__\n_WWWWW__\n__W__W__\n________"),
        Board::from_string(b"________\nW_W_____\n_WWWWBBB\nBBWBWW__\nBBBWWWW_\nBBWWWW__\nB_BBBW__\n___BB___")
    ];

    let start = Instant::now();

    for b in ffo_positions.iter() {

        let mut to = false;
        println!("Board: \n{}", b);
        println!("Board: \n{:?}", b);
        evals.prune_board(*b);
        evals.n_rounds(3000);
        println!("Solving...");
        let score = evals.solve_endgame(start, Duration::from_secs(100_000_000), &mut to);
        println!("Result: {} {}", score.0, score.1);
        assert!(!to);
    }

    println!("Final Time: {:?}", start.elapsed());
}