use bson;

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
        self.eval_raw(bits_b, bits_w).round() as i16
    }

    ///The _raw version does the same as eval, but returns the raw float.
    fn eval_raw(&self, bits_b : u64, bits_w : u64) -> f32 {
        self.patterns.iter().fold(
            0f32, |acc, ref p| acc + p.eval(bits_b, bits_w)
        )
    }

    ///Given a set of positions, adjust the weights of the patterns to fit the
    ///the data. For best results, the data should be shuffled before using.
    pub fn train(&mut self, training_data : &mut [(u64,u64,i16)], eta : f32) -> f64{
        let mut total_cost = 0.0f64;

        for i in 0..training_data.len() {
            let score = self.eval_raw(training_data[i].0, training_data[i].1);
            let mut cost = score - training_data[i].2 as f32;
            cost *= cost;

            total_cost += cost as f64;

            for p in self.patterns.iter_mut() {
                p.adjust(training_data[i].0, training_data[i].1, eta * cost);
            }
        }

        //return the average cost of the data
        total_cost / training_data.len() as f64
    }
}