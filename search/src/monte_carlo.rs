use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use std::rc::Weak;

type Position = (u64, u64, Turn);

enum NodeState {
    UNEXPLORED,
    EXPANDABLE,
    COMPLETE,
    PROVEN_WIN,
    PROVEN_DRAW,
    PROVEN_LOSS,
    INVALID,
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
            + EXPLORATION * ((n as f32).log()
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

    fn select_node(&mut self, out_tree : &mut [usize; 60], i : usize) -> usize {
        match self.state {
            NodeState::EXPANDABLE => {

            },
            NodeState::COMPLETE => {

            },
            _ => {
                panic!();
            }
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