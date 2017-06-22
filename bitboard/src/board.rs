use std::fmt;
use bit_ops::*;
use std::ops::Not;

pub const MAX_MOVES : usize = 28;

pub type MoveList = [Move; MAX_MOVES];
pub type MoveOrder = [(i32, usize); MAX_MOVES];

#[derive(Copy, Clone, PartialEq)]
pub enum Turn {
    BLACK,
    WHITE,
}

impl Not for Turn{
    type Output = Turn;
    fn not(self) -> Turn {
        match self {
            Turn::BLACK => Turn::WHITE,
            Turn::WHITE => Turn::BLACK,
        }
    }
}

impl fmt::Display for Turn {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Turn::BLACK => write!(f, "BLACK"),
            &Turn::WHITE => write!(f, "WHITE"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    data    : u8,
}

impl Move {
    pub fn new(x : u8, y : u8) -> Move {
        Move {
            data    : (x & 0b111) | ((y & 0b111) << 3),
        }
    }
    
    pub fn pass() -> Move {
        Move {
            data    : 0b1_000_000,
        }
    }
    
    pub fn null() -> Move {
        Move {
            data    : 0b10_000_000,
        }
    }
    
    pub fn x(&self) -> u8 {
        self.data & 0b111
    }
    
    pub fn y(&self) -> u8 {
        (self.data >> 3) & 0b111
    }
    
    pub fn is_pass(&self) -> bool {
        (self.data & 0b1_000_000) != 0
    }
    
    pub fn is_null(&self) -> bool {
        (self.data & 0b10_000_000) != 0
    }
    
    pub fn mask(&self) -> u64 {
        match self.data {
            0b1_000_000 => 0,
            d => 1u64 << (d)
        }
    }

    pub fn offset(&self) -> u8 {
        self.data & 0b111_111
    }
    
