
#[inline]
pub fn popcount_64(mut x : u64) -> u8 {
    x = (x & 0x5555555555555555u64) + ((x >> 1) & 0x5555555555555555u64);
    x = (x & 0x3333333333333333u64) + ((x >> 2) & 0x3333333333333333u64);
    x = (x & 0x0F0F0F0F0F0F0F0Fu64) + ((x >> 4) & 0x0F0F0F0F0F0F0F0Fu64);
   ((x * 0x0101010101010101u64) >> 56) as u8
}

#[inline]
pub fn bitscan_64(x : u64) -> u8 {
    popcount_64( ( x & -(x as i64) as u64) -1 )
}

pub const NOT_A_FILE : u64 = 0xFE_FE_FE_FE_FE_FE_FE_FEu64; // ~0x0101010101010101
pub const NOT_H_FILE : u64 = 0x7F_7F_7F_7F_7F_7F_7F_7Fu64; // ~0x8080808080808080
pub const NOT_1_ROW  : u64 = 0xFF_FF_FF_FF_FF_FF_FF_00u64; // ~0x00000000000000FF
pub const NOT_8_ROW  : u64 = 0x00_FF_FF_FF_FF_FF_FF_FFu64; // ~0xFF00000000000000

#[inline]
pub fn sout_occl(mut gen : u64, mut pro : u64) -> u64{
    pro &= NOT_8_ROW;
    gen |= pro & (gen >>  8);
    pro &=        pro >>  8 ;
    gen |= pro & (gen >> 16);
    pro &=        pro >> 16 ;
    gen |= pro & (gen >> 32);
    gen
}

#[inline]
pub fn sout_one(gen : u64) -> u64{
    (gen & NOT_1_ROW) >> 8
}

#[inline]
pub fn nort_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= NOT_1_ROW;
    gen |= pro & (gen <<  8);
    pro &=        pro <<  8 ;
    gen |= pro & (gen << 16);
    pro &=        pro << 16 ;
    gen |= pro & (gen << 32);
    gen
}

#[inline]
pub fn nort_one(gen : u64) -> u64 {
    (gen & NOT_8_ROW) << 8
}

#[inline]
pub fn east_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= NOT_A_FILE;
    gen |= pro & (gen << 1);
    pro &=        pro << 1 ;
    gen |= pro & (gen << 2);
    pro &=        pro << 2 ;
    gen |= pro & (gen << 4);
    gen
}

#[inline]
pub fn east_one(gen : u64) -> u64 {
    (gen & NOT_H_FILE) << 1
}

#[inline]
pub fn noea_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= NOT_A_FILE & NOT_8_ROW;
    gen |= pro & (gen <<  9);
    pro &=        pro <<  9 ;
    gen |= pro & (gen << 18);
    pro &=        pro << 18 ;
    gen |= pro & (gen << 36);
    gen
}

#[inline]
pub fn noea_one(gen : u64) -> u64 {
    (gen & NOT_H_FILE & NOT_8_ROW) << 9
}

#[inline]
pub fn soea_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= NOT_A_FILE & NOT_1_ROW;
    gen |= pro & (gen >>  7);
    pro &=        pro >>  7 ;
    gen |= pro & (gen >> 14);
    pro &=        pro >> 14 ;
    gen |= pro & (gen >> 28);
    gen
}

#[inline]
pub fn soea_one(gen : u64) -> u64 {
    (gen & NOT_H_FILE & NOT_1_ROW) >> 7
}

#[inline]
pub fn west_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= NOT_H_FILE;
    gen |= pro & (gen >> 1);
    pro &=        pro >> 1 ;
    gen |= pro &  gen >> 2 ;
    pro &=        pro >> 2 ;
    gen |= pro & (gen >> 4);
    gen
}

#[inline]
pub fn west_one(gen : u64) -> u64 {
    (gen & NOT_A_FILE) >> 1
}

#[inline]
pub fn sowe_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= NOT_H_FILE & NOT_1_ROW;
    gen |= pro & (gen >>  9);
    pro &=        pro >>  9 ;
    gen |= pro & (gen >> 18);
    pro &=        pro >> 18 ;
    gen |= pro & (gen >> 36);
    gen
}

