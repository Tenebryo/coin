
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

pub const not_a_file : u64 = 0xfefefefefefefefeu64; // ~0x0101010101010101
pub const not_h_file : u64 = 0x7f7f7f7f7f7f7f7fu64; // ~0x8080808080808080
pub const not_1_row  : u64 = 0xFF_FF_FF_FF_FF_FF_FF_00u64;
pub const not_8_row  : u64 = 0x00_FF_FF_FF_FF_FF_FF_FFu64;

#[inline]
pub fn sout_occl(mut gen : u64, mut pro : u64) -> u64{
    gen &= not_1_row;
    gen |= pro & (gen >>  8);
    pro &=       (pro >>  8);
    gen |= pro & (gen >> 16);
    pro &=       (pro >> 16);
    gen |= pro & (gen >> 32);
    gen
}

#[inline]
pub fn sout_one(gen : u64) -> u64{
    (gen & not_1_row) >> 8
}

#[inline]
pub fn nort_occl(mut gen : u64, mut pro : u64) -> u64 {
    gen &= not_8_row;
    gen |= pro & (gen <<  8);
    pro &=       (pro <<  8);
    gen |= pro & (gen << 16);
    pro &=       (pro << 16);
    gen |= pro & (gen << 32);
    gen
}

#[inline]
pub fn nort_one(gen : u64) -> u64 {
    (gen & not_8_row) << 8
}

#[inline]
pub fn east_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= not_a_file;
    gen |= pro & (gen << 1);
    pro &=       (pro << 1);
    gen |= pro & (gen << 2);
    pro &=       (pro << 2);
    gen |= pro & (gen << 4);
    gen
}

#[inline]
pub fn east_one(gen : u64) -> u64 {
    (gen & not_a_file) << 1
}

#[inline]
pub fn noea_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= not_a_file & not_8_row;
    gen |= pro & (gen <<  9);
    pro &=       (pro <<  9);
    gen |= pro & (gen << 18);
    pro &=       (pro << 18);
    gen |= pro & (gen << 36);
    gen
}

#[inline]
pub fn noea_one(gen : u64) -> u64 {
    (gen & not_a_file & not_8_row) << 9
}

#[inline]
pub fn soea_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= not_a_file & not_1_row;
    gen |= pro & (gen >>  7);
    pro &=       (pro >>  7);
    gen |= pro & (gen >> 14);
    pro &=       (pro >> 14);
    gen |= pro & (gen >> 28);
    gen
}

#[inline]
pub fn soea_one(gen : u64) -> u64 {
    (gen & not_a_file & not_1_row) >> 7
}

#[inline]
pub fn west_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= not_h_file;
    gen |= pro & (gen >> 1);
    pro &=       (pro >> 1);
    gen |= pro & (gen >> 2);
    pro &=       (pro >> 2);
    gen |= pro & (gen >> 4);
    gen
}

#[inline]
pub fn west_one(gen : u64) -> u64 {
    (gen & not_h_file) >> 1
}

#[inline]
pub fn sowe_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= not_h_file & not_1_row;
    gen |= pro & (gen >>  9);
    pro &=       (pro >>  9);
    gen |= pro & (gen >> 18);
    pro &=       (pro >> 18);
    gen |= pro & (gen >> 36);
    gen
}

#[inline]
pub fn sowe_one(gen : u64) -> u64 {
    (gen & not_h_file & not_1_row) >> 9
}

#[inline]
pub fn nowe_occl(mut gen : u64, mut pro : u64) -> u64 {
    pro &= not_h_file & not_8_row;
    gen |= pro & (gen <<  7);
    pro &=       (pro <<  7);
    gen |= pro & (gen << 14);
    pro &=       (pro << 14);
    gen |= pro & (gen << 28);
    gen
}

#[inline]
pub fn nowe_one(gen : u64) -> u64 {
    (gen & not_h_file & not_8_row) << 7
}

#[inline]
pub fn propagate(mut gen : u64) -> u64 {
   let mut attacks = east_one(gen) | west_one(gen);
   gen    |= attacks;
   attacks    |= nort_one(gen) | sout_one(gen);
   return attacks;
}



