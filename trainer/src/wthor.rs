
/// This file deals with loading information from WTHOR format game databases.

use std::io::prelude::*;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Result;
use std::fs::File;
use std::path::Path;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;

const RS_OFFSET : usize = 6;
const TS_OFFSET : usize = 7;
const MV_OFFSET : usize = 8;
const MV_LENGTH : usize = 60;

pub struct Game {
    moves   : Vec<Move>,
    score   : i32,
}

impl Game {
    pub fn new(wthor_raw : &[u8]) -> Game {

        let mut moves = vec![];

        for m in 0..60 {
            let x = (wthor_raw[MV_OFFSET + m] % 10)-1;
            let y = (wthor_raw[MV_OFFSET + m] / 10)-1;

            moves.push(Move::new(x,y));
        }

        let mut g = Game {
            moves :     moves,
            score :     0,
        };

        //find the ending score of the game. (WTHOR stores a weird, rather 
        //useless value for the score).
        let end = g.get_nth_board(60);
        g.score = end.count_pieces(Turn::BLACK) - end.count_pieces(Turn::WHITE);

        g
    }

    pub fn load_wthor_database(database : Path) -> Result<Vec<Game>> {

        let mut file = File::open(&database);

        file.seek(SeekFrom::Start(16))?;

        let mut games = vec![];

        let n = 16;

        Ok(games)
    }

    pub fn get_nth_board(&self, n : usize) -> Board {
        let mut bb = Board::new();
        let mut t = Turn::BLACK;

        let mut counter = 0;
        let mut i = 0;
        while counter < n {
            if bb.has_moves(t) {
                bb.do_move(self.moves[i], t);
                i += 1;
            }
            counter += 1;
            t = !t;
        }

        return bb;
    }
}