#[inline]
pub fn sowe_one(gen : u64) -> u64 {
    (gen & NOT_A_FILE & NOT_1_ROW) >> 9
}

#[inline]
pub fn nowe_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= NOT_H_FILE & NOT_8_ROW;
    gen |= pro & (gen <<  7);
    pro &=        pro <<  7 ;
    gen |= pro & (gen << 14);
    pro &=        pro << 14 ;
    gen |= pro & (gen << 28);
    gen
}

#[inline]
pub fn nowe_one(gen : u64) -> u64 {
    (gen & NOT_A_FILE & NOT_8_ROW) << 7
}

#[inline]
pub fn propagate(mut gen : u64) -> u64 {
   let mut attacks = east_one(gen) | west_one(gen);
   gen      |= attacks;
   attacks  |= nort_one(gen) | sout_one(gen);
   return attacks;
}



#[inline]
pub fn vertical_mirror(mut b : u64) -> u64 {
    b = ((b >>  8) & 0x00FF00FF00FF00FFu64) | ((b <<  8) & 0xFF00FF00FF00FF00u64);
    b = ((b >> 16) & 0x0000FFFF0000FFFFu64) | ((b << 16) & 0xFFFF0000FFFF0000u64);
    b = ((b >> 32) & 0x00000000FFFFFFFFu64) | ((b << 32) & 0xFFFFFFFF00000000u64);
    b
}

#[inline]
pub fn horizontal_mirror(mut b : u64) -> u64 {
    b = ((b >> 1) & 0x5555555555555555u64) | ((b << 1) & 0xAAAAAAAAAAAAAAAAu64);
    b = ((b >> 2) & 0x3333333333333333u64) | ((b << 2) & 0xCCCCCCCCCCCCCCCCu64);
    b = ((b >> 4) & 0x0F0F0F0F0F0F0F0Fu64) | ((b << 4) & 0xF0F0F0F0F0F0F0F0u64);
    b
}

#[inline]
pub fn transpose(mut b : u64) -> u64 {
    let mut t = 0;

    t = (b ^ (b >> 7)) & 0x00AA00AA00AA00AAu64;
    b = b ^ t ^ (t << 7);
    t = (b ^ (b >> 14)) & 0x0000CCCC0000CCCCu64;
    b = b ^ t ^ (t << 14);
    t = (b ^ (b >> 28)) & 0x00000000F0F0F0F0u64;
    b = b ^ t ^ (t << 28);

    b
}

#[inline]
pub fn board_sym(mut bb : u64, mut bw : u64, sym : u8) -> (u64,u64) {
    if sym & 1 != 0 {
        bb = horizontal_mirror(bb);
        bw = horizontal_mirror(bw);
    }
    if sym & 2 != 0 {
        bb = vertical_mirror(bb);
        bw = vertical_mirror(bw);
    }
    if sym & 4 != 0 {
        bb = transpose(bb);
        bw = transpose(bw);
    }

    (bb,bw)
}

