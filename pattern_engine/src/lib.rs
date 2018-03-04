#![feature(asm, iterator_step_by)]
extern crate bson;
extern crate rand;
extern crate bitboard;
extern crate heuristic;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod pattern_lut;
mod pattern;
mod pattern_set;
mod twos_to_threes_lut;
mod staged_heuristic;

pub use pattern_lut::PatternLUT;
pub use pattern::Pattern;
pub use pattern_set::PatternSet; 
pub use staged_heuristic::StagedHeuristic;

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rand;

    use pattern_set::PatternSet;
    use pattern_lut::PatternLUT;
    use pattern::Pattern;

    use std::time::Instant;

    #[test]
    fn patternlut_test() {
        let mut rng = rand::thread_rng();
        let mut mask = 0;
        for _ in 0..8 {
            loop {
                let j = rng.gen::<u64>()%64;
                if ((1<<j) & mask) == 0 {
                    mask |= 1<<j;
                    break;
                }
            }
        }
        PatternLUT::from_mask(mask);
    }

    #[test]
    fn pattern_bench() {
        let mut rng = rand::thread_rng();
        let mut mask = 0;
        for _ in 0..8 {
            loop {
                let j = rng.gen::<u64>()%64;
                if ((1<<j) & mask) == 0 {
                    mask |= 1<<j;
                    break;
                }
            }
        }
        
        use bitboard::bit_ops::popcount_64;

        let testb = rng.gen::<u64>();
        let mut testw = rng.gen::<u64>();

        testw &= !testb;


        let p = Pattern::seed_random(mask);

        let iters = 10000;

        let now = Instant::now();
        for _ in 0..iters {
            p.eval(testb,testw);
        }
        let dur = now.elapsed();

        let total_time = dur.as_secs() * 1_000_000_000u64 + dur.subsec_nanos() as u64;

        println!("Pattern::eval\t\t\t\t{:?} ns/iter", total_time as f32/iters as f32);

    }

    #[test]
    fn patternset_test() {
        let mut rng = rand::thread_rng();

        let masks : Vec<u64> = (0..12).map(|_| {
            let mut mask = 0;
            for _ in 0..8 {
                loop {
                    let j = rng.gen::<u64>()%64;
                    if ((1<<j) & mask) == 0 {
                        mask |= 1<<j;
                        break;
                    }
                }
            }
            mask
        }).collect::<Vec<_>>();

        let p = PatternSet::from_masks(&masks);

        let testb = rng.gen::<u64>();
        let mut testw = rng.gen::<u64>();

        testw &= !testb;

        let iters = 10000;

        let now = Instant::now();
        for _ in 0..iters {
            p.eval(testb,testw);
        }
        let dur = now.elapsed();

        let total_time = dur.as_secs() * 1_000_000_000u64 + dur.subsec_nanos() as u64;

        println!("PatternSet::eval\t\t\t{:?} ns/iter", total_time/iters);
    }
}