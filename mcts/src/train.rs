use rand::{self, Rng};
use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::fs::File;

use std::error::Error;

use bincode;

use bitboard::*;
use eval::*;
use mcts::*;

// const EVAL_ROUNDS : usize = 100;
// const EVAL_GAMES : usize = 4;
// const EVAL_CUTOFF : usize = 3;
// const EVAL_RANDOM : usize = 4;
// const TRAINING_ITERATIONS : usize = 128;
// const TRAINING_BATCH_SIZE : usize = 128;
// const GAME_HISTORY_LENGTH : usize = 10_000;
// const GAMES_PER_ROUND : usize = 5;
// const SELF_PLAY_VARIANCE_TURNS : usize = 15;

const EVAL_ROUNDS : usize = 800;
const EVAL_GAMES : usize = 25;
const EVAL_CUTOFF : usize = 15;
const EVAL_RANDOM : usize = 10;
const TRAINING_ITERATIONS : usize = 1064;
const TRAINING_BATCH_SIZE : usize = 256;
const GAME_HISTORY_LENGTH : usize = 10_000;
const GAMES_PER_ROUND : usize = 256;
const SELF_PLAY_VARIANCE_TURNS : usize = 15;

#[derive(Clone, Serialize, Deserialize)]
struct Game {
    states  : Vec<(EvalInput, EvalOutput)>,
    moves   : Vec<Move>,
}

impl Game {
    fn new() -> Game {
        Game {
            states  : vec![],
            moves   : vec![],
        }
    }

    fn add_position(&mut self, input : EvalInput, output : EvalOutput) {
        self.states.push((input, output));
    }

    fn add_move(&mut self, m : Move) {
        self.moves.push(m);
    }

    fn set_result(&mut self, mut result : f32) {
        for i in 0..(self.states.len()) {
            (self.states[i].1).1 = result;
            result = -result;
        }
    }
}

pub struct MctsTrainer<E: Evaluator> {
    best            : usize,
    players         : Vec<MctsTree<E>>,
    recent_games    : Vec<Game>,
    last_game       : usize,
}

impl<E: Evaluator> MctsTrainer<E> {
    pub fn new(n : usize, player : E) -> MctsTrainer<E> {
        MctsTrainer {
            best            : 0,
            players         : (0..n).map(|_| MctsTree::new(player.clone())).collect::<Vec<_>>(),
            recent_games    : vec![],
            last_game       : 0,
        }
    }

    /// Evaluate all the players against the best player, updating the best
    /// player if one surpasses it.
    fn evaluate_players(&mut self) {
        for i in 0..(self.players.len()) {
            if i != self.best {
                self.eval_player(i);
            }
        }
    }

    /// Evaluate the idx player against the best player */
    fn eval_player(&mut self, idx : usize) {
        println!("[COIN]     Evaluating Player {} against {}", idx, self.best);
        let mut wins = 0;
        let mut rwins = 0;
        for i in 0..EVAL_GAMES {
            print!("\r[COIN]       Game {} ({}-{})", i, wins, i-wins);
            io::stdout().flush().unwrap();

            let r = self.eval_game(idx, i%2 == 0);

            if r < 0.0 {
                wins += 1;
            }
        }
        for i in 0..EVAL_RANDOM {
            let r = self.random_game(idx, i%2 == 0);
            if r < 0.0 {
                rwins += 1;
            }
        }

        println!("\r[COIN]       Result: {} to {} ({} random winrate)", 
            wins, EVAL_GAMES  - wins, rwins as f32 / EVAL_RANDOM as f32);

        if wins >= EVAL_CUTOFF {
            self.best = idx;
        }
    }

