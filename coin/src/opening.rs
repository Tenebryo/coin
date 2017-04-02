use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::Result;


use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;


///
///Opening book file format:
///(16 bytes: hash table array size (n))
///n * (20 bytes: (1 byte used)(16 bytes: board position, normalized)(1 byte: white best move)(1 byte: black best move))
///
pub struct Opening {
    file    : File,
}

impl Opening {
    pub fn new(filename : String) -> Result<Opening> {
        let open_file = File::open(&filename)?;
        Ok(Opening {
            file    : open_file,
        })
    }
    
    pub fn get_move(&mut self, bb : Board, t : Turn) -> Result<Move> {
        Ok(Move::null())
    }
}
