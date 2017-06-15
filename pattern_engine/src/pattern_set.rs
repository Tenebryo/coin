//use bson;

use Pattern;

pub struct PatternSet {
    patterns : Vec< Pattern >,
}

impl PatternSet {
    pub fn new() -> PatternSet {
        PatternSet {
            patterns: vec![],
        }
    }

    pub fn from_masks(masks : &[u64]) -> PatternSet {

        let patterns = (0..(masks.len())).map(|i| {
            Pattern::seed_random(masks[i])
        }).collect::<Vec<_>>();

        PatternSet {
            patterns : patterns
        }
    }

    ///load a pattern set from bson data.
    pub fn from_bson() -> PatternSet {
        PatternSet::new()
    }

    pub fn save_bson(&self) {

    }

    ///adds the scores of each of the patterns together and returns the final
    ///score. This should be tuned to the range [-640,640], representing the
    ///final position piece difference times 10 (for extra granularity).
    pub fn eval(&self, bits_b : u64, bits_w : u64) -> i16 {
        self.eval_raw(bits_b, bits_w).0.round() as i16
    }

    ///The _raw version does the same as eval, but returns the raw float.
    fn eval_raw(&self, bits_b : u64, bits_w : u64) -> (f32, [(u64,u64);8]) {

        use bitboard::bit_ops::all_board_syms;

        let syms = all_board_syms(bits_b, bits_w);

        // let score = self.patterns.iter()
        //     .fold(0f32, |acc, ref p| acc + syms.iter()
        //         .map(|&(bb, bw)| p.eval(bb, bw))
        //         .fold(0f32, |acc, ref x| acc + x)
        //     );

        let score = self.patterns.iter()
            .fold(0f32, |acc, ref p| acc
                + p.eval(syms[0].0, syms[0].1)
                + p.eval(syms[1].0, syms[1].1)
                + p.eval(syms[2].0, syms[2].1)
                + p.eval(syms[3].0, syms[3].1)
                + p.eval(syms[4].0, syms[4].1)
                + p.eval(syms[5].0, syms[5].1)
                + p.eval(syms[6].0, syms[6].1)
                + p.eval(syms[7].0, syms[7].1)
            );

        (score, syms)
    }

    ///Given a set of positions, adjust the weights of the patterns to fit the
    ///the data. For best results, the data should be shuffled before using.
    pub fn train(&mut self, training_data : &mut [(u64,u64,i16)], eta : f32) -> f64{
        let mut total_cost = 0.0f64;

        for i in 0..training_data.len() {
            let (score, syms) = self.eval_raw(training_data[i].0, training_data[i].1);
            let mut cost = score - training_data[i].2 as f32;
            cost *= cost;

            total_cost += cost as f64;

            for p in self.patterns.iter_mut() {
                for sym in syms.iter() {
                    p.adjust(sym.0, sym.1, eta * cost);
                }
            }
        }

        //return the average cost of the data
        total_cost / training_data.len() as f64
    }
}