use bitboard::Board;
use bitboard::Position;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use std::rc::Weak;
use std::time::Instant;
use std::time::Duration;
use std::thread;

use rand;
use rand::Rng;

use rayon::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum NodeState {
    INVALID,        // not a valid node.
    PROVEN_WIN,     // there is at least one child that is a proven loss for the other player
    PROVEN_DRAW,    // current player can force a draw
    EXPLORED,       // already expanded
    UNEXPLORED,     // hasn't been expanded yet
    PROVEN_LOSS,    // every move leads to a loss for the current player
}

const EXPLORATION : f32 = 4.0;
const SIMULATIONS_PER_NODE : i32 = 1064 * 2;

/// Represents a node in the game tree that has been or is to be explored
#[derive(Clone, PartialEq)]
struct SearchTreeNode {
    children    : Vec<SearchTreeNode>,
    position    : Position,
    state       : NodeState,
    results     : i32,
    simulations : i32,
    child_count : u32,
    prev_move   : Move, 
}

impl SearchTreeNode {

    /// Creates a new SearchTreeNode at the given position from the board
    pub fn new(b : Board, pm : Move) -> SearchTreeNode {
        let pieces = b.pieces();

        let state = if b.is_done() {
            let c = b.count_pieces();
            if c.0 > c.1 {
                NodeState::PROVEN_WIN
            } else if c.0 == c.1 {
                NodeState::PROVEN_DRAW
            } else {
                NodeState::PROVEN_LOSS
            }
        } else {
            NodeState::UNEXPLORED
        };
        // TODO: add heuristic value,
        SearchTreeNode {
            children    : vec![],
            position    : Position::from_board(b),
            state       : state,
            results     : 0,
            simulations : 1,
            child_count : 0,
            prev_move   : pm,
        }
    }

    /// Construct an empty node that hasn't been initialized yet
    pub fn empty() -> SearchTreeNode {
        // TODO: add heuristic value,
        SearchTreeNode {
            children    : vec![],
            position    : Position::from_board(Board::new()),
            state       : NodeState::INVALID,
            results     : 0,
            simulations : 1,
            child_count : 0,
            prev_move   : Move::null(),
        }
    }

    /// Get a Board representation of the Position this node
    fn get_board(&self) -> Board {
        self.position.to_board()
    }

    /// calculate the score from the statistics on this node and the total number of simulations
    fn get_score(&self, n : i32) -> f32 {
        let r = self.results as f32;
        let s = self.simulations as f32;

        (r / s) + EXPLORATION * ((n as f32).log(2.718281828459045) / s).sqrt()
    }

    /// Expand this node, simulate the child nodes, and update this node.
    fn expand(&mut self) -> (i32, i32, u32) {
        self.state = NodeState::EXPLORED;
        let b = self.get_board();

        let mut ms = empty_movelist();
        let n = b.get_moves(&mut ms) as usize;

        self.children.reserve_exact(n);

        //run some simulations in parallel.
        let sims = ms[0..n].par_iter().map( |&m| {
            let mut bc = b.copy();
            bc.f_do_move(m);

            let mut r = 0;

            //simulate 1000 games
            let mut s = 1000;
            while s != 0 {
                r += fast_simulate(bc.copy());
                s -= 1;
            }
            
            let mut ch = SearchTreeNode::new(bc, m);
            ch.add_runs(r, SIMULATIONS_PER_NODE);

            (r, SIMULATIONS_PER_NODE, ch)
        });

        let mut tr = 0;
        let mut ts = 0;

        for (rs, ss, chs) in sims.collect::<Vec<_>>() {
            tr += rs;
            ts += ss;
            self.children.push(chs);
        }

        let c = self.children.len() as u32;
        self.add_children(c);

        self.add_runs(tr, ts);
        (tr, ts, c)
    }

    /// Select the most promising node in the explored game tree and expand it
    fn select_node(&mut self, sims : i32) -> (i32, i32, u32) {
        //find best child to expand.
        //ignore solved (propagated later)
        if NodeState::UNEXPLORED == self.state {
            let (r, s, c) = self.expand();

            return (-r, s, c);
        }

        use std::f32;
        let mut b = 0.0;
        let mut bi : i32 = -1;
        //find the subnode with the best score
        for i in 0..(self.children.len()) {
            match self.children[i].state {
                NodeState::EXPLORED | NodeState::UNEXPLORED => {
                    let score = self.children[i].get_score(sims);
                    if bi == -1 || b < score {
                        bi = i as i32;
                        b = score;
                    }
                },
                _ => {}
            }
        }

        if bi != -1 {
            
            // update results and simulations
            let (r, s, c) = self.children[bi as usize].select_node(sims);
            self.add_runs(r, s);
            self.add_children(c);

            self.update_proven_state();

            return (-r, s, c);
        } else {
            self.update_proven_state();
            return (0,0,0);
            
            panic!()
        }
    }

