use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use std::rc::Weak;

type Position = (u64, u64, Turn);

enum NodeState {
    UNEXPLORED,     // hasn't been expanded yet
    EXPLORED,       // already expanded
    PROVEN_WIN,     // there is at least one child that is a proven loss for the other player
    PROVEN_DRAW,    // current player can force a draw
    PROVEN_LOSS,    // every move leads to a loss for the current player
    INVALID,        // not a valid node.
}

const EXPLORATION : f32 = 1.0;

#[derive(Copy, Clone, PartialEq)]
struct SearchTreeNode {
    children    : Vec<SearchTreeNode>,
    position    : Position,
    state       : NodeState,
    results     : i32,
    simulations : i32,
    score       : f32,
    max_subscore: f32,
}

impl SearchTreeNode {
    pub fn new(b : Board) -> SearchTreeNode {
        let pieces = b.pieces();
        // TODO: add heuristic value,
        SearchTreeNode {
            children    : vec![],
            position    : (pieces.0, pieces.1, b.get_turn()),
            state       : NodeState::UNEXPLORED,
            results     : 0,
            simulations : 1,
            score       : 0.0,
            max_subscore: 0.0, 
        }
    }

    pub fn empty() -> SearchTreeNode {
        // TODO: add heuristic value,
        SearchTreeNode {
            children    : vec![],
            position    : (0, 0, Turn::BLACK),
            state       : NodeState::INVALID,
            results     : 0,
            simulations : 1,
            score       : 0.0,
            hr_score    : 0.0, 
        }
    }

    fn get_board(&self) -> Board {
        Board::position(self.position.0, self.position.1, self.position.2)
    }

    fn get_score(&self, n : i32) -> f32 {
        (self.results as f32 / self.simulations  as f32)
            + EXPLORATION * ((n as f32).log()/self.simulations).sqrt()
    }

    fn expand(&mut self) {
        let b = self.get_board();

        let ms = empty_movelist();
        let n = b.get_moves(&mut ms);

        self.children.reserve_exact(n);
        for i in 0..n {
            let mut bc = b.copy();
            bc.f_do_move(ms[i]);

            self.children.push(SearchTreeNode::new(bc));
        }
    }

    fn select_node(&mut self, out_tree : &mut [usize; 60], idx : usize) -> usize {
        //find best child to expand.
        //ignore solved (propagated later)
        if NodeState::UNEXPLORED = self.state {
            return idx;
        }

        use std::f32;
        let mut b = 0.0;
        let mut bi = -1;
        //find the subnode with the best score
        for i in 0..(self.children.size()) {
            match self.children[i].state {
                NodeState::EXPLORED | NodeState::UNEXPLORED => {
                    let score = self.children[i].score;
                    if bi == -1 || b < score {
                        bi = i;
                        b = score;
                    }
                },
                _ => {}
            }
        }

        if bi != -1 {
            out_tree[idx] = bi;
            return self.children[bi].select_node(out_tree, idx+1);
        } else {
            panic!()
        }
    }
}

pub struct MonteCarloSearch {
    root_node   : SearchTreeNode,
}

impl MonteCarloSearch {
    /// Creates a new Monte Carlo Tree search engine at the root node of an
    /// Othello game.
    pub fn new() -> MonteCarloSearch {
        MonteCarloSearch {
            root_node: SearchTreeNode::new(Board::new()),
        }
    }

    /// Prunes the monte Carlo Search Tree branches that are no longer relevant
    /// i.e. ones that represent moves not played.
    pub fn play_move(&mut self, m : Move) {
        use std::mem;
        let mut board = self.root_node.get_board();
        let i = board.get_move_index(m);

        let tmp = mem::replace(&mut self.root_node.children[i], 
                                SearchTreeNode::empty());

        self.root_node = tmp;
    }

    ///Finds the SearchTreeNode with the 
    fn select_node(&mut self, out_tree : &mut [usize; 60]) -> usize {
        
    }
}