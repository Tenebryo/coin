use rand::{self, Rng};
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::io::stdout;
use std::fs;
use std::fs::File;

use std::mem;
use std::error::Error;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::*;
use std::sync::atomic::*;

use std::time::Instant;
use std::time::Duration;

use bincode;
use serde_json;

use threadpool::*;
use scoped_threadpool::Pool;
use rayon::prelude::*;

use bitboard::*;
use eval::*;
use mcts::*;
use game::*;

// const EVAL_ROUNDS : usize = 100;
// const EVAL_GAMES : usize = 4;
// const EVAL_CUTOFF : usize = 3;
// const EVAL_RANDOM : usize = 4;
// const TRAINING_ITERATIONS : usize = 128;
// const TRAINING_BATCH_SIZE : usize = 128;
// const GAME_HISTORY_LENGTH : usize = 10_000;
// const GAME_BATCHES_PER_ROUND : usize = 5;
// const SELF_PLAY_VARIANCE_TURNS : usize = 15;

const EVAL_ROUNDS : usize = 400;
const EVAL_GAMES : usize = TF_EVAL_BATCH_SIZE;
const EVAL_CUTOFF : usize = TF_EVAL_BATCH_SIZE * 70 / 128;
const EVAL_RANDOM : usize = TF_EVAL_BATCH_SIZE;
const TRAINING_ITERATIONS : usize = 512;
const TRAINING_BATCH_SIZE : usize = 1024;
const GAME_HISTORY_LENGTH : usize = 65_536;
const GAME_BATCHES_PER_ROUND : usize = 16;
const SELF_PLAY_VARIANCE_TURNS : usize = 10;

pub struct MctsTrainer<'a> {
    best         : usize,
    players      : Vec<(Arc<ParallelCoinNetWorker>, MctsTree<ParallelCoinNet>)>,
    recent_games : Vec<Game>,
    last_game    : usize,
    data_folder  : &'a Path,
}