    fn from_off(off : u8) -> Move {
        Move {
            data    : off & 0b111_111,
        }
    }
}

impl fmt::Display for Move {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_pass() {
            write!(f, "(PASS)")
        } else if self.is_null() {
            write!(f, "(NULL)")
        } else {
            write!(f, "({},{})", self.x(), self.y())
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub struct Board {
    black   : u64,
    white   : u64,
    bmove   : u64,
    wmove   : u64,
}

impl Board {

    /// Returns a new othello board initialized to the staring position.
    pub fn new() -> Board {
        Board {
            black   : 0b00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000u64,
            white   : 0b00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000u64,
            bmove   : 0b00000000_00000000_00010000_00100000_00000100_00001000_00000000_00000000u64,
            wmove   : 0b00000000_00000000_00001000_00000100_00100000_00010000_00000000_00000000u64,
        }
    }
    
    
    /// Returns a copy of the board
    pub fn copy(&self) -> Board {
        Board {
            black   : self.black,
            white   : self.white,
            bmove   : self.bmove,
            wmove   : self.wmove,
        }
    }
    
    
    /// Checks whether or not the game is over
    pub fn is_done(&self) -> bool {
        self.bmove == 0 && self.wmove == 0
    }
    
    
    /// Checks whether a player has a legal move
    pub fn has_move(&self, t : Turn) -> bool {
        match t {
            Turn::BLACK => self.bmove != 0,
            Turn::WHITE => self.wmove != 0,
        }
    }
    
    
    /// Checks whether a move is legal
    pub fn check_move(&self, t : Turn, m : Move) -> bool {
        match t {
            Turn::BLACK => (self.bmove & m.mask()) != 0,
            Turn::WHITE => (self.wmove & m.mask()) != 0,
        }
    }
    
    
    /// Updates a board by playing the given move for the given player
    pub fn do_move(&mut self, t : Turn, m : Move) -> u64{
    
        if m.is_pass() || m.is_null() {
            return 0;
        }
        let mut pro = 0u64; 
        let mut gen = 0u64;
        let mut msk = 0u64;
        let org = m.mask();

        match t {
            Turn::BLACK => {
                gen = self.black;
                pro = self.white;
                
                self.black |= org;
            },
            Turn::WHITE => {
                gen = self.white;
                pro = self.black;
                
                self.white |= org;
            }
        }

        msk |= sout_occl(gen, pro) & nort_occl(org, pro);
        msk |= nort_occl(gen, pro) & sout_occl(org, pro);
        msk |= east_occl(gen, pro) & west_occl(org, pro);
        msk |= west_occl(gen, pro) & east_occl(org, pro);
        msk |= sowe_occl(gen, pro) & noea_occl(org, pro);
        msk |= noea_occl(gen, pro) & sowe_occl(org, pro);
        msk |= soea_occl(gen, pro) & nowe_occl(org, pro);
        msk |= nowe_occl(gen, pro) & soea_occl(org, pro);

        self.black ^= msk;
        self.white ^= msk;

        self.update_moves_fast();

        msk
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] 
    pub fn f_do_move(&mut self, t : Turn, m : Move) -> u64 {

        use do_moves_fast::fast_do_move;

        if m.is_pass() || m.is_null() {
            return 0;
        }
        let mut pro = 0u64; 
        let mut gen = 0u64;

        match t {
            Turn::BLACK => {
                gen = self.black;
                pro = self.white;
            },
            Turn::WHITE => {
                gen = self.white;
                pro = self.black;
            }
        }

        let flipped = fast_do_move(m.x(), m.y(), gen, pro);

        self.black ^= flipped;
        self.white ^= flipped;

        match t {
            Turn::BLACK => {
                self.white ^= m.mask();
            },
            Turn::WHITE => {
                self.black ^= m.mask();
            }
        }

        self.update_moves_fast();
        flipped
    }
    
    
    /// Returns the bit mask of the given player's pieces
    pub fn pieces(&self, t : Turn) -> u64 {
        match t {
            Turn::BLACK => self.black,
            Turn::WHITE => self.white,
        }
    }
    
    
    /// Returns the mobility bit mask of the given player
    pub fn mobility(&self, t : Turn) -> u64 {
        match t {
            Turn::BLACK => self.bmove,
            Turn::WHITE => self.wmove,
        }
    }
    
    
    /// Returns the stability bit mask of the given player
    pub fn stability(&self, t : Turn) -> u64 {
        //sides
        const top : u64 = 255u64;
        const bot : u64 = 18374686479671623680u64;
        const lft : u64 = 72340172838076673u64;
        const rht : u64 = 9259542123273814144u64;
        
        let gen = match t {
            Turn::BLACK => self.black,
            Turn::WHITE => self.white
        };
        let pcs = self.black|self.white;

        let vrt = nort_occl(bot & pcs, pcs) & sout_occl(top & pcs, pcs);
        let hrz = east_occl(lft & pcs, pcs) & west_occl(rht & pcs, pcs);
        let dg1 = noea_occl((bot|lft) & pcs, pcs) & sowe_occl((top|rht) & pcs, pcs);
        let dg2 = nowe_occl((bot|rht) & pcs, pcs) & soea_occl((top|lft) & pcs, pcs);

        let mut stb = (0x8100000000000081u64 | (vrt & hrz & dg1 & dg2)) & gen;

        //expand stable areas. At most 16 iterations necessary to reach from one
        //corner to the other
        for _ in 0..16 {
            stb |= gen & (
                (nort_one(stb) | sout_one(stb) | vrt) &
                (east_one(stb) | west_one(stb) | hrz) &
                (noea_one(stb) | sowe_one(stb) | dg1) &
                (nowe_one(stb) | soea_one(stb) | dg2)
            );
        }

        stb
    }
    
    
    /// Counts the number of pieces a player has on the board.
    pub fn count_pieces(&self, t : Turn) -> u8 {
        match t {
            Turn::BLACK => popcount_64(self.black),
            Turn::WHITE => popcount_64(self.white),
        }
    }
    
    
    /// Gets the moves available to a given player and stores them in the
    /// array that is passed as an argument. The number of moves is returned.
    pub fn get_moves(&self, t : Turn, out_moves : &mut MoveList) -> u8 {
    
        if !self.has_move(t) {
            out_moves[0] = Move::pass();
            return 1;
        }
    
        let mut mvs = match t {
            Turn::BLACK => self.bmove,
            Turn::WHITE => self.wmove,
        };
        
        let n = popcount_64(mvs);
        
        for i in 0..n {
            out_moves[i as usize] = Move::from_off(bitscan_64(mvs));
            mvs ^= out_moves[i as usize].mask();
        }
        
        n as u8
    }
    
    /// Writes moves representing the current empty squares on the board to
    /// the out parameter `out_moves`. Note that this means the provided array
    /// must be large enough. Returns the number of empty squares found.
    pub fn get_empty(&self, out_moves : &mut [Move]) -> u8 {
        
        let mut mvs = !(self.black | self.white);
        if mvs == 0 {
            return 0;
        }
        
        let n = popcount_64(mvs);
        
        for i in 0..n {
            out_moves[i as usize] = Move::from_off(bitscan_64(mvs));
            mvs ^= out_moves[i as usize].mask();
        }
        
        n as u8
    }
    
    
    /// Returns the index of a valid move for the given player in the move array
    pub fn get_move_index(&self, t : Turn, m : Move) -> usize {
        popcount_64(self.mobility(t) & (m.mask()-1)) as usize
    }
    
    
    // Internal to the Board struct, finds and updates the moves for the given
    // player.
    fn find_moves(&mut self, t : Turn) {
        let mut moves = 0;
        let empty = !(self.black | self.white);
        let mut tmp = 0;
        
        let gen = match t {
            Turn::BLACK => self.black,
            Turn::WHITE => self.white
        };
        let pro = match t {
            Turn::BLACK => self.white,
            Turn::WHITE => self.black
        };

        tmp = sout_one(sout_occl(gen, pro) & pro);
        moves |= tmp & empty;

        tmp = nort_one(nort_occl(gen, pro) & pro);
        moves |= tmp & empty;

        tmp = east_one(east_occl(gen, pro) & pro);
        moves |= tmp & empty;

        tmp = west_one(west_occl(gen, pro) & pro);
        moves |= tmp & empty;

        tmp = soea_one(soea_occl(gen, pro) & pro);
        moves |= tmp & empty;

        tmp = sowe_one(sowe_occl(gen, pro) & pro);
        moves |= tmp & empty;

        tmp = noea_one(noea_occl(gen, pro) & pro);
        moves |= tmp & empty;

        tmp = nowe_one(nowe_occl(gen, pro) & pro);
        moves |= tmp & empty;

        match t {
            Turn::BLACK => {self.bmove = moves;},
            Turn::WHITE => {self.wmove = moves;}
        };
    }

    ///This function makes sure the move bitboards are current in the function
    pub fn update_moves(&mut self) {
        self.find_moves(Turn::BLACK);
        self.find_moves(Turn::WHITE);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]    
    pub fn update_moves_fast(&mut self) {
        use find_moves_fast::fast_find_moves;
        self.bmove = fast_find_moves(self.black, self.white);
        self.wmove = fast_find_moves(self.white, self.black);
    }
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut err = write!(f, "  A B C D E F G H | A B C D E F G H | A B C D E F G H\n");
        
        for y in 0..8 {
            let mut t = err.and(write!(f, "{}", y+1));
            err = t;
            for x in 0..8 {
                let m = Move::new(x,y).mask();
                let e = err.and(
                    if self.black & m != 0 {
                        write!(f, " @")
                    } else if self.white & m != 0 {
                        write!(f, " O")
                    } else {
                        write!(f, "  ")
                    }
                );
                err = e;
            }
            
            t = err.and(write!(f, " |"));
            
            for x in 0..8 {
                let m = Move::new(x,y).mask();
                let e = err.and(
                    if self.bmove & m != 0 {
                        write!(f, " *")
                    } else {
                        write!(f, "  ")
                    }
                );
                err = e;
            }
            
            t = err.and(write!(f, " |"));
            
            for x in 0..8 {
                let m = Move::new(x,y).mask();
                let e = err.and(
                    if self.wmove & m != 0 {
                        write!(f, " *")
                    } else {
                        write!(f, "  ")
                    }
                );
                err = e;
            }
            t = err.and(write!(f, "|"));
            
            for x in 0..8 {
                let m = Move::new(x,y).mask();
                let e = err.and(
                    if self.black & m != 0 {
                        write!(f, " *")
                    } else {
                        write!(f, "  ")
                    }
                );
                err = e;
            }
            t = err.and(write!(f, "|"));
            
            for x in 0..8 {
                let m = Move::new(x,y).mask();
                let e = err.and(
                    if self.white & m != 0 {
                        write!(f, " *")
                    } else {
                        write!(f, "  ")
                    }
                );
                err = e;
            }
            t = err.and(write!(f, "\n"));
            err = t;
        }
        
        err
    }
}



















