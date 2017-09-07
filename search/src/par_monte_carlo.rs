use bitboard::Board;
use bitboard::Position;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use std::rc::Weak;
use std::time::Instant;
use std::time::Duration;
use std::thread;

use std::sync::Mutex;
use std::sync::RwLock;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::AtomicUsize;


use rand;
use rand::Rng;

use rayon;
use rayon::prelude::*;

//fake enum is just integers
mod node_state {
    pub const INVALID : usize = 0;        // not a valid node.
    pub const PROVEN_WIN : usize = 1;     // there is at least one child that is a proven loss for the other player
    pub const PROVEN_DRAW : usize = 2;    // current player can force a draw
    pub const EXPLORED : usize = 3;       // already expanded
    pub const UNEXPLORED : usize = 4;     // hasn't been expanded yet
    pub const PROVEN_LOSS : usize = 5;    // every move leads to a loss for the current player
}

const EXPLORATION : f32 = 3.0;
const SIMULATIONS_PER_NODE : isize = 1064/4;

/// Represents a node in the game tree that has been or is to be explored
struct ParSearchTreeNode {
    children    : RwLock<Vec<ParSearchTreeNode>>,
    position    : Position,
    state       : AtomicUsize,
    results     : AtomicIsize,
    simulations : AtomicIsize,
    child_count : AtomicUsize,
    prev_move   : Move,
}

impl ParSearchTreeNode {

    /// Creates a new SearchTreeNode at the given position from the board
    pub fn new(b : Board, pm : Move) -> ParSearchTreeNode {
        let pieces = b.pieces();

        let state = if b.is_done() {
            let c = b.count_pieces();
            if c.0 > c.1 {
                node_state::PROVEN_WIN
            } else if c.0 == c.1 {
                node_state::PROVEN_DRAW
            } else {
                node_state::PROVEN_LOSS
            }
        } else {
            node_state::UNEXPLORED
        };
        // TODO: add heuristic value,
        ParSearchTreeNode {
            children    : RwLock::new(vec![]),
            position    : Position::from_board(b),
            state       : AtomicUsize::new(state),
            results     : AtomicIsize::new(0),
            simulations : AtomicIsize::new(1),
            child_count : AtomicUsize::new(0),
            prev_move   : pm,
        }
    }

    /// Creates an empty SearchTreeNode
    pub fn empty() -> ParSearchTreeNode {
        ParSearchTreeNode {
            children    : RwLock::new(vec![]),
            position    : Position::from_board(Board::new()),
            state       : AtomicUsize::new(node_state::INVALID),
            results     : AtomicIsize::new(0),
            simulations : AtomicIsize::new(1),
            child_count : AtomicUsize::new(0),
            prev_move   : Move::null(),
        }
    }

    /// Get a Board representation of the Position this node
    fn get_board(&self) -> Board {
        self.position.to_board()
    }

    /// calculate the score from the statistics on this node and the total number of simulations
    fn get_score(&self, n : isize) -> f32 {
        let r = self.results.load(Ordering::SeqCst) as f32;
        let s = self.simulations.load(Ordering::SeqCst) as f32;

        (r / s) + EXPLORATION * ((n as f32).log(2.718281828459045) / s).sqrt()
    }

    /// Expand this node, simulate the child nodes, and update this node.
    fn expand(&self) -> (isize, isize, usize) {
        self.state.store(node_state::EXPLORED, Ordering::SeqCst);
        let b = self.get_board();

        let mut ms = empty_movelist();
        let n = b.get_moves(&mut ms) as usize;


        //run some simulations in parallel.
        let sims = ms[0..n].par_iter().map( |&m| {
            let mut bc = b.copy();
            bc.f_do_move(m);

            let mut r = 0;

            //simulate 1000 games
            let mut s = SIMULATIONS_PER_NODE;
            while s != 0 {
                r += fast_simulate(bc.copy());
                s -= 1;
            }
            
            let mut ch = ParSearchTreeNode::new(bc, m);
            ch.add_runs(r, SIMULATIONS_PER_NODE);

            (r, SIMULATIONS_PER_NODE, ch)
        }).collect::<Vec<_>>();

        let mut tr = 0;
        let mut ts = 0;

        {//get lock for children
            let mut children = self.children.write().unwrap();
            children.reserve_exact(n);
            for (rs, ss, chs) in sims {
                tr += rs;
                ts += ss;
                children.push(chs);
            }

        }
        let c = n;//self.children.len() as u32;

        self.add_children(c);
        self.add_runs(tr, ts);

        (tr, ts, c)
    }

