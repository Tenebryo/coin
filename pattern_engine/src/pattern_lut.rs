use twos_to_threes_lut::TWOS_TO_THREES;


#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const TABLE_SIZE : usize = 256;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const SHORT_MASKS : [u64; 8] = [
    0x0000_0000_0000_00FF,
    0x0000_0000_0000_FF00,
    0x0000_0000_00FF_0000,
    0x0000_0000_FF00_0000,
    0x0000_00FF_0000_0000,
    0x0000_FF00_0000_0000,
    0x00FF_0000_0000_0000,
    0xFF00_0000_0000_0000
];

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const SHORT_OFFSET : [u64; 8] = [0,8,16,24,32,40,48,56];

///A LUT to convert stone configurations of static square sets into consecutive
///indices.
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub struct PatternLUT {
    mask  : u64,
    table : [[u32; TABLE_SIZE];8],
}

#[derive(Serialize, Deserialize)]
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl PatternLUT {
    ///Create a new empty Pattern Look Up Table
    fn new(mask : u64) -> PatternLUT {
        PatternLUT {
            mask,
            table : [[0; TABLE_SIZE]; 8],
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

        for m in 0..8 { 
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

        i += self.table[0][(bits_w & 0xFF) as usize];
        bits_w >>= 8;
        i += self.table[1][(bits_w & 0xFF) as usize];
        bits_w >>= 8;
        i += self.table[2][(bits_w & 0xFF) as usize];
        bits_w >>= 8;
        i += self.table[3][(bits_w & 0xFF) as usize];
        bits_w >>= 8;
        i += self.table[4][(bits_w & 0xFF) as usize];
        bits_w >>= 8;
        i += self.table[5][(bits_w & 0xFF) as usize];
        bits_w >>= 8;
        i += self.table[6][(bits_w & 0xFF) as usize];
        bits_w >>= 8;
        i += self.table[7][(bits_w & 0xFF) as usize];

        i <<= 1;

        i += self.table[0][(bits_b & 0xFF) as usize];
        bits_b >>= 8;
        i += self.table[1][(bits_b & 0xFF) as usize];
        bits_b >>= 8;
        i += self.table[2][(bits_b & 0xFF) as usize];
        bits_b >>= 8;
        i += self.table[3][(bits_b & 0xFF) as usize];
        bits_b >>= 8;
        i += self.table[4][(bits_b & 0xFF) as usize];
        bits_b >>= 8;
        i += self.table[5][(bits_b & 0xFF) as usize];
        bits_b >>= 8;
        i += self.table[6][(bits_b & 0xFF) as usize];
        bits_b >>= 8;
        i += self.table[7][(bits_b & 0xFF) as usize];

        return i as usize;
    }
}

///A LUT to convert stone configurations of static square sets into consecutive
///indices.
#[derive(Serialize, Deserialize, Clone)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub struct PatternLUT {
    mask  : u64,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl PatternLUT {

    ///Generate a Pattern Look Up Table given a mask that represents a static
    ///set of board squares
    pub fn from_mask(mask : u64) -> PatternLUT {
        PatternLUT {
            mask,
        }
    }

    pub fn eval(&self, bits_b : u64, bits_w : u64) -> usize {
        #[allow(unused_assignments)]
        let mut r1 : u64 = 0;
        #[allow(unused_assignments)]
        let mut r2 : u64 = 0;

        unsafe {
            asm!("PEXT $2, $1, $0" : "=r"(r1) : "r"(bits_w) , "r"(self.mask));
            asm!("PEXT $2, $1, $0" : "=r"(r2) : "r"(bits_b) , "r"(self.mask));
        }
        
        r1 = TWOS_TO_THREES[r1 as usize] as u64;
        r2 = TWOS_TO_THREES[r2 as usize] as u64;

        (r1 + r1 + r2) as usize
    }
}