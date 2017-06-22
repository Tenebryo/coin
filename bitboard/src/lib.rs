#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(asm)]

pub mod bit_ops;
pub mod board;
mod find_moves_fast;
mod do_moves_fast;

pub use board::Board;
pub use board::Turn;
pub use board::Move;
pub use board::MoveList;
pub use board::MoveOrder;
pub use board::MAX_MOVES;


#[cfg(test)]
mod tests {
    extern crate rand;

    use bit_ops;

    use board::Board;
    use board::Turn;
    use board::Move;
    use board::MoveList;
    use board::MAX_MOVES;

    use std::time::Instant;

    use self::rand::Rng;

    #[test]
    fn it_works() {
        let mut b = Board::new();
        
        println!("{}", b);
        
        b.do_move(Move::new(3,2));
        
        println!("{}", b);
    }

    #[test]
    fn fast_find_moves_test() {
        for _ in 0..1024 {
            let mut b1 : Board = Board::new();
            let mut b2 : Board = Board::new();

            let mut rng = rand::thread_rng();

            assert!(b1 == b2);

            let mut t = Turn::BLACK;

            while !b1.is_done() {
                let mut moves : MoveList = [Move::null(); MAX_MOVES];

                let n = b1.get_moves(&mut moves);

                if n != 0 {
                    let m = moves[rng.gen::<usize>()%(n as usize)];

                    b1.do_move(m);
                    b1.update_moves();
                    b2.do_move(m);
                    b2.update_moves_fast();
                }

                assert!(b1 == b2);

                t = !t;
            }
        }
    }

    #[test]
    fn fast_do_move_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..1024 {
            let mut b1 : Board = Board::new();
            let mut b2 : Board = Board::new();


            assert!(b1 == b2);

            let mut t = Turn::BLACK;

            while !b1.is_done() {
                let mut moves : MoveList = [Move::null(); MAX_MOVES];

                let n = b1.get_moves(&mut moves);

                let bb1 = b1;
                let bb2 = b2;

                let mut f1 = 0;
                let mut f2 = 0;
                let mut m = Move::null();
                if n != 0 {
                    m = moves[rng.gen::<usize>()%(n as usize)];

                    f1 = b1.do_move(m);
                    b1.update_moves();
                    f2 = b2.f_do_move(m);
                    b2.update_moves_fast();
                }

                if (b1 != b2) {
                    println!("{}\n\n{}", bb1, bb2);
                    println!("{}\n\n{}", b1, b2);

                    println!("Move: {}", m);

                    bit_ops::print_bitboard(f1);
                    bit_ops::print_bitboard(f2);

                    println!("{:064b}\n{:064b}", bb2.pieces().0, bb2.pieces().1);
                }

                assert!(b1 == b2);

                t = !t;
            }
        }
    }

    #[test]
    fn get_moves_bench() {
        let b = Board::new();

        let iters = 100000;

        let t = bench(|| {
            let mut list : MoveList = [Move::null(); MAX_MOVES];
            b.get_moves(&mut list);
        }, iters);

        println!("get_moves: {} ns/iter", (t as f32/iters as f32));
    }

    #[test]
    fn update_moves_bench() {

        let iters = 100000;

        let t = bench(|| {
            let mut b = Board::new();
            b.update_moves();
        }, iters);

        println!("find_moves: {} ns/iter", (t as f32/iters as f32));
    }

    #[test]
    fn update_moves_fast_bench() {

        let iters = 100000;

        let t = bench(|| {
            let mut b = Board::new();
            b.update_moves_fast();
        }, iters);

        println!("fast_find_moves: {} ns/iter", (t as f64/iters as f64));
    }

    #[test]
    fn do_move_bench() {
        let iters = 100000;
        let t = bench(|| {
            let mut b = Board::new();
            b.do_move(Move::new(3,2));
        }, iters);
        println!("do_move: {} ns/iter", (t as f64/iters as f64));
    }

    #[test]
    fn f_do_move_bench() {
        let iters = 100000;
        let t = bench(|| {
            let mut b = Board::new();
            b.f_do_move(Move::new(3,2));
        }, iters);
        println!("f_do_move: {} ns/iter", (t as f64/iters as f64));
    }

    #[test]
    fn fast_do_move_bench() {
        let iters = 100000;
        let mut rng = rand::thread_rng();
        let p = rng.gen::<u64>();
        let mut o = rng.gen::<u64>();
        o &= !p;
        let m = rng.gen::<u8>();

        use do_moves_fast::fast_do_move;
        let t = bench(|| {
            fast_do_move(m, m & 7, (m >> 3) & 7, p, o);
        }, iters);
        
        println!("fast_do_move: {} ns/iter", (t as f64/iters as f64));
    }
    
    fn bench<F>(sample : F, iters : usize) -> u64 where F: Fn() {
        let now = Instant::now();
        for _ in 0..iters {
            sample();
        }
        let dur = now.elapsed();

        dur.as_secs() * 1_000_000_000u64 + dur.subsec_nanos() as u64
    }
}
