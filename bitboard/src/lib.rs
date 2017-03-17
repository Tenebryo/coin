pub mod bit_ops;
pub mod board;

pub use board::Board;
pub use board::Turn;
pub use board::Move;
pub use board::MoveList;
pub use board::MoveOrder;
pub use board::MAX_MOVES;


#[cfg(test)]
mod tests {
    use board::Board;
    use board::Turn;
    use board::Move;
    
    #[test]
    fn it_works() {
        let mut b = Board::new();
        
        println!("{}", b);
        
        b.do_move(Turn::BLACK, Move::new(3,2));
        
        println!("{}", b);
    }
}
