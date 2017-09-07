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

            for (sym, cost) in (0..(last-first)).map(|bi| {

                let (c, sym) = patterns.eval_raw(bdata[bi].ps, bdata[bi].os);
                let cost = c  - (bdata[bi].sc as f32);

                total_cost += cost*cost;

                (sym, cost)
            }).collect::<Vec<_>>() {
                patterns.adjust(sym, eta*cost);
            }
        }

        total_cost /= tdata.len() as f32;

        println!("Epoch {:4} complete. Average cost = {:10.2}. Eta = {}", epoch+epoch_start, total_cost, eta);

    }

    total_cost
}