    /// Play an evaluation game (i.e. using best guess instead of weighted guess
    /// to select moves with mcts). */
    fn eval_game(&mut self, idx : usize, start : bool) -> f32 {
        let mut g = Game::new();
        let mut rng = rand::thread_rng();
        let mut b = Board::new();

        let mut cp = self.best;
        let mut op = idx;
        let mut turn = 1.0;
        let mut t = 1;

        if start {
            let tp = cp;
            cp = op;
            op = tp;

            turn = -turn;
        }

        /*  Initialize each player for the game. */
        self.players[cp].set_position(b);
        self.players[op].set_position(b);
        self.players[cp].set_temp(0.25);
        self.players[op].set_temp(0.25);

        print!(" Move  1");
        /*  Simulate the game. */
        while !b.is_done() {
            print!("\x08\x08{:2}",t);
            io::stdout().flush().unwrap();
            t += 1;

            let mut moves = empty_movelist();
            let n = b.get_moves(&mut moves);

            let mut selected_move = Move::pass();

            if n != 0 {
                self.players[cp].n_rounds(EVAL_ROUNDS);
                let val = self.players[cp].evaluate(&[Board::new()]);

                // let mut mi = 0;
                // let mut mx = val.0[moves[0].offset() as usize];

                // for i in 1..(n as usize) {
                //     let tmp = val.0[moves[i].offset() as usize];
                //     if mx < tmp {
                //         mi = i;
                //         mx = tmp;
                //     }
                // }


                let sum : f32 = (0..(n as usize)).map(|i| val.0[moves[i].offset() as usize]).sum();

                assert!(sum >= 0.0);

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

            self.players[cp].prune(selected_move);
            self.players[op].prune(selected_move);

            /*  Swap the current player and the waiting player. */
            let tp = cp;
            cp = op;
            op = tp;

            turn = -turn;
        }

        turn * b.piece_diff() as f32 / 64.0
    }

    /// Play a game against a random player. */
    fn random_game(&mut self, idx : usize, start : bool) -> f32 {
        let mut g = Game::new();
        let mut rng = rand::thread_rng();
        let mut b = Board::new();

        let mut op = idx;
        let mut turn = 1.0;

        if start {
            turn = -turn;
        }

        /*  Initialize each player for the game. */
        self.players[op].set_position(b);
        self.players[op].set_temp(0.25);

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
                    self.players[op].n_rounds(EVAL_ROUNDS);
                    let val = self.players[op].evaluate(&[Board::new()]);

                    let sum : f32 = (0..(n as usize)).map(|i| val.0[moves[i].offset() as usize]).sum();

                    assert!(sum >= 0.0);

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

            self.players[op].prune(selected_move);

            turn = -turn;
        }

        turn * b.piece_diff() as f32 / 64.0
    }

    /// Train all the non-best . */
    fn training_round(&mut self, eta : f32) {
        for i in 0..(self.players.len()) {
            if i != self.best {
                self.train_player(i, eta);
            }
        }
    }

    /// Trains a player on TRAINING_ITERATIONS batches. */
    fn train_player(&mut self, idx : usize, eta : f32) {
        println!("[COIN]     Training Player {}", idx);
        let mut rng = rand::thread_rng();
        /*  do many times. */
        for i in 0..TRAINING_ITERATIONS {

            /*  Build a mini-batch. */
            let mut input = vec![];
            let mut output = vec![];

            while input.len() < TRAINING_BATCH_SIZE {
                let selected_game = rng.choose(&self.recent_games).unwrap();
                let selected = rng.choose(&selected_game.states);

                match selected {
                    Some(s) => {
                        /*  make sure we don't care about passes... */
                        if s.0[0].mobility().0 != 0 {
                            input.push(s.0);
                            output.push(s.1.clone());
                        }
                    },
                    None => {}
                }
            }

            /*  train on the mini-batch: */
            let err = self.players[idx].train(&input, &output, eta);


            print!("[COIN]       Epoch {} (loss={})\r", i, err);
            io::stdout().flush().unwrap();
        }
        println!("");
    }

