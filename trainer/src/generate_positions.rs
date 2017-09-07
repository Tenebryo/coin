use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::empty_movelist;

use search::NegamaxSearch;
use search::TranspositionTable;
use heuristic::Heuristic;

use TrainingPosition;

use rand;
use rand::Rng;
use std::i32;
use std::time::Instant;
use std::cell::RefCell;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn get_random_position(empty : u8) -> Board {
    let mut b = Board::new();
    let mut mvl = empty_movelist();
    let mut rng = rand::thread_rng();
    
    while b.total_empty() > empty {
        if b.is_done() {
            b = Board::new();
        }

        let n = b.get_moves(&mut mvl);

        let m = mvl[rng.gen::<usize>() % n as usize];

        b.f_do_move(m);
    }
    b
}


pub fn generate_positions<H: 'static + Heuristic + Clone + Send>(
    num_threads             : usize, 
    positions_per_thread    : usize, 
    positions               : &mut Vec<TrainingPosition>,
    heuristic               : &mut H,
    empty_min               : u8,
    empty_max               : u8,
    step                    : u8,
) {
    let mut rng = rand::thread_rng();
    //simple training, generate random game scenario.

    positions.reserve_exact(positions_per_thread * num_threads);

    let (tx, rx) = channel();
    let threads_running = Arc::new(AtomicUsize::new(0));

    for t in 0..num_threads {

        let ctx = tx.clone();
        let threads_running = threads_running.clone();
        let mut heuristic = (*heuristic).clone();

        threads_running.fetch_add(1, Ordering::SeqCst);


        thread::spawn( move || {

            println!("Thread {} starting...", t);
            let mut ng = NegamaxSearch::new(Box::new(heuristic), Instant::now());
            let mut rng = rand::thread_rng();

            for i in 0..positions_per_thread {

                ng.clear_transpositions();
                let mut p = get_random_position(rng.gen_range::<u8>(empty_min, empty_max));

                let mut m = Move::null();

                let score = ng.negamax_opt(p.copy(), i32::MIN + 1, i32::MAX - 1, 
                                            step);

                //positions.push((p1, score as i16, m));

                if Err(e) = ctx.send(TrainingPosition::new(p, m, score as i16)) {
                    break;
                }
                //println!("{}", p1);
            }
            threads_running.fetch_sub(1, Ordering::SeqCst);
            println!("Thread {} stopping...", t);
        });
    }

    let mut i = 0;
    while threads_running.load(Ordering::SeqCst) > 0 {
        let t = rx.recv().unwrap();
        

        if i % 1000 == 0 {
            print!("Finished {: >10} positions.\r", i);
        }
        
        positions.push(t);
        i+=1;
    }
    println!("");
}