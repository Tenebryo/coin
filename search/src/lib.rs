extern crate bitboard;
extern crate heuristic;

mod negamax_ab_timeout;
mod mtdf_id_timeout;
mod transposition;

pub use negamax_ab_timeout::negamax_ab_timeout;
pub use mtdf_id_timeout::{mtdf_timeout, mtdf_id_timeout};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
