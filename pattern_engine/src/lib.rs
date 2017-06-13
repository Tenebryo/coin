#![feature(test)]
extern crate bson;
extern crate rand;
extern crate test;
extern crate bitboard;

mod pattern_lut;
mod pattern;
mod pattern_set;

pub use pattern_lut::PatternLUT;
pub use pattern::Pattern;
pub use pattern_set::PatternSet; 

#[cfg(test)]
mod tests {
    use pattern_lut::PatternLUT;
    use test;
    #[test]
    fn it_works() {
        test::black_box(PatternLUT::from_mask(0b00000000_00000000_00000000_00011000_00000000_00100010_00001110_00000000));
    }
}

#[cfg(test)]
mod bench {

    use pattern_lut::PatternLUT;
    use pattern::Pattern;
    use test::Bencher;
    use test;

    use rand::Rng;
    use rand;

    #[bench]
    fn pattern_lut_gen_bench(b: &mut Bencher) {
        b.iter(||{
            PatternLUT::from_mask(0b00000000_00000000_00000000_00011000_00000000_00100010_00001110_00000000)
        });
    }
    #[bench]
    fn pattern_eval_bench(b: &mut Bencher) {

        const SCORE_SIZE : usize = 6561; //2187

        let mut scores = vec![0f32; SCORE_SIZE].into_boxed_slice();

        let mut rng = rand::thread_rng();
        for i in 0..SCORE_SIZE {
            scores[i] = rng.gen::<f32>();
        }

        let mut mask : u64 = 0;

        for _ in 0..8 {
            loop {
                let j = rng.gen::<u64>()%64;
                if ((1<<j) & mask) == 0 {
                    mask |= 1<<j;
                    break;
                }
            }
        }


        let p = Pattern::seed_random(mask);

        let testb = rng.gen::<u64>();
        let mut testw = rng.gen::<u64>();

        testw &= !testb;

        println!("\nMask: {:64b}\nTest Bitboard: {:64b}:{:64b}", mask, testb, testw);

        b.iter(||{
            test::black_box(p.eval(testb,testw));
        });
    }
}