    /// Generates GAMES_PER_ROUND new self-play games. */
    fn self_play(&mut self, iter : usize) {
        let mut positions = 0;
        let mut new_games : Vec<Game> = vec![];
        for i in 0..GAMES_PER_ROUND {
            print!("\r[COIN]     Game {}", i);
            io::stdout().flush().unwrap();

            let g = self.self_play_game();

            positions += g.states.len();

            // if i == GAMES_PER_ROUND-1 {
            //     for s in &g.states {
            //         println!("{}", s.0[0]);
            //     }
            // }

            new_games.push(g.clone());

            if self.recent_games.len() <= GAME_HISTORY_LENGTH {
                self.recent_games.push(g);
            } else {
                self.recent_games[self.last_game] = g;
                self.last_game += 1;
                self.last_game %= GAME_HISTORY_LENGTH;
            }
        }

        let mut data = bincode::serialize(&new_games, bincode::Infinite).unwrap();

        let test : Vec<Game> = bincode::deserialize(&data[..]).unwrap();

        let path_str = format!("./data/iter{:03}/new_games.dat", iter);
        let path = Path::new(&path_str);

        fs::create_dir_all(path.parent().unwrap()).unwrap();

        let mut fd = File::create(path).unwrap();
        fd.write_all(&data);

        println!("\r[COIN]     Added {} new positions.", positions);
    }

    /// Have the best player play a game against itself and return the result
    fn self_play_game(&mut self) -> Game {
        let mut g = Game::new();
        let mut rng = rand::thread_rng();
        let mut b = Board::new();

        let mut cp = self.best;
        let mut turns = 0;
        let mut turn = 1.0;

        /*  Initialize each player for the game. */
        self.players[cp].set_position(b);
        self.players[cp].set_temp(1.0);

        /*  Simulate the game. */
        while !b.is_done() {
            let mut moves = empty_movelist();
            let n = b.get_moves(&mut moves);

            let mut selected_move = Move::pass();
            let mut val = EvalOutput::new();

            if n != 0 {
                self.players[cp].n_rounds(EVAL_ROUNDS);
                val = self.players[cp].evaluate(&[Board::new()]);

                let sum : f32 = (0..(n as usize)).map(|i| val.0[moves[i].offset() as usize]).sum();

                assert!(sum >= 0.0);

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

            g.add_position([b.clone()], val);

            /*  Apply the move to the board and player. */
            b.f_do_move(selected_move);

            self.players[cp].prune(selected_move);
            
            turns += 1;
            turn = -turn;

            if turns == SELF_PLAY_VARIANCE_TURNS {
                self.players[cp].set_temp(1.0e-1);
            }
        }

        let result = turn * (b.piece_diff() as f32 / 64.0);
        g.set_result(result);

        g
    }

    /// Save all the players to a directory
    fn save_players(&mut self, iter : usize) {
        let dir_path = format!("./data/iter{:03}/",iter);
        let dir = Path::new(&dir_path);
        fs::create_dir_all(dir).unwrap();
        let can_dir = &fs::canonicalize(dir).unwrap_or_else(|_| panic!(line!()));
        for i in 0..(self.players.len()) {
            let err = if i == self.best {
                self.players[i].save(&can_dir.join(Path::new(&format!("CoinNet-checkpoint.best"))))
            } else {
                self.players[i].save(&can_dir.join(Path::new(&format!("CoinNet-checkpoint.{}", i))))
            };

            match err {
                Ok(_) => {},
                Err(e) => {
                    println!("Error saving player {}: {}", i, e);
                }
            }
        }
    }

    pub fn load_files(&mut self, dir : &Path) -> Result<usize,Box<Error>> {
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
            let mut contents : Vec<u8> = vec![];
            File::open(dir.join(Path::new("new_games.dat")))?
                .read_to_end(&mut contents)?;

            let mut decoded : Vec<Game> = bincode::deserialize(&contents[..])?;

            new_games.append(&mut decoded);
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
                        self.players[i].load(&last_dir.join(prefix))?;
                    } else {
                        self.players[i].load(&last_dir.join(prefix2))?;
                        self.best = i;
                    }

                    let output = self.players[i].eval.evaluate(&[Board::new()]);


                    println!("[COIN] Loaded player {} => ({:?})", i, output.1);
                }
            },
            None => {}
        }

        Ok(p_iter_dirs.len())
    }

    /// Do a training iteration.
    pub fn iteration(&mut self, iter : usize, eta : f32) {
        println!("[COIN]   Generating self-play games...");
        self.self_play(iter);
        
        println!("[COIN]   Training Players...");
        self.training_round(eta);

        println!("[COIN]   Saving Player Checkpoints...");
        self.save_players(iter);

        println!("[COIN]   Evaluating Players...");
        self.evaluate_players();
    }
}

