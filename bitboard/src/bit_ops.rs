
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