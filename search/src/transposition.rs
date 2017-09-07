use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;

use bitboard::Board;
use std::i32;

type Position = (u64, u64);

type ZobristHash = u32;

type Usage = (usize, ZobristHash); //age then zobrist hash

fn zobrist(b : Board) -> ZobristHash {
    let (mut bp, mut wp) = b.pieces();
    
    let mut hash = 0;
    
    for i in 0..64 {
        if (bp & 1) != 0 {
            hash ^= ZOBRIST_BITS[i][1];
        } else if (wp & 1) != 0 {
            hash ^= ZOBRIST_BITS[i][2];
        } else {
            hash ^= ZOBRIST_BITS[i][0];
        }
        bp >>= 1;
        wp >>= 1;
    }
    
    hash
}

fn position(b : Board) -> Position {
    b.pieces()
}

const DEFAULT_VALUE : (i32, i32) = (i32::MIN, i32::MAX);

// ///Transposition table. Currently maintains a maximum size by removing the oldest
// ///key when overflow occurs. This could be changed to remove a random key or
// ///perhaps the least used key.
// pub struct TranspositionTable {
//     entries     : HashMap<ZobristHash, (Position, i32, i32)>,
//     size        : usize,
//     age         : usize,
// }


// impl TranspositionTable {
    
//     ///Creates a new transpotion table with a given maximum size
//     ///
//     pub fn new(max_size : usize) -> TranspositionTable {
//         TranspositionTable {
//             entries     : HashMap::new(),
//             size        : if max_size <= 0 {1} else {max_size},
//             age         : 0,
//         }
//     }
    
//     ///Inserts or updates 
//     pub fn update(&mut self, b : Board, low : i32, high : i32) {
//         let zob = zobrist(b);
//         let pos = position(b);
        
//         if self.entries.contains_key(&zob) {
//             //overwrite the old entry, because we have a new one
//             let mut e = self.entries.entry(zob).or_insert((pos, low, high));
//             *e = (pos, low, high);
//             return;
//         }
        
//         if self.size <= 0 {
//             //No more space, need to remove a random key to insert new one.
//             //in this case the size doesn't change
//             let (&key,_) = self.entries.iter().next().unwrap();
//             self.entries.remove(&key);
            
//         } else {
//             //If there is space in the table, the remaining space will decrease.
//             self.size -= 1;
//         }
        
//         //insert bound information
//         self.entries.insert(zob, (pos,low,high));
        
//         //age increases
//         self.age += 1;
//     }
    
//     ///Gets the bound information for a board.
//     pub fn fetch(&self, b : Board) -> (i32, i32) {
//         let zob = zobrist(b);
//         if self.entries.contains_key(&zob) {
//             let pos = position(b);
            
//             //should never panic on this unwrap.
//             let &(p, low, high) = self.entries.get(&zob).unwrap();
            
//             //only fetch if it is actually the correct board stored here and not
//             //a hash collision.
//             if p == pos {
//                 (low, high)
//             } else {
//                 DEFAULT_VALUE
//             }
//         } else {
//             DEFAULT_VALUE
//         }
//     }
    
    
//     ///Clears all entries from the table
//     pub fn clear(&mut self) {
//         self.size += self.entries.len();
//         self.entries.clear();
//         self.age = 0;
//     }
    
    
//     ///Returns the number of entries stored in the table.
//     pub fn size(&self) -> usize {
//         self.entries.len()
//     }
// }

///Parallel Transposition table. Currently maintains a maximum size by removing the oldest
///key when overflow occurs. This could be changed to remove a random key or
///perhaps the least used key.
pub struct TranspositionTable {
    entries     : Mutex<HashMap<ZobristHash, (Position, i32, i32)>>,
    size        : AtomicUsize,
    age         : AtomicUsize,
}

impl TranspositionTable {
    
