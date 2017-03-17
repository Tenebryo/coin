extern crate bitboard;

#[macro_use]
pub mod common;

pub mod heuristic;
pub mod search;

use std::env;
use std::io;
use std::process;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;

use search::SearchEngine;
use heuristic::HPattern;


fn main() {
    
    let mut color = match env::args().nth(1).unwrap_or_else(||{
        cerrln!("Usage (TODO)");
        process::exit(0)
    }).as_ref() {
        "Black" => Turn::BLACK,
        "White" => Turn::WHITE,
        _ => {
            cerrln!("Usage (TODO)");
            process::exit(0)
        }
    };
    
    
    
    let mut b = Board::new();
    
    cerrln!("{}", b);
    
    let mut t = Turn::BLACK;
    let mut se = SearchEngine::new(t);
    let mut hb = HPattern{};
    let mut ms_left = 300_000;
    
    println!("Init done");
    
    while !b.is_done() {
        
        if t == color {
            let m = se.mtdf_id(
                b.copy(), &mut hb, t, 21, 10000
            );
            b.do_move(t, m);
            
            if m.is_null() {
                println!("-1 -1");
            } else if m.is_pass() {
                println!("-1 -1");
            } else {
                println!("{} {}", m.x()+1, m.y()+1);
            }
        } else {
            
            let mut x_inp = String::new();
            match io::stdin().read_line(&mut x_inp) {
                Ok(n) => {
                    let mut y_inp = x_inp.split_off(1).split_off(1);
                    let mut ms_inp = y_inp.split_off(1).split_off(1);
                    ms_inp = ms_inp.trim().to_string();
                    
                    cerrln!("'{}', '{}', '{}'", x_inp, y_inp, ms_inp);
                    
                    let m = Move::new(
                        x_inp.parse::<u8>().unwrap()-1,
                        y_inp.parse::<u8>().unwrap()-1
                    );
                    
                    ms_left = ms_inp.parse::<u64>().unwrap();
                    
                    b.do_move(t, m);
                },
                Err(e) => {
                    panic!(e)
                }
            }
            
        }
        
        
        t = !t;
        cerrln!("{}", b);
    }
    
    cerrln!("RESULT: {}/{}", b.count_pieces(Turn::BLACK), b.count_pieces(Turn::WHITE));
}
