use bitboard::*;

pub type EvalInput  = [Board;1];
pub type EvalOutput = ([f32;64],f32);

pub trait Evaluator: Sized + Clone {
    fn evaluate(&self, input : &EvalInput) -> EvalOutput;
    fn train(&self, input : &EvalInput, target : &EvalOutput);
}