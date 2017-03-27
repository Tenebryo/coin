
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
    pro &=       (pro >>  8);
    gen |= pro & (gen >> 16);
    pro &=       (pro >> 16);
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
    pro &=       (pro <<  8);
    gen |= pro & (gen << 16);
    pro &=       (pro << 16);
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
    pro &=       (pro << 1);
    gen |= pro & (gen << 2);
    pro &=       (pro << 2);
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
    pro &=       (pro <<  9);
    gen |= pro & (gen << 18);
    pro &=       (pro << 18);
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
    pro &=       (pro >>  7);
    gen |= pro & (gen >> 14);
    pro &=       (pro >> 14);
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
    pro &=       (pro >> 1);
    gen |= pro & (gen >> 2);
    pro &=       (pro >> 2);
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
    pro &=       (pro >>  9);
    gen |= pro & (gen >> 18);
    pro &=       (pro >> 18);
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
    pro &=       (pro <<  7);
    gen |= pro & (gen << 14);
    pro &=       (pro << 14);
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