    /// Check children, determine if this node has a proven state, and update the state
    fn update_proven_state(&mut self) {
        let mut best_state = NodeState::INVALID;
        for i in 0..(self.children.len()) {
            if best_state < self.children[i].state {
                best_state = self.children[i].state;
            }
        }

        match best_state {
            NodeState::PROVEN_LOSS => {
                // if there is a proven loss for the opponent, this move is a proven win
                self.state = NodeState::PROVEN_WIN;
            },
            NodeState::PROVEN_DRAW => {
                // if the best is a proven draw (this implies that there are no unproven branches left)
                // then this node is also a proven draw
                self.state = NodeState::PROVEN_DRAW;
            },
            NodeState::PROVEN_WIN => {
                // if the best state is a proven win, then this node is a proven loss for the current player
                self.state = NodeState::PROVEN_LOSS;
            },
            //no other states matter:
            _ => {}
        }
    }

    /// Updates the number of results and simulations in the node and subnodes
    fn add_runs(&mut self, r : i32, s : i32) {
        self.results += r;
        self.simulations += s;
    }

    /// Updates the number of children in the node and subnodes
    fn add_children(&mut self, c : u32) {
        self.child_count += c;
    }

    /// Does an iteration of Monte Carlo Tree search
    fn search(&mut self) {
        let s = self.simulations;
        let mut tmp = [0; 60];
        self.select_node(s);

        // for i in tmp[0..60].iter() {
        //     eprint!("{} ", i);
        // }
        // eprintln!("");
    }
}

fn fast_simulate(mut b : Board) -> i32 {
    use std::i32;

    let mut mvs = empty_movelist();
    let mut e = b.total_empty();
    let mut r = rand::thread_rng();
    let mut c = 1;

    while e > 0 {
        let n = b.get_moves(&mut mvs);
        b.f_do_move(mvs[(r.gen::<u8>() % n) as usize]);
        e -= 1;
        c = -c;
    }
    while !b.is_done() {
        let n = b.get_moves(&mut mvs);
        b.f_do_move(mvs[(r.gen::<u8>() % n) as usize]);
        c = -c;
    }

    let p = b.count_pieces();
    (c * (p.0 as i32 - p.1 as i32).signum())
}


/// Struct used to keep track of the searched game tree and reuse it, etc.
pub struct MonteCarloSearch {
    root_node   : SearchTreeNode,
}

impl MonteCarloSearch {
    /// Creates a new Monte Carlo Tree search engine at the root node of an
    /// Othello game.
    pub fn new() -> MonteCarloSearch {
        MonteCarloSearch {
            root_node: SearchTreeNode::new(Board::new(), Move::null()),
        }
    }

    /// Prunes the monte Carlo Search Tree branches that are no longer relevant
    /// i.e. ones that represent moves not played.
    pub fn prune(&mut self, i : usize) {
        use std::mem;

        let tmp = mem::replace(&mut self.root_node.children[i], 
                                SearchTreeNode::empty());

        self.root_node = tmp;
    }

    ///searches the
    pub fn search_for_millis(&mut self, b : Board, msleft : u64) -> Move {
        let start = Instant::now();

        //check if we already have information on this position
        let mut flag = true;
        for i in 0..(self.root_node.children.len()) {
            if self.root_node.children[i].position == Position::from_board(b) {
                self.prune(i);
                flag = false;

                eprintln!("[COIN]: Saved {} Game Tree Nodes and {} Simulations!",
                    self.root_node.child_count,
                    self.root_node.simulations
                );
                break;
            }
        }
        if flag {
            eprintln!("[COIN]: Discarded Previous Game Tree.");
            self.root_node = SearchTreeNode::new(b, Move::null());
        }

        eprintln!("[COIN]: Expanding Game Tree...");

        while start.elapsed() < Duration::from_millis(msleft) {
            self.root_node.search();

            match self.root_node.state {
                NodeState::PROVEN_DRAW | NodeState::PROVEN_WIN | NodeState::PROVEN_LOSS => {
                    eprintln!("[COIN]: Game Tree Solved!");
                    break;
                },
                _ => {}
            }
        }

        eprintln!("[COIN]: Searched {} Nodes with {} Simulations",
            self.root_node.child_count,
            self.root_node.simulations
        );

        let sims = self.root_node.simulations;

        let mut mvs = empty_movelist();
        let n = b.get_moves(&mut mvs) as usize;

        let mut bi = 0;
        let mut bs = self.root_node.children[0].simulations;

        for i in 1..n {
            let sc = self.root_node.children[i].simulations;

            eprintln!("{}, {}, {}, {:?}", 
                self.root_node.children[i].prev_move,
                self.root_node.children[i].get_score(sims),
                self.root_node.children[i].simulations,
                self.root_node.children[i].state
            );
            if sc > bs {
                bi = i;
                bs = sc;
            }
        }

        eprintln!("[COIN]: Best Move: {}", mvs[bi]);

        self.prune(bi);

        eprintln!("[COIN]: {}/{} ({:.2})",
            self.root_node.results,
            self.root_node.simulations,
            self.root_node.results as f32 / self.root_node.simulations as f32
        );

        mvs[bi]
    }
}