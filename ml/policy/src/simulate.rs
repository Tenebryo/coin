use rand;
use rand::Rng;

use std::error::Error;

use bitboard::Board;
use bitboard::Move;
use bitboard::empty_movelist;
use bitboard::MAX_MOVES;

use policy::Policy;

pub struct Game {
    pub positions   :   Vec<Board>,
    pub moves       :   Vec<Move>,
    pub len         :   usize,
    pub result      :   i32,
}

impl Game {
    fn new() -> Game {
        Game {
            positions : vec![],
            moves : vec![],
            len     : 0,
            result: 0,
        }
    }

    fn add_position(&mut self, b : Board, m : Move) {
        self.len += 1;
        self.positions.push(b);
        self.moves.push(m);
    }
}

pub fn play_game(po : &mut Policy) -> Result<Game, Box<Error>> {
    let mut game = Game::new();
    let mut r = 1i32;

    let mut b = Board::new();

    let mut mvs = empty_movelist();
    let mut ord = [(0.0, 0); MAX_MOVES];
    let mut pwt = [0.0; 64];

    while !b.is_done() {
        po.eval(b, &mut pwt);

        let n = b.get_moves(&mut mvs) as usize;

        let mut m = mvs[0];
        let mut h = -10.0;
        for i in 0..n {
            let t = pwt[mvs[i].offset() as usize];
            ord[i] = (t, i);
            if t > h { h = t; m = mvs[i]; }
        }

        game.add_position(b, m);

        b.f_do_move(m);

        r = -r;
    }

    
    game.result = r * {use std::i32; let (p,o) = b.count_pieces(); (p as i32 - o as i32).signum()};

    Ok(game)
}

pub fn play_game_set(po : &mut Policy, n : usize) -> Result<Vec<Game>, Box<Error>> {
    Ok((0..n).map(|_| play_game(po).unwrap() ).collect::<Vec<_>>())
}