COIN, the Othello/Reversi bot written in Rust.
==============================================

COIN was written for the Caltech CS2 Final Project, a bot for an Othello Tournament, although as a TA of the course. 

# Techniques

The main search algorithm is a MCTS implementation driven by a combined policy/value convolutional network, a la AlphaZero. In addition an endgame solver was implemented to ensure games are not thrown away unreasonably.

## Machine Learning (Using Tensorflow)

### Model

The policy/value network has gone through two iterations so far, consisting generally of some convolutional layers with batch normalization, dropout, and ReLU activation, followed by diverging stacks of hidden layers to a policy output (sigmoid activation) and a value output (tanh activation). The first version had 4 layers of 3x3 convolutions and the new version has 2 layers of 5x5 convolutions followed by 4 layers of 3x3 convolutions. This was optimized using the Momentum optimizer with a parameter of 0.9. The model is built in the [mcts/src/model.py](mcts/src/model.py) file. The model has ~500,000 trainable parameters overall.

The choice to make a 3x3 convolutional network was made based on [this paper](https://arxiv.org/pdf/1711.06583.pdf), which achieved results using supervised learning with a convolutional network.

### Training

The code for training is in [mcts/src/train.rs](mcts/src/train.rs); it implements the same reinforcement learning process as AlphaZero. The model is trained from scratch, without the need for any external data, labeled or otherwise. One interesting observation I made while running the self-play and training was that the average loss after each round of self-play (consisting of 1024 games) at first decreased as it learned the basics of how to play the game, but then started to increase again as more complex and novel positions were encountered during self-play. This would perhaps suggest that a more complex model could bear improvements, and the relative strength of the model at various stages in training has yet to be assessed.

## Endgame Solver

The endgame solver is relatively simple, but crucial to the bot's performance. Othello is a simple enough game towards the end that alpha-beta minimax can solve the game with 20+ moves left, in most cases. Thus, because the MCTS could be inaccurate in the endgame, an endgame solver was implemented. It is implemented in the [mcts/src/solver/](mcts/src/solver/) directory. The solver is just an optimized alpha-beta implementation with move ordering and a WLD heuristic for maximum pruning. It can often reach a depth of around 25 in a reasonable amount of time. Simple hand-tuned heuristics are used to minimize the number of nodes searched. A killer move table is used to track the number of beta cutoffs a move to a given position produces. Opponent mobility is minimized with dual intent: search branches that will require less computations first, and minimizing the opponent's mobility is a basic strategic goal, hopefully searching better moves first. Finally, the depth 2 minimax value of the current player's mobility is used as well to hopefully choose better moves that will produce more cutoffs. These 3 factors are linearly combined with hand-tuned coefficients (determined experimentally) to order the moves for all alpha-beta calls while there are more than 5 empty squares (below this and it is faster to disregard move ordering). A transposition table was also implemented, but this seemed to give only moderate performance improvements for single searches, and instead mostly helped speed up subsequent searches in a game.

# Results

For the 2018 competition, the model was trained for 170 rounds (~48 hours on a NVidia 970m + i7-4710HQ laptop). The bot placed second in the round-robin tournament, notably beating (with 1 tie) all the other bots in the top 10 (including 1st place, a highly optimized and well written bot called [Flippy](https://github.com/jeffreyan11/othello_engine)).

A former professional player present at the tournament played against the bot and highlighted some of its weaknesses (it was a loss for COIN), but noted that COIN played perfect book moves for the first 20 moves or so. I thought that this was really interesting, since the model learned to play accepted opening book moves without any previous game data - only with self-play and reinforcement learning.

# Future Improvements

All losses in the 2018 tournament were due to a wipeout bug (where all COIN's pieces are captured early in the game), or playing random moves when the endgame solver can't find a winning move. Both of these should be relatively easy fixes. MCTS could also be improved by batching multiple calls to the Tensorflow backend; this would require implementing a modification to make the MCTS algorithm parallelized, which is doable, but slightly awkward. The endgame solver can also probably be improved, as bots such as [Flippy](https://github.com/jeffreyan11/othello_engine), [Edax](https://github.com/abulmo/edax-reversi), and [NTest](https://github.com/weltyc/ntest) are often able to solve positions with over 30 empty squares.

More interesting improvements could be made on the ML side. Based on the feedback from a former professional player, it seems that patters larger than can't easily be learned with 4 3x3 convolutional layers are important for the strength of bots (especially against human players). As such, investigations into better model architectures are warranted (larger convolution patches might take care of the large-pattern problem).

# Installing and Running

The project is written in Rust, which you can easily install with [Rustup](https://rustup.rs). On linux, you shouldn't need to install anything else - Cargo will take care of all the dependencies - but on Windows you will have to install the Tensorflow binaries ahead of time (the only way to do this seems to be to extract `tensorflow.lib` from the Windows python pip library and convert - not simply rename - it into `tensorflow.dll`). Then you can `cargo build --release` in the [coin](/coin) directory to build the player or the [mcts](/mcts) directory to build the trainer.