pub const SYMMETRY_MAPS : [[usize; 64]; 8] = [
    [
         0,  1,  2,  3,  4,  5,  6,  7,
         8,  9, 10, 11, 12, 13, 14, 15,
        16, 17, 18, 19, 20, 21, 22, 23,
        24, 25, 26, 27, 28, 29, 30, 31,
        32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55,
        56, 57, 58, 59, 60, 61, 62, 63
    ],[
         7,  6,  5,  4,  3,  2,  1,  0,
        15, 14, 13, 12, 11, 10,  9,  8,
        23, 22, 21, 20, 19, 18, 17, 16,
        31, 30, 29, 28, 27, 26, 25, 24,
        39, 38, 37, 36, 35, 34, 33, 32,
        47, 46, 45, 44, 43, 42, 41, 40,
        55, 54, 53, 52, 51, 50, 49, 48,
        63, 62, 61, 60, 59, 58, 57, 56,
    ],[
        56, 57, 58, 59, 60, 61, 62, 63,
        48, 49, 50, 51, 52, 53, 54, 55,
        40, 41, 42, 43, 44, 45, 46, 47,
        32, 33, 34, 35, 36, 37, 38, 39,
        24, 25, 26, 27, 28, 29, 30, 31,
        16, 17, 18, 19, 20, 21, 22, 23,
         8,  9, 10, 11, 12, 13, 14, 15,
         0,  1,  2,  3,  4,  5,  6,  7
    ],[
        63, 62, 61, 60, 59, 58, 57, 56,
        55, 54, 53, 52, 51, 50, 49, 48,
        47, 46, 45, 44, 43, 42, 41, 40,
        39, 38, 37, 36, 35, 34, 33, 32,
        31, 30, 29, 28, 27, 26, 25, 24,
        23, 22, 21, 20, 19, 18, 17, 16,
        15, 14, 13, 12, 11, 10,  9,  8,
         7,  6,  5,  4,  3,  2,  1,  0
    ],[
         0,  8, 16, 24, 32, 40, 48, 56,
         1,  9, 17, 25, 33, 41, 49, 57,
         2, 10, 18, 26, 34, 42, 50, 58,
         3, 11, 19, 27, 35, 43, 51, 59,
         4, 12, 20, 28, 36, 44, 52, 60,
         5, 13, 21, 29, 37, 45, 53, 61,
         6, 14, 22, 30, 38, 46, 54, 62,
         7, 15, 23, 31, 39, 47, 55, 63
    ],[
         7, 15, 23, 31, 39, 47, 55, 63,
         6, 14, 22, 30, 38, 46, 54, 62,
         5, 13, 21, 29, 37, 45, 53, 61,
         4, 12, 20, 28, 36, 44, 52, 60,
         3, 11, 19, 27, 35, 43, 51, 59,
         2, 10, 18, 26, 34, 42, 50, 58,
         1,  9, 17, 25, 33, 41, 49, 57,
         0,  8, 16, 24, 32, 40, 48, 56
    ],[
        56, 48, 40, 32, 24, 16,  8,  0,
        57, 49, 41, 33, 25, 17,  9,  1,
        58, 50, 42, 34, 26, 18, 10,  2,
        59, 51, 43, 35, 27, 19, 11,  3,
        60, 52, 44, 36, 28, 20, 12,  4,
        61, 53, 45, 37, 29, 21, 13,  5,
        62, 54, 46, 38, 30, 22, 14,  6,
        63, 55, 47, 39, 31, 23, 15,  7
    ],[
        63, 55, 47, 39, 31, 23, 15,  7,
        62, 54, 46, 38, 30, 22, 14,  6,
        61, 53, 45, 37, 29, 21, 13,  5,
        60, 52, 44, 36, 28, 20, 12,  4,
        59, 51, 43, 35, 27, 19, 11,  3,
        58, 50, 42, 34, 26, 18, 10,  2,
        57, 49, 41, 33, 25, 17,  9,  1,
        56, 48, 40, 32, 24, 16,  8,  0
    ]
];

#[inline]
pub fn all_board_syms(mut bb : u64, mut bw : u64) -> [(u64,u64);8] {
    let mut r = [(0,0);8];

    r[0] = (bb, bw);

    bb = transpose(bb);
    bw = transpose(bw);

    r[4] = (bb, bw);

    bb = vertical_mirror(bb);
    bw = vertical_mirror(bw);

    r[5] = (bb, bw);

    bb = transpose(bb);
    bw = transpose(bw);

    r[1] = (bb, bw);

    bb = vertical_mirror(bb);
    bw = vertical_mirror(bw);

    r[3] = (bb, bw);

    bb = transpose(bb);
    bw = transpose(bw);

    r[7] = (bb, bw);

    bb = vertical_mirror(bb);
    bw = vertical_mirror(bw);

    r[6] = (bb, bw);

    bb = transpose(bb);
    bw = transpose(bw);

    r[2] = (bb, bw);

    r
}

pub fn print_bitboard(b : u64) {
    println!("{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n{:08b}\n",
        (b >>  0) & 0xff,
        (b >>  8) & 0xff,
        (b >> 16) & 0xff,
        (b >> 24) & 0xff,
        (b >> 32) & 0xff,
        (b >> 40) & 0xff,
        (b >> 48) & 0xff,
        (b >> 56) & 0xff
    );
}