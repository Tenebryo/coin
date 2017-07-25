extern crate bitboard;
extern crate heuristic;
extern crate search;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;

#[macro_use]
pub mod common;

pub mod player;
pub mod opening;

use std::env;
use std::io;
use std::process;

use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;

use player::Player;


fn main() {
    
    let mut t = match env::args().nth(1).unwrap_or_else(||{
        cerrln!("Usage: ./coin [Black|White]");
        process::exit(0)
    }).as_ref() {
        "Black" => Turn::BLACK,
        "White" => Turn::WHITE,
        _ => {
            cerrln!("Usage: ./coin [Black|White]");
            process::exit(0)
        }
    };
    
    
    let mut ms_left = 0;
    let mut b = Board::new();

    if t == Turn::BLACK {
        b.f_do_move(Move::pass());
    }
    
    cerrln!("{}", b);
    
    let mut p = Player::new(t);
    
    println!("Init done");
    
    while !b.is_done() {
    
        //wait for opponent move first
        let mut x_inp = String::new();
        match io::stdin().read_line(&mut x_inp) {
            Ok(_) => {
                let mut n = x_inp.find(char::is_whitespace).unwrap();
                let mut y_inp = x_inp.split_off(n).split_off(1);
                n = y_inp.find(char::is_whitespace).unwrap();
                let mut ms_inp = y_inp.split_off(n).split_off(1);
                ms_inp = ms_inp.trim().to_string();
                
                let m = match (
                    x_inp.parse::<i8>().unwrap(),
                    y_inp.parse::<i8>().unwrap()
                ) {
                    (-1,-1) => Move::pass(),
                    (x,y) => Move::new(x as u8,y as u8)
                };
                
                ms_left = ms_inp.parse::<u64>().unwrap();
                
                cerrln!("[COIN]: Opponent moved {} <{} ms>", m, ms_left);
                
                b.f_do_move(m);
            },
            Err(e) => {
                panic!(e)
            }
        }
        cerrln!("\n{}", b);
        
        // make my move
        let m = p.do_move(b.copy(), ms_left);
        
        b.f_do_move(m);
        
        if m.is_null() {
            println!("-1 -1");
        } else if m.is_pass() {
            println!("-1 -1");
        } else {
            println!("{} {}", m.x(), m.y());
        }
        
        cerrln!("\n{}", b);
    }
    
    cerrln!("RESULT: {}/{}", b.count_pieces().1, b.count_pieces().0);
}