impl<'a> MctsTrainer<'a> {
    pub fn new(n : usize, data_folder : &'a Path, model : &Path, vars : Option<&Path>) -> MctsTrainer<'a> {
        let players = (0..n).map(|_| {

            let (evals, worker) = ParallelCoinNet::new_worker_group(model, vars).unwrap();

            let evals = MctsTree::new(evals);

            (worker, evals)
        }).collect::<Vec<_>>();

        MctsTrainer {
            best            : 0,
            players,
            recent_games    : vec![],
            last_game       : 0,
            data_folder     : data_folder,
        }
    }

    /// Evaluate all the players against the best player, updating the best
    /// player if one surpasses it.
    fn evaluate_players(&mut self) {
        let start = Instant::now();

        for i in 0..(self.players.len()) {
            if i != self.best {
                self.eval_player(i);
            }
        }

        let elapsed = start.elapsed();
        println!("[COIN]         Time elapsed: {:5}.{:09}", elapsed.as_secs(), elapsed.subsec_nanos());

    }

    fn play_random(&mut self) {
        let start = Instant::now();
        let mut tpool = Pool::new(TF_EVAL_BATCH_SIZE as u32 + 1);

        let rwins = Arc::new(AtomicUsize::new(0));
        let running = Arc::new(AtomicUsize::new(0));

        println!("\r[COIN]     Playing Random Games...");
        tpool.scoped(|tpool| {

            let (ref worker, ref evals) = self.players[self.best];

            let mut par = true;
            for _i in 0..TF_EVAL_BATCH_SIZE {
                let rwins = rwins.clone();
                let running = running.clone();
                running.fetch_add(1, Ordering::SeqCst);
                //schedule a game on the threadpool. # of games >>> # of threads,
                //since the bottleneck is the TF evaluations

                let p1 = evals.clone();

                tpool.execute(move || {
                    let r = Self::random_game(p1, par);
                    if r < 0.0 {
                        rwins.fetch_add(1, Ordering::SeqCst);
                    }
                    running.fetch_sub(1, Ordering::SeqCst);
                });
                par = !par;
            }

            let mut p1_steps = 0;
            while running.load(Ordering::SeqCst) != 0 {
                p1_steps += worker.do_a_work();
                print!("\r[COIN]         Total Evals: [{:8}] ({:4.1}%)", 
                    p1_steps, p1_steps as f32 / (0.3 * (TF_EVAL_BATCH_SIZE * EVAL_ROUNDS) as f32));
                stdout().flush().unwrap();
            }
            println!("");
        });

        let rwins = rwins.load(Ordering::SeqCst);

        let elapsed = start.elapsed();
        println!("[COIN]       Best player random winrate: {}", rwins as f32 / EVAL_RANDOM as f32);
        println!("[COIN]         Time elapsed: {:5}.{:09}", elapsed.as_secs(), elapsed.subsec_nanos());
    }

    /// Evaluate the idx player against the best player */
    fn eval_player(&mut self, idx : usize) {
        let start = Instant::now();
        println!("[COIN]     Evaluating Player {} against {}", idx, self.best);

        let mut tpool = Pool::new(TF_EVAL_BATCH_SIZE as u32 + 1);

        let wins = Arc::new(AtomicUsize::new(0));
        let running = Arc::new(AtomicUsize::new(0));

        // let worker_p1 = &self.players[self.best].0;
        // let worker_p2 = &self.players[idx].0;

        tpool.scoped(|tpool| {

            let (ref worker1, ref eval1) = self.players[self.best];
            let (ref worker2, ref eval2) = self.players[idx];
            
            let mut par = true;
            for _i in 0..TF_EVAL_BATCH_SIZE {
                let wins = wins.clone();
                let running = running.clone();
                running.fetch_add(1, Ordering::SeqCst);
                //schedule a game on the threadpool. # of games >>> # of threads,
                //since the bottleneck is the TF evaluations
                let (p1, p2) = (eval1.clone(), eval2.clone());

                if par {
                    tpool.execute(move || {
                        let r = Self::eval_game(p1, p2);
                        if r < 0.0 {
                            wins.fetch_add(1, Ordering::SeqCst);
                        }
                        running.fetch_sub(1, Ordering::SeqCst);
                    });
                } else {
                    tpool.execute(move || {
                        let r = Self::eval_game(p2, p1);
                        if r > 0.0 {
                            wins.fetch_add(1, Ordering::SeqCst);
                        }
                        running.fetch_sub(1, Ordering::SeqCst);
                    });
                }
                par = !par;
            }


            // tpool.execute(|| {
            //     while running.load(Ordering::SeqCst) != 0 {
            //         worker_p1.do_a_work();
            //     }
            // });
            // tpool.execute(|| {
            //     while running.load(Ordering::SeqCst) != 0 {
            //         worker_p2.do_a_work();
            //     }
            // });

            let mut p1_steps = 0;
            let mut p2_steps = 0;
            while running.load(Ordering::SeqCst) != 0 {
                p1_steps += worker1.do_a_work();
                p2_steps += worker2.do_a_work();
                print!("\r[COIN]         Total Evals: [P1,P2] = [{:8}, {:8}] ({:4.1}%)", 
                    p1_steps, p2_steps, (p1_steps + p2_steps) as f32 / (0.6 * (TF_EVAL_BATCH_SIZE * EVAL_ROUNDS) as f32));
                stdout().flush().unwrap();
            }
            println!("");
        });
        // tpool.join();

        // tpool.join();

        let wins = wins.load(Ordering::SeqCst);

        let elapsed = start.elapsed();
        println!("\r[COIN]       Result: {} to {}", wins, EVAL_GAMES  - wins);
        println!("\r[COIN]         Time elapsed: {:5}.{:09}", 
            elapsed.as_secs(), elapsed.subsec_nanos());

        if wins >= EVAL_CUTOFF {
            self.best = idx;
        }
    }

    /// Play an evaluation game (i.e. using best guess instead of weighted guess
    /// to select moves with mcts). */
    fn eval_game(mut p1 : MctsTree<ParallelCoinNet>, mut p2 : MctsTree<ParallelCoinNet>) -> f32 {
        let mut rng = rand::thread_rng();
        let mut b = Board::new();

        let mut turn = 1.0;
        // let mut t = 1;

        /*  Initialize each player for the game. */
        p1.set_position(b);
        p2.set_position(b);
        p1.set_temp(0.25);
        p2.set_temp(0.25);

        /*  Simulate the game. */
        while !b.is_done() {

            let mut moves = empty_movelist();
            let n = b.get_moves(&mut moves);

            let mut selected_move = Move::pass();

            if n != 0 {
                let val = if turn > 0.0 {
                    p1.n_rounds(EVAL_ROUNDS);
                    p1.evaluate(&b)
                } else {
                    p2.n_rounds(EVAL_ROUNDS);
                    p2.evaluate(&b)
                };


                let sum : f32 = (0..(n as usize)).map(|i| val.0[moves[i].offset() as usize]).sum();

                if (sum < 0.0) {
                    eprintln!("[COIN]       Probable Overflow {}", line!());
                }

                let rmove = rng.gen_range(0.0, sum);

                let mut mi = 0;
                let mut sm = 0.0;

                for i in 0..(n as usize) {
                    sm += val.0[moves[i].offset() as usize];
                    if rmove <= sm {
                        mi = i;
                        break;
                    }
                }

                selected_move = moves[mi];
            }

            /*  Apply the move to the board and each of the players. */
            b.f_do_move(selected_move);

            p1.prune(selected_move);
            p2.prune(selected_move);

            turn = -turn;
        }

        turn * b.piece_diff() as f32 / 64.0
    }

    /// Play a game against a random player. */
    fn random_game(mut p1 : MctsTree<ParallelCoinNet>, start : bool) -> f32 {
        let mut rng = rand::thread_rng();
        let mut b = Board::new();

        let mut turn = 1.0;

        if start {
            turn = -turn;
        }

        /*  Initialize each player for the game. */
        p1.set_position(b);
        p1.set_temp(0.25);

        /*  Simulate the game. */
        while !b.is_done() {
            let mut moves = empty_movelist();
            let n = b.get_moves(&mut moves);

            let mut selected_move = Move::pass();

            if n != 0 {
                if turn > 0.0 {
                    selected_move = match rng.choose(&moves[..(n as usize)]) {
                        Some(&m) => m,
                        None => Move::pass(),
                    };
                } else {
                    p1.n_rounds(EVAL_ROUNDS);
                    let val = p1.evaluate(&b);

                    let sum : f32 = (0..(n as usize)).map(|i| val.0[moves[i].offset() as usize]).sum();

                    if (sum < 0.0) {
                        eprintln!("[COIN]       Probable Overflow {}", line!());
                    }

                    let rmove = rng.gen_range(0.0, sum);

                    let mut mi = 0;
                    let mut sm = 0.0;

                    for i in 0..(n as usize) {
                        sm += val.0[moves[i].offset() as usize];
                        if rmove <= sm {
                            mi = i;
                            break;
                        }
                    }

                    selected_move = moves[mi];
                }
            }

            /*  Apply the move to the board and each of the players. */
            b.f_do_move(selected_move);

            p1.prune(selected_move);

            turn = -turn;
        }

        turn * b.piece_diff() as f32 / 64.0
    }

    /// Train all the non-best . */
    fn training_round(&mut self, eta : f32) {
        for i in 0..(self.players.len()) {
            self.train_player(i, eta);
            // if i != self.best {
            // }
        }
    }

    /// Trains a player on TRAINING_ITERATIONS batches. */
    fn train_player(&mut self, idx : usize, eta : f32) {
        println!("[COIN]     Training Player {}", idx);
        let mut rng = rand::thread_rng();
        /*  do many times. */
        let mut mean_err = 0.0;
        for i in 0..TRAINING_ITERATIONS {

            /*  Build a mini-batch. */
            let mut input = vec![];
            let mut output = vec![];

            // seems like a bottleneck (probably premuting)
            while input.len() < TRAINING_BATCH_SIZE {
                let selected_game = rng.choose(&self.recent_games).unwrap();
                let selected = rng.choose(&selected_game.states);

                match selected {
                    Some(s) => {
                        /*  make sure we don't care about passes... */
                        if s.0.mobility().0 != 0 {
                            let perm = rng.gen::<usize>() % 8;

                            let mut t_in = s.0.clone();
                            // t_in.permute(perm);
                            
                            let mut t_out = s.1.clone();
                            t_out.1 = t_out.1.signum();
                            // t_out.permute(perm);

                            input.push(t_in);
                            output.push(t_out);
                        }
                    },
                    None => {}
                }
            }

            let perms = (0..TRAINING_BATCH_SIZE)
                .map(|_| rng.gen::<usize>() % 8)
                .collect::<Vec<_>>();

            let input = input.par_iter()
                .enumerate()
                .map(|(i, &t_in)| {t_in.permute(perms[i]); t_in})
                .collect::<Vec<_>>();

            let output = output.par_iter()
                .enumerate()
                .map(|(i, &t_out)| {t_out.permute(perms[i]); t_out})
                .collect::<Vec<_>>();

            /*  train on the mini-batch: */
            let err = self.players[idx].0.train(&input, &output, eta);

            mean_err += err;

            print!("[COIN]       Epoch {} (loss={})\r", i, err);
            io::stdout().flush().unwrap();
        }
        println!("[COIN]       Mean Loss = {}", mean_err / TRAINING_ITERATIONS as f32);
    }

    /// Generates GAME_BATCHES_PER_ROUND new self-play games. */
    fn self_play(&mut self, iter : usize) {
        // let mut new_games = Arc::new(Mutex::new(vec![]));

        let (ng_tx, ng_rx) = channel();

        let p = self.best;

        let mut tpool = Pool::new(TF_EVAL_BATCH_SIZE as u32);

        for r in 0..GAME_BATCHES_PER_ROUND {

            let start = Instant::now();

            println!("\r[COIN]     Generating Self-Play Games... (R{})", r);
            let running = Arc::new(AtomicUsize::new(0));
            
            tpool.scoped(|tpool| {

                let (ref worker, ref eval) = self.players[p];

                for _i in 0..TF_EVAL_BATCH_SIZE {
                    // let new_games = new_games.clone();
                    let running = running.clone();
                    let p1 = eval.clone();
                    running.fetch_add(1, Ordering::SeqCst);
                    //schedule a game on the threadpool. # of games >>> # of threads,
                    //since the bottleneck is the TF evaluations

                    let tx = ng_tx.clone();

                    tpool.execute(move || {
                        let r = Self::self_play_game(p1);

                        tx.send(r).unwrap();
                        // new_games.lock().unwrap().push(r);
                        running.fetch_sub(1, Ordering::SeqCst);
                    });
                }

                let mut p1_steps = 0;
                while running.load(Ordering::SeqCst) != 0 {
                    p1_steps += worker.do_a_work();
                    print!("\r[COIN]         Total Evals: [{:8}] ({:4.1}%)", 
                        p1_steps, p1_steps as f32 / (0.6 * (TF_EVAL_BATCH_SIZE * EVAL_ROUNDS) as f32));
                    stdout().flush().unwrap();
                }
                println!("");
            });
            // tpool.join();

            let elapsed = start.elapsed();
            println!("\r[COIN]       Time elapsed: {:5}.{:09}", elapsed.as_secs(), elapsed.subsec_nanos());
        }

        let mut new_games = Vec::with_capacity(TF_EVAL_BATCH_SIZE * GAME_BATCHES_PER_ROUND);
        loop {
            if let Ok(e) = ng_rx.try_recv() {
                new_games.push(e);
            } else {
                break;
            }
        }

        let data = bincode::serialize(&new_games, bincode::Infinite).unwrap();
        let sample_data = serde_json::to_string(&new_games[0..128]).unwrap();

        let _test : Vec<Game> = bincode::deserialize(&data[..]).unwrap();

        let path_str = format!("iter{:03}/new_games.dat", iter);
        let path = self.data_folder.join(Path::new(&path_str));

        let sample_path_str = format!("iter{:03}/new_games.json", iter);
        let sample_path = self.data_folder.join(Path::new(&sample_path_str));

        fs::create_dir_all(path.parent().unwrap()).unwrap();

        let mut fd = File::create(path).unwrap();
        fd.write_all(&data).unwrap();

        let mut fd = File::create(sample_path).unwrap();
        fd.write_all(&sample_data.as_bytes()).unwrap();

        let n = new_games.len();

        for (i,e) in new_games.into_iter().enumerate() {
            if self.recent_games.len() < GAME_HISTORY_LENGTH {
                self.recent_games.push(e);
            } else {
                self.recent_games[(self.last_game+i) % GAME_HISTORY_LENGTH] = e;
            }
        }
        self.last_game = (self.last_game + n) % GAME_HISTORY_LENGTH;

        println!("\r[COIN]     Added {} new games.", n);
    }

    /// Have the best player play a game against itself and return the result
    fn self_play_game(mut p1 : MctsTree<ParallelCoinNet>) -> Game {
        let mut g = Game::new();
        let mut rng = rand::thread_rng();
        let mut b = Board::new();

        let mut turns = 0;
        let mut turn = 1.0;

        /*  Initialize each player for the game. */
        p1.set_position(b);
        p1.set_temp(1.0);

        /*  Simulate the game. */
        while !b.is_done() {
            let mut moves = empty_movelist();
            let n = b.get_moves(&mut moves);

            let mut selected_move = Move::pass();
            let mut val = EvalOutput::new();

            if n != 0 {
                if (turns < SELF_PLAY_VARIANCE_TURNS) {
                    p1.apply_dirichlet_noise(0.03);
                }

                p1.n_rounds(EVAL_ROUNDS);
                val = p1.evaluate(&b);

                let sum : f32 = (0..(n as usize)).map(|i| val.0[moves[i].offset() as usize]).sum();

                if (sum < 0.0) {
                    eprintln!("[COIN]       Probable Overflow {}", line!());
                }

                let rmove = rng.gen_range(0.0, sum);

                let mut mi = 0;
                let mut sm = 0.0;

                for i in 0..(n as usize) {
                    sm += val.0[moves[i].offset() as usize];
                    if rmove <= sm {
                        mi = i;
                        break;
                    }
                }

                selected_move = moves[mi];
            }

            g.add_position(b.clone(), val);
            g.add_move(selected_move);

            /*  Apply the move to the board and player. */
            b.f_do_move(selected_move);

            p1.prune(selected_move);
            
            turns += 1;
            turn = -turn;

            if turns == SELF_PLAY_VARIANCE_TURNS {
                p1.set_temp(1.0e-1);
            }
        }

        let result = (b.piece_diff() as f32).signum();
        g.set_result(-result);

        g
    }

    /// Save all the players to a directory
    fn save_players(&mut self, iter : usize) {
        let dir_path = format!("iter{:03}/",iter);
        let dir = self.data_folder.join(Path::new(&dir_path));
        fs::create_dir_all(dir.clone()).unwrap();
        let can_dir = &fs::canonicalize(dir).unwrap_or_else(|_| panic!(line!()));
        for i in 0..(self.players.len()) {
            let err = if i == self.best {
                self.players[i].0.save(&can_dir.join(Path::new(&format!("CoinNet-checkpoint.best"))))
            } else {
                self.players[i].0.save(&can_dir.join(Path::new(&format!("CoinNet-checkpoint.{}", i))))
            };

            match err {
                Ok(_) => {},
                Err(e) => {
                    println!("Error saving player {}: {}", i, e);
                }
            }
        }
    }

    pub fn load_files(&mut self) -> Result<usize,Box<Error>> {

        let dir = self.data_folder;
        use std::cmp::max;
        let mut p_iter_dirs = vec![];
        for dir in fs::read_dir(dir)? {
            let entry = dir?;
            if entry.file_name().to_str().unwrap().starts_with("iter") {
                p_iter_dirs.push(entry.path());
            }
        }

        println!("[COIN] Existing training directories: {:?}", p_iter_dirs);

        let mut new_games = vec![];

        for dir in &p_iter_dirs {
            eprintln!("[COIN]   Loading Games from {:?}", dir.join(Path::new("new_games.dat")));
            match File::open(dir.join(Path::new("new_games.dat"))) {
                Ok(mut fd) => {
                    let mut contents : Vec<u8> = vec![];
                    fd.read_to_end(&mut contents)?;
                    let mut decoded : Vec<Game> = bincode::deserialize(&contents[..])?;
                    new_games.append(&mut decoded);
                },
                Err(_) => {
                    println!("[COIN]   Directory ({:?}) missing game files...", dir);
                }
            }


        }

        let n = new_games.len();

        if n > GAME_HISTORY_LENGTH {
            let mut tmp = new_games[(n-GAME_HISTORY_LENGTH)..].iter()
                .cloned().collect::<Vec<_>>();
            self.recent_games.append(&mut tmp);
        } else {
            self.recent_games.append(&mut new_games);
        }

        println!("[COIN] Loaded {} saved games.", self.recent_games.len());

        match p_iter_dirs.last() {
            Some(last_dir) => {
                println!("[COIN] Loading checkpoints from \"{:?}\"...", last_dir);
                for i in 0..(self.players.len()) {
                    let file = format!("CoinNet-checkpoint.{}.index",i);
                    let prefix = format!("CoinNet-checkpoint.{}",i);
                    let prefix2 = format!("CoinNet-checkpoint.best");
                    if last_dir.join(file).is_file() {
                        self.players[i].0.load(&last_dir.join(prefix))?;
                    } else {
                        self.players[i].0.load(&last_dir.join(prefix2))?;
                        self.best = i;
                    }

                    // let output = self.players[i].1[0].eval.evaluate(&Board::new());


                    // println!("[COIN] Loaded player {} => ({:?})", i, output.1);
                    println!("[COIN] Loaded player {}", i);
                }
            },
            None => {}
        }

        Ok(p_iter_dirs.len())
    }

    pub fn seed_players(&mut self, iter : usize, wthor_file : &Path) -> Result<(),Box<Error>>{
        let mut games = vec![]; //load_wthor_database(wthor_file)?;

        for dir in fs::read_dir(wthor_file)? {
            let entry = dir?;
            if entry.path().is_file() && entry.file_name().to_str().unwrap().ends_with(".wtb") {
                eprint!("[COIN] Loading data from {:?}...", entry.path());
                match load_wthor_database(&entry.path()) {
                    Ok(mut ngames) => {
                        games.append(&mut ngames);
                        eprintln!("Success!");
                    }
                    _ => {
                        eprintln!("Error Loading Games.");
                    }
                }
            }
        }

        eprintln!("[COIN] Loaded {} training positions.", games.len());

        mem::swap(&mut self.recent_games, &mut games);

        self.best = 10000;

        self.training_round(0.05);
        self.training_round(0.025);
        self.training_round(0.0125);

        self.best = 0;

        self.save_players(iter);

        self.evaluate_players();


        mem::swap(&mut self.recent_games, &mut games);

        let n = self.recent_games.len();

        self.recent_games.append(&mut games);
        self.recent_games.truncate(GAME_HISTORY_LENGTH);
        self.last_game = n;

        Ok(())
    }

    /// Do a training iteration.
    pub fn iteration(&mut self, iter : usize, eta : f32) {
        println!("[COIN]   Generating self-play games...");
        self.self_play(iter);
        
        println!("[COIN]   Training Players...");
        self.training_round(eta);

        println!("[COIN]   Evaluating Players...");
        self.evaluate_players();
        self.play_random();

        println!("[COIN]   Saving Player Checkpoints...");
        self.save_players(iter);
    }
}


#[cfg(test)]
mod tests {
    
    use eval::*;
    use mcts::*;
    use train::*;

    use std::error::Error;
    use std::result::Result;

    #[test]
    fn correctness_test() {
        
        // use std::path::Path;

        // let mut trainer = MctsTrainer::new(6, &Path::new("./data/CoinNet_model.pb"), None);

        // let n = trainer.load_files(&Path::new("./data")).unwrap();

        // let g = &trainer.recent_games[trainer.last_game];

        // eprintln!("{} {}", g.moves.len(), g.states.len());

        // for &(b,EvalOutput(_,v)) in g.states.iter() {
        //     eprintln!("Board: (Result: {})\n{}", v, b);
        // }

        // eprintln!("Done testing");
    }
}