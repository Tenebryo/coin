use PatternLUT;

///A set of scores associated with each possible stone configuration for a 
///static set of squares.
pub struct Pattern {
    //a mask representing the pattern squares
    //mask    : u64,
    //the scores associated with each square, indexed base 3
    scores  : Box<[f32]>,
    //the number of occurances of each stone in the training data to filter
    //rare patterns.
    occurs  : Box<[i16]>,
    //a LUT to quickly index into the score table.
    poslut  : PatternLUT,
}

impl Pattern {
    ///Create a pattern with the given mask and score vector
    pub fn new(mask : u64, scores : Box<[f32]>) -> Pattern {
        let poslut = PatternLUT::from_mask(mask);
        let occurs = (0..(scores.len()))
            .map(|_| 0i16)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Pattern {
            /*mask,*/ 
            scores, 
            occurs, 
            poslut
        }
    }

    ///Create a randomly seeded pattern.
    pub fn seed_random(mask : u64) -> Pattern {
        use bitboard::bit_ops::popcount_64;
        use rand::Rng;
        use rand;

        let nbits = popcount_64(mask);
        let n = 3usize.pow(nbits as u32);
        let mut data = Vec::with_capacity(n);

        let mut rng = rand::thread_rng();

        for _ in 0..n {
            data.push(rng.gen::<f32>());
        }

        Pattern::new(mask, data.into_boxed_slice())
    }

    ///load the pattern lookup table from a BSON document.
    pub fn load_from_bson() -> PatternLUT {
        PatternLUT::from_mask(0)
    }

    ///Save the pattern to a BSON document.
    pub fn save_to_bson(&self) /*-> bson::Document */{

    }

    ///Evaluate the score for a pattern on a specific bitboard.
    pub fn eval(&self, bits_b : u64, bits_w : u64) -> f32 {
        self.scores[self.poslut.eval(bits_b, bits_w)]
    }

    ///Adjust the pattern scores for a specific board given a gradient.
    pub fn adjust(&mut self, bits_b : u64, bits_w : u64, amount : f32) {
        let i = self.poslut.eval(bits_b, bits_w);
        self.scores[i] -= amount;
        self.occurs[i] += 1;
    }
}