    ///Creates a new transpotion table with a given maximum size
    ///
    pub fn new(max_size : usize) -> TranspositionTable {
        TranspositionTable {
            entries     : Mutex::new(HashMap::new()),
            size        : AtomicUsize::new(if max_size <= 0 {1} else {max_size}),
            age         : AtomicUsize::new(0),
        }
    }
    
    ///Inserts or updates 
    pub fn update(&self, b : Board, low : i32, high : i32) {

        let mut entries = self.entries.lock().unwrap();

        let zob = zobrist(b);
        let pos = position(b);
        
        if entries.contains_key(&zob) {
            //overwrite the old entry, because we have a new one
            let mut e = entries.entry(zob).or_insert((pos, low, high));
            *e = (pos, low, high);
            return;
        }
        
        if self.size.load(SeqCst) <= 0 {
            //No more space, need to remove a random key to insert new one.
            //in this case the size doesn't change
            let (&key,_) = entries.iter().next().unwrap();
            entries.remove(&key);
            
        } else {
            //If there is space in the table, the remaining space will decrease.
            self.size.fetch_sub(1, SeqCst);
        }
        
        //insert bound information
        entries.insert(zob, (pos,low,high));
        
        //age increases
        self.age.fetch_add(1, SeqCst);
    }
    
    ///Gets the bound information for a board.
    pub fn fetch(&self, b : Board) -> (i32, i32) {
        let entries = self.entries.lock().unwrap();

        let zob = zobrist(b);
        if entries.contains_key(&zob) {
            let pos = position(b);
            
            //should never panic on this unwrap.
            let &(p, low, high) = entries.get(&zob).unwrap();
            
            //only fetch if it is actually the correct board stored here and not
            //a hash collision.
            if p == pos {
                (low, high)
            } else {
                DEFAULT_VALUE
            }
        } else {
            DEFAULT_VALUE
        }
    }
    
    
    ///Clears all entries from the table
    pub fn clear(&self) {
        let mut entries = self.entries.lock().unwrap();

        self.size.fetch_add(entries.len(), SeqCst);
        entries.clear();
        self.age.store(0, SeqCst);
    }
    
    
    ///Returns the number of entries stored in the table.
    pub fn size(&self) -> usize {
        self.entries.lock().unwrap().len()
    }
}

// #[test]
// fn perf_transposition_table() {
//     use std::time::Instant;
//     use std::time::Duration;
    
//     let iters = 10000usize;

//     let mut tt = TranspositionTable::new(2000000);

//     let b = Board::new();

//     let s = Instant::now();
//     for _ in 0..iters {
//         tt.update(b, 0, 100000);
//     }
//     let t = s.elapsed();

//     eprintln!("SEQ: {:?}/{}", t, iters);
// }

// #[test]
// fn perf_transposition_table_par() {

//     use std::time::Instant;
//     use std::time::Duration;
    
//     let iters = 10000usize;

//     let mut tt = ParTranspositionTable::new(2000000);

//     let b = Board::new();

//     let s = Instant::now();
//     for _ in 0..iters {
//         tt.update(b, 0, 100000);
//     }
//     let tm = s.elapsed();

//     eprintln!("PAR: {:?}/{}", tm, iters);
// }

