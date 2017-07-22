use bitboard::Board;
use pattern_engine::PatternSet;

use TrainingPosition;

use rand;
use rand::Rng;
use std::cmp::min;

use serde_json;

pub fn train_pattern(
    patterns    : &mut PatternSet,
    epochs      : usize,
    epoch_start : usize,
    batch_size  : usize,
    tdata       : &Vec<TrainingPosition>,
    eta         : f32,
) -> f32 {

    ///cost function is F(x) = (p(x) - b)^2, so the gradient function is
    /// vF(x) = (p(x) - b) (d/d x_i)((p(x) - b))

    let num_batches = ((tdata.len()-1) / batch_size) + 1;
    let mut total_cost = 0.0;

    for epoch in 0..epochs {

        total_cost = 0.0;

        for batch in 0..num_batches {
            let first = batch * batch_size;
            let last  = min(tdata.len(), (batch + 1) * batch_size);
            let bdata = &tdata[first..last];

            let mut syms = vec![];

            let mut cost : f32 = 0.0;
            for bi in 0..(last-first) {

                let (c, sym) = patterns.eval_raw(bdata[bi].ps,bdata[bi].os);
                cost += c  - (bdata[bi].sc as f32);

                syms.push(sym);
            }


            cost /= (last-first) as f32;

            // println!("Cost = {:10?}", cost);

            for bi in 0..(last-first) {
                patterns.adjust(syms[bi], eta*cost);
            }

            total_cost += cost*cost;
        }

        total_cost /= num_batches as f32;

        println!("Epoch {:3} complete. Average cost = {:10.2}.", epoch+epoch_start, total_cost);

    }

    total_cost
}