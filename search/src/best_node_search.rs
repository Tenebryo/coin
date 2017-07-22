use heuristic::Heuristic;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::empty_movelist;

use transposition::TranspositionTable;
use negamax_ab_timeout;

fn bns_next_guess(alpha : i32, beta : i32, subtree_count : i32) -> i32 {
    alpha + ((beta-alpha) * (subtree_count - 1))/subtree_count
}

pub fn bns_timeout<H : Heuristic>(
    ttbl    : &mut TranspositionTable, 
    bb      : Board, 
    h       : &mut H,
    mut g   : i32,
    d       : u8,
    ms_left : u64,
    start   : Instant,
    to_flag : &mut bool
) -> Move {

    let mut best_move = Move::pass();
    let mut low = i32::MIN;
    let mut high = i32::MAX;

    let mut better = 0;

    let mut best_move = Move::null();

    let mut tt = TranspositionTable::new(10_000_000);

    let mut alpha = i32::MIN + 1;
    let mut beta = i32::MAX - 1;

    //get moves
    let mut mvl = empty_movelist();
    let n = bb.get_moves(&mut mvl);

    let mut subtree_count = n;

    //black magic rust do-while loop
    while {
        let test = bns_next_guess(alpha, beta, subtree_count);

        better = 0;

        for i in 0..n {
            let bc = bb.copy();
            bc.do_move(mvl[i]);
            let refute = negamax_ab_timeout(ttbl, bb, h, -test, -(test-1), -1, d, 5, 
                               ms_left, start, to_flag, &mut Move::null))

            if refute >= test {
                better += 1;
                best_move = mvl[i];
            }
        }


        if better > 1 {
            beta += test - alpha;
            alpha = test;
            subtree_count == better;
        } else if better == 0 {
            beta -= 100;
        }


        !((beta-alpha < 2) || (better == 1))
    } {}

    best_move
}