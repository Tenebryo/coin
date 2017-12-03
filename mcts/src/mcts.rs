use std::time::{Instant, Duration};

use std::path::Path;
use std::result::Result;
use std::error::Error;

use bitboard::*;
use eval::*;

const EXPLORATION_CONSTANT : f32 = 1.0;

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
            value       : score as f32 / 64.0,
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

        let val = e.evaluate(&[b]);

        let mut moves = empty_movelist();
        let n = b.get_moves(&mut moves) as usize;

        self.edges.reserve_exact(n);
        for i in 0..n {
            self.edges.push(
                MctsEdge::new(&b, moves[i],val.0[moves[i].offset() as usize])
            );
        }
        if n==0 {
            self.edges.push(MctsEdge::new(&b, Move::pass(),1.0));
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
        for i in 0..n {
            ntot += self.edges[i].sims;
        }
        let sqrt_n = (ntot as f32).sqrt();

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
                    let qu = self.edges[i].qu(sqrt_n) + 1.0e4;
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

        let delta = -match self.edges[max_edge].state() {
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
}

/*  An edge from one node to the next given an action in the monte carlo tree
    search tree. */
#[derive(Clone)]
struct MctsEdge {
    action  : Move,
    sims    : usize,
    prior   : f32,
    sum     : f32,
    to      : MctsNode,
}

impl MctsEdge {
    fn new(position : &Board, action : Move, prior : f32) -> MctsEdge {
        let mut pos = *position;
        pos.f_do_move(action);
        MctsEdge {
            action  : action,
            sims    : 0,
            prior   : prior,
            sum     : 0.0,
            to      : MctsNode::new(&pos),
        }
    }

    /// Computes the action value for this edge.
    fn q(&self) -> f32 {
        /*  we add a small term to blow up the q value if this edge hasn't been
            explored yet. */
        self.sum / (self.sims as f32 + 1.0e-32)
    }

    /// Computes the upper confidence bound for this edge.
    fn u(&self, n : f32) -> f32 {
        self.prior * n / (self.sims as f32 + 1.0)
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
    eval    : E,
    temp    : f32,
}

impl<E : Evaluator> MctsTree<E> {
    pub fn new(eval : E) -> MctsTree<E> {
        MctsTree {
            root    : MctsNode::new(&Board::new()),
            eval    : eval,
            temp    : 5.0e-2,
        }
    }

    /// Runs a fixed number of simulations or until the game is solved.
    pub fn n_rounds(&mut self, n : usize) {
        for _ in 0..n {
            match self.root.state {
                MctsNodeState::Branch => {
                    self.root.select_and_backprop(&mut self.eval);
                },
                MctsNodeState::Leaf => {
                    self.root.expand_and_eval(&mut self.eval);
                },
                _ => {break;}
            }
        }
    }

    /// Runs simulations until a certain amount of time has passed or the game
    /// is solved.
    pub fn time_rounds(&mut self, millis : u64) {
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(millis) {
            match self.root.state {
                MctsNodeState::Branch => {
                    self.root.select_and_backprop(&mut self.eval);
                },
                MctsNodeState::Leaf => {
                    self.root.expand_and_eval(&mut self.eval);
                },
                _ => {break;}
            }
        }
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune(&mut self, action : Move) {
        use std::mem;

        let i = self.root.position.to_board().get_move_index(action);

        if self.root.edges.len() == 0 {
            self.root.expand_and_eval(&mut self.eval);
        }

        let tmp = mem::replace(&mut self.root.edges[i].to, MctsNode::empty());

        self.root = tmp;
    }

    /// Takes the opponents move and prunes all the now-irrelevant tree parts
    pub fn prune_board(&mut self, board : Board) {
        use std::mem;

        if self.root.edges.len() == 0 {
            self.root.expand_and_eval(&mut self.eval);
        }

        let i = 32;

        for (j,e) in &self.root.edges().enumerate() {
            if e.position.to_board() == board {
                i = j;
                break;
            }
        }

        let tmp = mem::replace(&mut self.root.edges[i].to, MctsNode::empty());

        self.root = tmp;
    }

    pub fn set_position(&mut self, position : Board) {
        self.root = MctsNode::new(&position);
    }

    pub fn set_temp(&mut self, temp : f32) {
        self.temp = temp;
    }
}

/// The MctsTree is an Evaluator itself (since it is an improvement operator on
/// whatever evaluator is given to it).
impl<E : Evaluator> Evaluator for MctsTree<E> {
    fn evaluate(&mut self, input : &EvalInput) -> EvalOutput {
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
                    res.1 = s;
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
                    res.1 = s;
                },
                //no other states matter:
                _ => {}
            }
        } else {
            let mut nsum = 0;
            let mut ntsum = 0.0;
            let mut wsum = 0.0;
            for e in &self.root.edges {
                let tmp = (e.sims as f64).powf(1.0/self.temp as f64);
                nsum += e.sims;
                ntsum += tmp;
                wsum += e.sims as f32 * e.sum;

                res64.0[e.action.offset() as usize] = tmp;
            }

            for i in 0..64 {
                res.0[i] = (res64.0[i] / ntsum) as f32;
            }

            res.1 = wsum / (nsum as f32);
        }

        res
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