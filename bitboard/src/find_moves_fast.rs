
fn get_some_moves(P : u64, mask : u64, dir : u8) -> u64{
    let mut flip_l : u64 = 0; 
    let mut flip_r : u64 = 0;
    let mut mask_l : u64 = 0; 
    let mut mask_r : u64 = 0;
    let dir2 = dir << 1;
    let dir4 = dir << 2;

    flip_l  = P | (mask & (P << dir));    flip_r  = P | (mask & (P >> dir));
    mask_l  = mask & (mask << dir);       mask_r  = mask & (mask >> dir);
    flip_l |= mask_l & (flip_l << dir2);  flip_r |= mask_r & (flip_r >> dir2);
    mask_l &= mask_l << dir2;             mask_r &= mask_r >> dir2;
    flip_l |= mask_l & (flip_l << dir4);  flip_r |= mask_r & (flip_r >> dir4);

    ((flip_l & mask) << dir) | ((flip_r & mask) >> dir)
}

pub fn fast_find_moves(P : u64, O : u64) -> u64 {
    let mask = O & 0x7E7E7E7E7E7E7E7Eu64;

    (get_some_moves(P, mask, 1) // horizontal
    | get_some_moves(P, O, 8)   // vertical
    | get_some_moves(P, mask, 7)   // diagonals
    | get_some_moves(P, mask, 9))
    & !(P|O) // mask with empties
}