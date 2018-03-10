#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(asm)]

#[macro_use]
extern crate serde_derive;

pub mod bit_ops;
pub mod board;
mod find_moves_fast;
// mod do_moves_fast;
mod do_moves_faster;

pub use board::Board;
pub use board::Position;
pub use board::Turn;
pub use board::Move;
pub use board::MoveList;
pub use board::MoveOrder;
pub use board::MAX_MOVES;
pub use board::empty_movelist;
pub use board::empty_moveorder;


#[cfg(test)]
mod tests {
    extern crate rand;

    use bit_ops;

    use board::Board;
    use board::Turn;
    use board::Move;
    use board::MoveList;
    use board::MAX_MOVES;
    use board::empty_movelist;

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

        let mut rng = rand::thread_rng();
        let mut bs = (0..iters).map(|_| {
            Board::position(rng.gen::<u64>(), rng.gen::<u64>(), Turn::BLACK)
        }).collect::<Vec<_>>();


        let mut i = 0;
        let mut sum = 0;
        let t = bench(|| {
            bs[i].update_moves_fast();
            i += 1;
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

    #[test]
    fn simulate_bench() {
        let iters = 100000;
        let mut rng = rand::thread_rng();

        let rands = (0..60_000_000).map(|_| rng.gen::<u8>()).collect::<Vec<_>>();

        let mut i = 0;
        let mut test_fn = || {
            let mut b = Board::new();
            
            let mut mvs = empty_movelist();
            let mut e = b.total_empty();
            let mut r = rand::thread_rng();

            while e > 0 {
                let n = b.get_moves(&mut mvs);
                b.do_move(mvs[(rands[i] % n) as usize]);
                e -= 1;
                i += 1;
            }
            while !b.is_done() {
                let n = b.get_moves(&mut mvs);
                b.do_move(mvs[(rands[i] % n) as usize]);
                i += 1;
            }
        };

        let t = bench(test_fn, iters);

        let mut ns = (t as f64/iters as f64);

        println!("fast_do_move: {:.2} ns/iter ({:.2} iter/s)", ns, 1_000_000_000.0/ns);
    }

    #[test]
    fn test_fast_min() {
        use do_moves_fast::fast_min;

        println!("{}", fast_min(0,5));
        println!("{}", fast_min(5,2));
        println!("{}", fast_min(100,5));
        println!("{}", fast_min(2,7));
        println!("{}", fast_min(2,2));

        assert!(fast_min(0,5) == 0);
        assert!(fast_min(5,5) == 5);
        assert!(fast_min(100,5) == 5);
        assert!(fast_min(2,7) == 2);
    }

    fn explore_tree(b : Board, depth : usize) -> usize {

        if depth == 0 {
            return 1;
        }

        let mut mvs = empty_movelist();
        let n = b.get_moves(&mut mvs);

        let mut sum = 0;
        for i in 0..n {
            let mut bc = b;
            bc.f_do_move(mvs[i as usize]);

            sum += explore_tree(bc, depth-1);
        }

        return sum;
    }

    #[test]
    fn perft_11_bench() {
        let now = Instant::now();

        let n = explore_tree(Board::new(), 11);

        let dur = now.elapsed();

        let elapsed = dur.as_secs() as f64 + dur.subsec_nanos() as f64 / 1_000_000_000.0f64;
        println!("PERFT 11 time: {} s", elapsed);
        println!("PERFT 11 result: {} moves", n);
    }

    fn bench<F>(mut sample : F, iters : usize) -> u64 where F: FnMut() {
        let now = Instant::now();
        for _ in 0..iters {
            sample();
        }
        let dur = now.elapsed();

        dur.as_secs() * 1_000_000_000u64 + dur.subsec_nanos() as u64
    }
}
