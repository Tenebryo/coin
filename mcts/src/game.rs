
use std::io::prelude::*;
use std::io::Result;
use std::fs::File;
use std::path::Path;
use std::io;

use bitboard::*;
use eval::*;

const RS_OFFSET : usize = 6;
const TS_OFFSET : usize = 7;
const MV_OFFSET : usize = 8;
const MV_LENGTH : usize = 60;
const GAME_LENGTH : usize = 68;
const FILE_OFFSET : usize = 16;

#[derive(Clone, Serialize, Deserialize)]
pub struct Game {
    pub states  : Vec<(EvalInput, EvalOutput)>,
    pub moves   : Vec<Move>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            states  : vec![],
            moves   : vec![],
        }
    }

    pub fn from_wthor_raw(wthor_raw : &[u8]) -> Result<Game> {


        let mut b = Board::new();
        let mut g = Game::new();

        let mut turn = 1;
        for m in 0..60 {
            // stop when the game is done
            if b.is_done() {
                break;
            }
            // account for passes
            if !b.has_move().0 {
                b.f_do_move(Move::pass());
                turn = -turn;
            }

            //get the move
            let x = (wthor_raw[MV_OFFSET + m] % 10)-1;
            let y = (wthor_raw[MV_OFFSET + m] / 10)-1;

            let m = Move::new(x,y);

            //make sure the move is always valid
            if !(x <= 7 && y <= 7) {return Err(io::Error::new(io::ErrorKind::Other, "Move Out of Bounds"));};
            if !b.check_move(m).0 {return Err(io::Error::new(io::ErrorKind::Other, "Invalid Move"));};

            // add the move to the game

            let mut prior = [0.0;64];
            prior[m.offset() as usize] = 1.0;
            g.add_position(b, EvalOutput(prior, 0.0));
            g.add_move(m);

            b.f_do_move(m);
        }

        //find the ending score of the game. (WTHOR stores a weird, rather 
        //useless value for the score).
        let result = (b.piece_diff() as f32 / 64.0);

        g.set_result(-result);

        Ok(g)
    }

    pub fn add_position(&mut self, input : EvalInput, mut output : EvalOutput) {

        //convert to 1 hot encoding:
        let mut mmax = output.0[0];
        let mut imax = 0;
        for i in 1..(output.0.len()) {
            if mmax < output.0[i] {
                mmax = output.0[i];
                imax = i;
            }
        }

        for i in 0..(output.0.len()) {
            output.0[i] = if i == imax {
                1.0
            } else {
                0.0
            };
        }

        self.states.push((input, output));
    }

    pub fn add_move(&mut self, m : Move) {
        self.moves.push(m);
    }

    pub fn set_result(&mut self, mut result : f32) {
        for i in (0..(self.states.len())).rev() {
            (self.states[i].1).1 = result;
            result = -result;
        }
    }
}

pub fn load_wthor_database(database : &Path) -> Result<Vec<Game>> {
    let mut file = File::open(database)?;

    let mut games = vec![];
    let mut buf = vec![];

    file.read_to_end(&mut buf)?;

    for i in 0..((buf.len()-FILE_OFFSET)/GAME_LENGTH) {
        let i = GAME_LENGTH * i + FILE_OFFSET;
        games.push(Game::from_wthor_raw(&buf[i..(i+GAME_LENGTH)])?);
    }

    Ok(games)
}

#[test]
fn saved_games_test() {

    use bincode;

    let mut contents = vec![];

    File::open(&Path::new("./data/iter000/new_games.dat")).unwrap()
        .read_to_end(&mut contents).unwrap();

    let mut decoded : Vec<Game> = bincode::deserialize(&contents[..]).unwrap();

    for g in decoded {
        eprintln!("State: {}", (g.states[0].1).1);
    }
}