    /// Select the most promising node in the explored game tree and expand it
    fn select_node(&self, sims : isize) -> (isize, isize, usize) {
        //find best child to expand.
        //ignore solved (propagated later)
        if node_state::UNEXPLORED == self.state.load(Ordering::SeqCst) {
            let (r, s, c) = self.expand();

            return (-r, s, c);
        }

        use std::f32;
        let mut b = 0.0;
        let mut bi : isize = -1;
        //find the subnode with the best score
        {
            let children = self.children.read().unwrap();
            for i in 0..(children.len()) {
                match children[i].state.load(Ordering::SeqCst) {
                    node_state::EXPLORED | node_state::UNEXPLORED => {
                        let score = children[i].get_score(sims);
                        if bi == -1 || b < score {
                            bi = i as isize;
                            b = score;
                        }
                    },
                    _ => {}
                }
            }
        }

        if bi != -1 {
            
            // update results and simulations
            let children = self.children.read().unwrap();
            let (r, s, c) = children[bi as usize].select_node(sims);
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
    fn update_proven_state(&self) {
        let mut best_state = node_state::INVALID;
        {
            let children = self.children.read().unwrap();
            for i in 0..(children.len()) {
                let child_state = children[i].state.load(Ordering::SeqCst);
                if best_state < child_state {
                    best_state = child_state;
                }
            }
        }

        match best_state {
            node_state::PROVEN_LOSS => {
                // if there is a proven loss for the opponent, this move is a proven win
                self.state.store(node_state::PROVEN_WIN, Ordering::SeqCst);
            },
            node_state::PROVEN_DRAW => {
                // if the best is a proven draw (this implies that there are no unproven branches left)
                // then this node is also a proven draw
                self.state.store(node_state::PROVEN_DRAW, Ordering::SeqCst);
            },
            node_state::PROVEN_WIN => {
                // if the best state is a proven win, then this node is a proven loss for the current player
                self.state.store(node_state::PROVEN_LOSS, Ordering::SeqCst);
            },
            //no other states matter:
            _ => {}
        }
    }

    /// Updates the number of results and simulations in the node and subnodes
    fn add_runs(&self, r : isize, s : isize) {
        self.results.fetch_add(r, Ordering::SeqCst);// += r;
        self.simulations.fetch_add(s, Ordering::SeqCst);// += s;
    }

    /// Updates the number of children in the node and subnodes
    fn add_children(&self, c : usize) {
        self.child_count.fetch_add(c, Ordering::SeqCst);// += c;
    }

    /// Does an iteration of Monte Carlo Tree search
    fn search(&self) {
        let s = self.simulations.load(Ordering::SeqCst);
        self.select_node(s);

    }
}

fn fast_simulate(mut b : Board) -> isize {
    use std::i32;

    let mut mvs = empty_movelist();
    let mut e = b.total_empty()+2;
    let mut r = rand::thread_rng();
    let mut c = 1;

    while e > 0 {
        let n = b.get_moves(&mut mvs);
        b.f_do_move(mvs[(r.gen::<u8>() % n) as usize]);
        e -= 1;
        c = -c;
    }

    let p = b.count_pieces();
    (c * (p.0 as isize - p.1 as isize).signum())
}


/// Struct used to keep track of the searched game tree and reuse it, etc.
pub struct ParMonteCarloSearch {
    root_node   : ParSearchTreeNode,
}

impl ParMonteCarloSearch {
    /// Creates a new Monte Carlo Tree search engine at the root node of an
    /// Othello game.
    pub fn new() -> ParMonteCarloSearch {
        ParMonteCarloSearch {
            root_node: ParSearchTreeNode::new(Board::new(), Move::null()),
        }
    }

    /// Prunes the monte Carlo Search Tree branches that are no longer relevant
    /// i.e. ones that represent moves not played.
    pub fn prune(&mut self, i : usize) {
        use std::mem;

        // let tmp = mem::replace(&mut self.root_node.children[i], 
        //                         ParSearchTreeNode::empty());

        // self.root_node = tmp;

        let old_root = mem::replace(&mut self.root_node, ParSearchTreeNode::empty());
        self.root_node = mem::replace(&mut old_root.children.into_inner().unwrap()[i], ParSearchTreeNode::empty());
    }

    ///searches the
    pub fn search_for_millis(&mut self, b : Board, msleft : u64) -> Move {
        let start = Instant::now();

        //check if we already have information on this position
        let mut flag = true;
        let mut prune_i = 0;
        {
            let children = self.root_node.children.read().unwrap();
            for i in 0..(children.len()) {
                if children[i].position == Position::from_board(b) {
                    prune_i = i;
                    flag = false;

                    eprintln!("[COIN]: Saved {:?} Game Tree Nodes and {:?} Simulations!",
                        self.root_node.child_count.load(Ordering::SeqCst),
                        self.root_node.simulations.load(Ordering::SeqCst)
                    );
                    break;
                }
            }
        }
        if flag {
            eprintln!("[COIN]: Discarded Previous Game Tree.");
            self.root_node = ParSearchTreeNode::new(b, Move::null());
        } else {
            self.prune(prune_i);
        }

        eprintln!("[COIN]: Expanding Game Tree...");

        let solved = AtomicBool::new(false);
        let agents = AtomicUsize::new(0);
        let pool = rayon::ThreadPool::new(
            rayon::Configuration::new().num_threads(8)
        ).unwrap();

        const NUM_AGENTS : usize = 8;

        let prev_nodes = self.root_node.child_count.load(Ordering::SeqCst);
        let prev_sims  = self.root_node.simulations.load(Ordering::SeqCst);

        while start.elapsed() < Duration::from_millis(msleft) && !solved.load(Ordering::SeqCst) {
            let a = agents.load(Ordering::SeqCst);
            for _ in a..NUM_AGENTS {
                agents.fetch_add(1, Ordering::SeqCst);

                pool.install( || {
                    self.root_node.search();
                    agents.fetch_sub(1, Ordering::SeqCst);
                } );
            }

            thread::sleep_ms(0);
            match self.root_node.state.load(Ordering::SeqCst) {
                node_state::PROVEN_DRAW=> {
                    eprintln!("[COIN]: Game Tree Solved: PROVEN_DRAW");
                    solved.store(true, Ordering::SeqCst);
                },
                node_state::PROVEN_WIN => {
                    eprintln!("[COIN]: Game Tree Solved: PROVEN_WIN");
                    solved.store(true, Ordering::SeqCst);
                },
                node_state::PROVEN_LOSS => {
                    eprintln!("[COIN]: Game Tree Solved: PROVEN_LOSS");
                    solved.store(true, Ordering::SeqCst);
                },
                _ => {}
            }
        }

        let new_nodes = self.root_node.child_count.load(Ordering::SeqCst) - prev_nodes;
        let new_sims  = self.root_node.simulations.load(Ordering::SeqCst) - prev_sims;

        eprintln!("[COIN]: Searched {:?} Nodes with {:?} Simulations", new_nodes, new_sims );
        
        let elapsed = {
            let d = start.elapsed();
            (d.as_secs() as f32) + (d.subsec_nanos() as f32)/1_000_000_000f32
        };
        eprintln!("[COIN]: Time elapsed: {} ({:.2} sims/s, {:.2} nodes/s)", elapsed, new_sims as f32 / elapsed, new_nodes as f32 / elapsed);

        let mut mvs = empty_movelist();
        let n = b.get_moves(&mut mvs) as usize;

        let mut bi = 0;
        let mut bs = 0;

        //check which node seems best
        {//get lock
            let children = self.root_node.children.read().unwrap();
            for i in 0..n {
                if children[i].state.load(Ordering::SeqCst) == node_state::PROVEN_LOSS {
                    //if the opponent has a proven loss, take it!
                    return mvs[i];
                }

                let sc = children[i].simulations.load(Ordering::SeqCst);

                if sc > bs {
                    bi = i;
                    bs = sc;
                }
            }
        }

        eprintln!("[COIN]: Best Move: {}", mvs[bi]);

        self.prune(bi);

        eprintln!("[COIN]: {:?}/{:?} ({:.2})",
            self.root_node.results.load(Ordering::SeqCst),
            self.root_node.simulations.load(Ordering::SeqCst),
            self.root_node.results.load(Ordering::SeqCst) as f32 / self.root_node.simulations.load(Ordering::SeqCst) as f32
        );

        mvs[bi]
    }
}