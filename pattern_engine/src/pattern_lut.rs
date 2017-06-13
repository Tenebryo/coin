use bson;

const TABLE_SIZE : usize = 65536;
const SHORT_MASKS : [u64; 4] = [
    0x0000_0000_0000_FFFF,
    0x0000_0000_FFFF_0000,
    0x0000_FFFF_0000_0000,
    0xFFFF_0000_0000_0000
];
const SHORT_OFFSET : [u64; 4] = [0,16,32,48];

///A LUT to convert stone configurations of static square sets into consecutive
///indices.
pub struct PatternLUT {
    mask  : u64,
    table : [[u32; TABLE_SIZE];4],
}

impl PatternLUT {
    ///Create a new empty Pattern Look Up Table
    pub fn new(mask : u64) -> PatternLUT {
        PatternLUT {
            mask,
            table : [[0; TABLE_SIZE]; 4],
        }
    }

    ///Generate a Pattern Look Up Table given a mask that represents a static
    ///set of board squares
    pub fn from_mask(mask : u64) -> PatternLUT {
        let mut bits = vec![];
        let mut threes = vec![];
        let mut e = 1;
        for i in 0..64 {
            if mask & (1<<i) != 0 {
                bits.push(mask & (1<<i));
                threes.push(e);
                e *= 3;
            }
        }
        let nbits = bits.len();

        let mut lut = PatternLUT::new(mask);

        for m in 0..4 { 
            for i in 0..TABLE_SIZE {
                for j in 0..(nbits) {
                    if (((i as u64) << (SHORT_OFFSET[m])) & (SHORT_MASKS[m]) & (bits[j])) != 0 {
                        lut.table[m][i] += threes[j];
                    }
                }
            }
        }

        lut
    }

    ///Evaluate the LUT for a bitboard; returns a unique index for each 
    ///configuration of the pattern squares.
    pub fn eval(&self, mut bits_b : u64, mut bits_w : u64) -> usize {
        let mut i = 0;

        assert!((bits_b & bits_w) == 0);

        i += self.table[0][(bits_w & 0xFFFF) as usize];
        bits_w >>= 16;
        i += self.table[1][(bits_w & 0xFFFF) as usize];
        bits_w >>= 16;
        i += self.table[2][(bits_w & 0xFFFF) as usize];
        bits_w >>= 16;
        i += self.table[3][(bits_w & 0xFFFF) as usize];

        i <<= 1;

        i += self.table[0][(bits_b & 0xFFFF) as usize];
        bits_b >>= 16;
        i += self.table[1][(bits_b & 0xFFFF) as usize];
        bits_b >>= 16;
        i += self.table[2][(bits_b & 0xFFFF) as usize];
        bits_b >>= 16;
        i += self.table[3][(bits_b & 0xFFFF) as usize];

        return i as usize;
    }
}