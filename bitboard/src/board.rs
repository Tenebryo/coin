use std::fmt;
use bit_ops::*;
use std::ops::Not;
use std::hash::*;

pub const MAX_MOVES : usize = 30;

pub type MoveList = [Move; MAX_MOVES];
pub type MoveOrder = [(i32, usize); MAX_MOVES];


#[derive(Copy, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Turn(bool);

impl Turn {
    pub const BLACK : Turn = Turn(true);
    pub const WHITE : Turn = Turn(false);
}

impl Not for Turn{
    type Output = Turn;
    fn not(self) -> Turn {
        Turn(!self.0)
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

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
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

#[inline]
pub fn empty_movelist() -> MoveList {
    use std::mem;
    unsafe{mem::uninitialized()}
    //[Move::null(); MAX_MOVES]
}
#[inline]
pub fn empty_moveorder() -> MoveOrder {
    [(0,0); MAX_MOVES]
}


const NUM_TO_LET : [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
impl fmt::Display for Move {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_pass() {
            write!(f, "(PASS)")
        } else if self.is_null() {
            write!(f, "(NULL)")
        } else {
            write!(f, "({}{})", NUM_TO_LET[self.x() as usize], self.y()+1)
        }
    }
}

impl fmt::Debug for Move {

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

#[derive(Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Board {
    ///Player Stones
    ps   : u64,
    ///Opponent Stones
    os   : u64,
    ///Player Moves
    pm   : u64,
    ///Opponent Moves
    om   : u64,
    ///Current Turn
    ct   : Turn,
}

impl Board {

    /// Returns a new othello board initialized to the staring position.
    pub fn new() -> Board {
        Board {
            ps   : 0b00000000_00000000_00000000_00001000_00010000_00000000_00000000_00000000u64,
            os   : 0b00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000u64,
            pm   : 0b00000000_00000000_00010000_00100000_00000100_00001000_00000000_00000000u64,
            om   : 0b00000000_00000000_00001000_00000100_00100000_00010000_00000000_00000000u64,
            ct   : Turn::BLACK,
        }
    }

    pub fn from_string(data: &[u8]) -> Board {
        let mut m = 1;
        let mut ps = 0u64;
        let mut os = 0u64;

        for &c in data {
            match c as char {
                'B' => { ps |= m;},
                'W' => { os |= m;},
                '\n' => {continue;},
                _ => {},
            }
            m <<= 1;
        }

        Board::position(ps, os, Turn::BLACK)
    }

    /// Returns a new board from a given position and current turn, represented
    /// by two 64-bit integers
    pub fn position(ps : u64, os : u64, ct : Turn) -> Board {
        let pm = 0;
        let om = 0;
        let mut b = Board {
            ps, os,
            pm, om,
            ct,
        };

        b.update_moves_fast();
        b
    }
    
    
    /// Returns a copy of the board
    pub fn copy(&self) -> Board {
        Board {
            ps   : self.ps,
            os   : self.os,
            pm   : self.pm,
            om   : self.om,
            ct   : self.ct,
        }
    }

    pub fn permute(&mut self, perm : usize) {
        let s_syms = all_board_syms(self.ps, self.os);
        let m_syms = all_board_syms(self.pm, self.om);

        self.ps = s_syms[perm].0;
        self.os = s_syms[perm].1;
        self.pm = m_syms[perm].0;
        self.om = m_syms[perm].1;
    }

    /// Gets the current turn
    pub fn get_turn(&self) -> Turn {
        self.ct
    }
    
    /// Checks whether or not the game is over
    pub fn is_done(&self) -> bool {
        self.pm == 0 && self.om == 0
    }
    
    
    /// Checks whether a player has a legal move
    pub fn has_move(&self) -> (bool,bool) {
        (self.pm != 0, self.om != 0)
    }
    
    
    /// Checks whether a move is legal
    pub fn check_move(&self, m : Move) -> (bool,bool) {
        ((self.pm & m.mask()) != 0, (self.om & m.mask()) != 0)
    }
    
    
    /// Updates a board by playing the given move for the given player
    pub fn do_move(&mut self, m : Move) -> u64{
    
        if m.is_pass() || m.is_null() {
            self.swap();
            return 0;
        }
        let mut pro = 0u64; 
        let mut gen = 0u64;
        let mut msk = 0u64;
        let org = m.mask();

        gen = self.ps;
        pro = self.os;

        self.ps |= org;

        msk |= sout_occl(gen, pro) & nort_occl(org, pro);
        msk |= nort_occl(gen, pro) & sout_occl(org, pro);
        msk |= east_occl(gen, pro) & west_occl(org, pro);
        msk |= west_occl(gen, pro) & east_occl(org, pro);
        msk |= sowe_occl(gen, pro) & noea_occl(org, pro);
        msk |= noea_occl(gen, pro) & sowe_occl(org, pro);
        msk |= soea_occl(gen, pro) & nowe_occl(org, pro);
        msk |= nowe_occl(gen, pro) & soea_occl(org, pro);

        self.ps ^= msk;
        self.os ^= msk;

        self.update_moves_fast();

        self.swap();

        msk
    }

    // #[cfg(target_arch = "x86_64")] 
    // pub fn f_do_move(&mut self, m : Move) -> u64 {

    //     use do_moves_fast::fast_do_move;

    //     // if m.is_pass() || m.is_null() {
    //     //     self.swap();
    //     //     return 0;
    //     // }

    //     let flipped = fast_do_move(m.data , m.x(), m.y(), self.ps, self.os);

    //     self.ps ^= flipped;
    //     self.os ^= flipped;

    //     self.os ^= m.mask();

    //     self.update_moves_fast();

    //     self.swap();

    //     flipped
    // }
    

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))] 
    pub fn f_do_move(&mut self, m : Move) -> u64 {

        use do_moves_faster;

        // if m.is_pass() || m.is_null() {
        //     self.swap();
        //     return 0;
        // }

        let flipped = do_moves_faster::do_move(m.data, self.ps, self.os);

        self.ps ^= flipped;
        self.os ^= flipped;

        self.ps ^= m.mask();

        self.update_moves_fast();

        self.swap();

        flipped
    }


    pub fn swap(&mut self) {
        let mut tmp = self.ps;
        self.ps = self.os;
        self.os = tmp;

        tmp = self.pm;
        self.pm = self.om;
        self.om = tmp;

        self.ct = !self.ct;
    }
    
    
    /// Returns the bit mask of the given player's pieces
    pub fn pieces(&self) -> (u64,u64) {
        (self.ps, self.os)
    }
    
    
    /// Returns the mobility bit mask of the given player
    pub fn mobility(&self) -> (u64, u64) {
        (self.pm,self.om)
    }
    
    /// Calculates and returns the exposed pieces
    pub fn calculate_exposed(&self) -> (u64, u64) {
        let e = !(self.ps | self.os);
        let bloom = (
            nort_one(e) | sout_one(e) |
            east_one(e) | west_one(e) |
            noea_one(e) | sowe_one(e) |
            nowe_one(e) | soea_one(e)
        );

        (self.ps & bloom, self.os & bloom)
    }
    
    /// Returns the stability bit mask of the given player
    #[deprecated]
    pub fn stability(&self, t : Turn) -> u64 {
        //sides
        const top : u64 = 255u64;
        const bot : u64 = 18374686479671623680u64;
        const lft : u64 = 72340172838076673u64;
        const rht : u64 = 9259542123273814144u64;
        
        let gen = match t {
            Turn::BLACK => self.ps,
            Turn::WHITE => self.os,
        };
        let pcs = self.ps|self.os;

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
    
    
    /// Counts the number of stones each player has on the board.
    pub fn count_pieces(&self) -> (u8, u8) {
        (popcount_64(self.ps), popcount_64(self.os))
    }
    

    /// Counts gets the piece count difference between the current player and
    /// the opponent.
    pub fn piece_diff(&self) -> i8 {
        popcount_64(self.ps) as i8 - popcount_64(self.os) as i8
    }


    /// Counts the number of stones on the board.
    pub fn total_pieces(&self) -> u8 {
        popcount_64(self.ps | self.os)
    }

    /// Counts the number empty squares on the board.
    pub fn total_empty(&self) -> u8 {
        64-popcount_64(self.ps | self.os)
    }
    
    
    /// Gets the moves available to the current player and stores them in the
    /// array that is passed as an argument. The number of moves is returned.
    pub fn get_moves(&self, out_moves : &mut MoveList) -> u8 {
    
        if !self.has_move().0 {
            out_moves[0] = Move::pass();
            return 1;
        }
    
        let mut mvs = self.pm;
        
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
        
        let mut mvs = !(self.ps | self.os);
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
    
    
    /// Returns the index of a valid move for the current player in the move 
    /// array
    pub fn get_move_index(&self, m : Move) -> usize {
        popcount_64(self.mobility().0 & (m.mask()-1)) as usize
    }
    
    
    // Internal to the Board struct, finds and updates the moves for the given
    // player.
    #[deprecated]
    fn find_moves(&mut self, t : Turn) {
        let mut moves = 0;
        let empty = !(self.ps | self.os);
        let mut tmp = 0;
        
        let (gen,pro) = match t {
            Turn::BLACK => (self.ps,self.os),
            Turn::WHITE => (self.os,self.ps),
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
            Turn::BLACK => {self.pm = moves;},
            Turn::WHITE => {self.om = moves;},
        };
    }

    ///This function makes sure the move bitboards are current in the function
    #[deprecated]
    pub fn update_moves(&mut self) {
        self.find_moves(Turn::BLACK);
        self.find_moves(Turn::WHITE);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]    
    pub fn update_moves_fast(&mut self) {
        use find_moves_fast::fast_find_moves;
        self.pm = fast_find_moves(self.ps, self.os);
        self.om = fast_find_moves(self.os, self.ps);
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ps.to_le().hash(state);
        self.os.to_be().hash(state);
    }
}

use std::cmp;
impl cmp::PartialEq for Board {
    fn eq(&self, other : &Board) -> bool {
        self.ps == other.ps && self.os == other.os
    }
}

impl fmt::Display for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        const XY0 : usize = 145;
        const DX : usize = 6;
        const DY : usize = 156;
        let mut txt_board = include_bytes!("text_board.txt").iter().cloned().collect::<Vec<_>>();

        // let mut err = write!(f, "  A B C D E F G H\n");
        
        for y in 0..8 {
            // let mut t = err.and(write!(f, "{}", y+1));
            // err = t;
            for x in 0..8 {
                let m = Move::new(x,y).mask();
                let xy_i = XY0 + x as usize * DX + y as usize * DY;
                // let e = err.and(
                if self.ps & m != 0 {
                    txt_board[xy_i] = '@' as u8;
                    // write!(f, " @")
                } else if self.os & m != 0 {
                    txt_board[xy_i] = 'O' as u8;
                    // write!(f, " O")
                } else {
                    // write!(f, "  ")
                }
                // );
                // err = e;
            }
            
            // t = err.and(write!(f, "\n"));
            
            // err = t;
        }
        
        // err

        let str_board = String::from_utf8_lossy(&txt_board);

        write!(f, "{}", str_board)
    }
}

impl fmt::Debug for Board {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut err = write!(f, "  A B C D E F G H | A B C D E F G H | A B C D E F G H\n");
        
        for y in 0..8 {
            let mut t = err.and(write!(f, "{}", y+1));
            err = t;
            for x in 0..8 {
                let m = Move::new(x,y).mask();
                let e = err.and(
                    if self.ps & m != 0 {
                        write!(f, " @")
                    } else if self.os & m != 0 {
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
                    if self.pm & m != 0 {
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
                    if self.om & m != 0 {
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
                    if self.ps & m != 0 {
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
                    if self.os & m != 0 {
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

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Position {
    ps : u64,
    os : u64,
}

impl Position {
    pub fn from_board(b : Board) -> Position {
        Position {
            ps : b.ps,
            os : b.os,
        }
    }

    pub fn to_board(&self) -> Board {
        Board::position(self.ps, self.os, Turn::BLACK)
    }
}

