const ZOBRIST_BITS : [[u32; 3]; 64] = [
    [0b01000101011110010011101011111001u32, 0b00111000111010010110011001110110u32, 0b00011111100111001001101000000000u32],
    [0b00110011100001111100011101001101u32, 0b11011111000101110001000111110100u32, 0b01000011101011010000010101000010u32],
    [0b11011101101111011110010001101010u32, 0b01110010011000110001110101010100u32, 0b00110111111110111110110010111111u32],
    [0b01100001001111001110110110000101u32, 0b01110011100100010000001100101001u32, 0b00000111010111100100101001011011u32],
    [0b01100010011010010010011000101011u32, 0b10001011010001111011100010001001u32, 0b10010010110101010011101011101111u32],
    [0b00110010010000000100101101101110u32, 0b01011010101110010000001110011101u32, 0b11101110010011100010010110101000u32],
    [0b11110111100000110111100011000101u32, 0b10010000101101010000011011111000u32, 0b11000000010000100000010001100001u32],
    [0b11101000101101101011110010110100u32, 0b11101000110110010111010011001000u32, 0b01110100101101100100000010110001u32],
    [0b00111001110000101101110010011000u32, 0b00000000000011110000011010110011u32, 0b11010111111010100100110000101000u32],
    [0b00101000100100100010011111110100u32, 0b00111010101101110010110010010110u32, 0b10100111111110101100010111010011u32],
    [0b11101110101010011110011110000000u32, 0b10001000101011001000010110100010u32, 0b00100000010010010110100000110010u32],
    [0b11111100011011101111101000011101u32, 0b00011110110101000111001001101000u32, 0b11101110011101001111101100000100u32],
    [0b11101101100010010100100101000111u32, 0b10011011010100001000110011101010u32, 0b10000100010110000010000010001101u32],
    [0b10111011110100101111111111101111u32, 0b01100110001010101000110010110011u32, 0b11110111100001000101111100010110u32],
    [0b00000011010101111111100111010011u32, 0b01001100010101110101101000100101u32, 0b01101011010010111011101001001000u32],
    [0b10110001100011010101110001100101u32, 0b00101011011011000111101011011111u32, 0b11010010101010101100011011000101u32],
    [0b11011001100001001100000110001111u32, 0b11110001100010011101111001100110u32, 0b00111101000101111000000010110100u32],
    [0b11111010110000101000110101100010u32, 0b10101011100110011011110011111110u32, 0b10011001001110011001001100000001u32],
    [0b00110000100000101110111100111010u32, 0b01110101111010010000000111110011u32, 0b00010011101001111101001101000011u32],
    [0b11001001000000111111110110111010u32, 0b00100101100101101101100001010010u32, 0b00010100100101101100111000110111u32],
    [0b00001010110001010000010110001011u32, 0b01000111101111101100011011101110u32, 0b00011001100001011101000001010010u32],
    [0b00010111101000111001100011101100u32, 0b01011100100101100010101001111101u32, 0b10011010101010000010010010110011u32],
    [0b10100000011010001010101011000010u32, 0b10011000100010101000101100000011u32, 0b01001010010101111111101111101001u32],
    [0b10111011011101110110111010001010u32, 0b10111110101001100011101100001111u32, 0b10111000110111111010111100110111u32],
    [0b11011110100010000100011011000011u32, 0b01010011101001011000111011011001u32, 0b10011110111111011100010010000000u32],
    [0b10000011001111111101101000100101u32, 0b01101100111101000001110000111111u32, 0b01111010011001001000110010111010u32],
    [0b00100111010101011011010010110010u32, 0b10001000011110000000101110001100u32, 0b01111000000011001110111100010001u32],
    [0b01100111001101001110111001001011u32, 0b00011001111111011101001100011011u32, 0b01011100111101100101010100110111u32],
    [0b01101011010011101110110010000011u32, 0b00011011110011101001111001101100u32, 0b01111011011010000000011101001100u32],
    [0b00100011001011110110100110101100u32, 0b10110101010011011001010111001011u32, 0b11010100110111100110000111001100u32],
    [0b11111111000000100000101111000110u32, 0b11101010000000010100100011110111u32, 0b00110001100110010001100001100101u32],
    [0b01011000100010001010011001100001u32, 0b10111110111001001011111001100101u32, 0b00100000101000110101100110011011u32],
    [0b10110111110010111100100101000101u32, 0b01011110011101011101101101111010u32, 0b11000100101101101111111010010101u32],
    [0b10010111110101100111000000111110u32, 0b10001010101001000101101001011000u32, 0b00011010011111010000100101011010u32],
    [0b11111111010001111011111101110101u32, 0b00111111111100000111001101111101u32, 0b11101100100111011111101111011010u32],
    [0b10000001111011100110001110011111u32, 0b01000101100011011010100101011011u32, 0b00010100001101111011001100110101u32],
    [0b00101101010100000001110101001101u32, 0b00000111011100001010110001110001u32, 0b01111010110111010000000110011111u32],
    [0b11010010011100101000101000100111u32, 0b01111001111100011100111101011100u32, 0b00010101111011000000111100000010u32],
    [0b11001100111110110011011100110000u32, 0b11110101011101101011110010001011u32, 0b00100101101111100100101010001100u32],
    [0b01000010000100100111001111101111u32, 0b00001111010101100011011110111000u32, 0b01010100101101100010111101110111u32],
    [0b11101101100000000011110101101101u32, 0b00100000001001011010001110001011u32, 0b11010111101100111101010011001100u32],
    [0b10111101100111010100101101100101u32, 0b01111011001100011101001000000010u32, 0b10101001000110110011000011000011u32],
    [0b01001001000010100100101111101111u32, 0b11110001100011101111100100000011u32, 0b11110011011001110001100100000110u32],
    [0b00100001110110000111110011111011u32, 0b10000001101111111101110100011010u32, 0b11001001100010101010110010010110u32],
    [0b00100001001010101011110001001111u32, 0b00001000001101110101100101110010u32, 0b10101000011000011111000010110010u32],
    [0b00010100010011101101111001001010u32, 0b00000110010010111110110100110111u32, 0b00100001100110100111110000000000u32],
    [0b00101011010010000111000100110001u32, 0b01010001001010011100000100001010u32, 0b00110110001100010100110101100100u32],
    [0b01100110101100100110111101000010u32, 0b11111100101010110011101011000011u32, 0b00100100100000001100011010010100u32],
    [0b01000011010110111101010010110010u32, 0b10001001011110000010000001000100u32, 0b10010001011100001111101010100001u32],
    [0b11101100010110100111000111111010u32, 0b10101110001010110001101000000101u32, 0b01010001001111010101010111001101u32],
    [0b10011000101010001110001011000110u32, 0b10101111111100011111010000011000u32, 0b10010101000100100010010101001010u32],
    [0b01010001000000101111010101011010u32, 0b01101011111000001110101001010010u32, 0b01110001111101111000000100011001u32],
    [0b10011101100110000011110101001011u32, 0b00011101011011011001010101100000u32, 0b01110011011110010001101011100110u32],
    [0b01110110100000010110000101101011u32, 0b00100101101100001111100001010100u32, 0b11100001001000100111010001011001u32],
    [0b01010100101010101110110000000111u32, 0b10100110101000000000001000100100u32, 0b10001011011101000011100110100000u32],
    [0b10011101101110001100001000011000u32, 0b11011011100100100011010110110010u32, 0b10100000001101100101111000110000u32],
    [0b10011000000110101001011010011111u32, 0b10010110011000001001101000011101u32, 0b10011111111000111010011011110101u32],
    [0b10100100110011110110010011111001u32, 0b10010001101001000110110000001001u32, 0b00000111111111100111010110111010u32],
    [0b10010110011000010101110110101010u32, 0b10110111100000000000001001001111u32, 0b01011011000011100011010000001110u32],
    [0b00010011100000100111111101010110u32, 0b01000011100000001011011110111101u32, 0b11011000010011100100011010111000u32],
    [0b01111111110101110101000000110110u32, 0b10010100010111101000000001000011u32, 0b00100100011111001000110000110001u32],
    [0b11111001100100110010111000110000u32, 0b11000000111011111111101110110010u32, 0b10110110111110111001101001111110u32],
    [0b10110110110001011110110000001100u32, 0b00010100001110000111010111010100u32, 0b11101010000101100110001001000000u32],
    [0b10001100001000110011001001001010u32, 0b11110010101110110100111111100001u32, 0b11011011010011001011100001100100u32],
];
