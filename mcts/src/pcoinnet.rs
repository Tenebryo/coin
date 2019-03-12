use parking_lot::Mutex;
use eval::EvalInput;
use eval::EvalOutput;
use std::error::Error;
use std::path::Path;

use std::sync::Arc;
use std::sync::mpsc::*;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use std::time::*;

use eval::*;
use coinnet::*;

/// The max number of inputs to batch together
pub const TF_EVAL_BATCH_SIZE : usize = 256;

/// The max amount of time before starting a batch that is smaller than the 
/// max size.
pub const TF_EVAL_BATCH_TIMEOUT : u32 = 25_000;

/// A worker that processes batches of inputs through tensorflow
pub struct ParallelCoinNetWorker {
    pub net : CoinNet,
    work_rx : Receiver<(EvalInput, Sender<EvalOutput>)>,
    work_tx : Sender<(EvalInput, Sender<EvalOutput>)>
}

impl ParallelCoinNetWorker {
    fn new(model : &Path, vars : Option<&Path>) -> Result<ParallelCoinNetWorker, Box<Error>> {
        let mut net = CoinNet::new(model)?;
        if let Some(vars) = vars {
            net.load(vars)?;
        }

        // let net = Mutex::new(net);
        let (work_tx, work_rx) = channel();

        Ok(ParallelCoinNetWorker{
            net, work_tx, work_rx
        })
    }

    /// Receive any work sent to this batch worker, process it, and send it back
    /// to the instance that sent it. Timeout dictates how long to wait for new
    /// inputs (since the last one arrived, so max delay is batch_size*timeout)
    /// batch_size dictates how many inputs to send to the tensorflow runtime
    /// at a time.
    pub fn do_a_work(&mut self, batch_size : usize, timeout : Duration) -> usize {
        let mut inputs = vec![];
        let mut txs = vec![];

        // collect inputs from queue, subject to timeout.
        for _ in 0..batch_size {
            if let Ok((input, tx)) = self.work_rx.recv_timeout(timeout) {
                inputs.push(input);
                txs.push(tx);
            } else {
                break;
            }
        }

        let n = inputs.len();

        // don't do anything if there are no inputs to send to tensorflow.
        if n == 0 {
            return n;
        }

        // evaluate neural net on inputs
        let outputs = self.net.evaluate_batch(&inputs);

        // send outputs back to corresponding threads.
        for (o,tx) in outputs.into_iter().zip(txs.iter()) {
            tx.send(o).unwrap();
        }

        n
    }

    pub fn train(&mut self, inputs : &[EvalInput], targets : &[EvalOutput], eta : f32) -> f32 {
        self.net.train(inputs, targets, eta)
    }

    pub fn save(&mut self, filename : &Path) -> Result<(), Box<Error>> {
        self.net.save(filename)
    }

    pub fn load(&mut self, filename : &Path) -> Result<(), Box<Error>>{
        self.net.load(filename)
    }
}

/// An evaluator whose inputs are evaluated in batches to maximize throughput of
/// many parallel games.
pub struct ParallelCoinNet {
    // batch_worker  : Arc<ParallelCoinNetWorker>,
    batch_channel : Sender<(EvalInput, Sender<EvalOutput>)>,
    eval_tx       : Sender<EvalOutput>,
    eval_rx       : Receiver<EvalOutput>,
}

impl ParallelCoinNet {
    /// Creates a new pool of evaluators that send all evalutations to a common
    /// batch processor to reduce TF overhead.
    pub fn new_worker_group(model : &Path, vars : Option<&Path>) -> Result<(ParallelCoinNet, ParallelCoinNetWorker), Box<Error>> {
        let worker = ParallelCoinNetWorker::new(model, vars)?;

        Ok(
            ({
                let (eval_tx, eval_rx) = channel();
                ParallelCoinNet{
                    // batch_worker  : worker.clone(),
                    batch_channel : worker.work_tx.clone(),
                    eval_tx, eval_rx,
                }
            }, worker)
        )
    }
}

impl Clone for ParallelCoinNet {
    /// This struct represents a producer for the batched tensorflow executor,
    /// so cloning just needs a channel to the consumer and a new channel for
    /// this producer's outputs.
    fn clone(&self) -> ParallelCoinNet {
        let (eval_tx, eval_rx) = channel();
        ParallelCoinNet {
            // batch_worker  : self.batch_worker.clone(),
            batch_channel : self.batch_channel.clone(),
            eval_tx, eval_rx,
        }
    }
}

impl Evaluator for ParallelCoinNet {
    fn evaluate(&mut self, input : &EvalInput) -> EvalOutput {
        self.batch_channel.send((input.clone(), self.eval_tx.clone())).unwrap();

        self.eval_rx.recv().unwrap()
    }

    fn evaluate_batch(&mut self, _input : &[EvalInput]) -> Vec<EvalOutput> {
        unimplemented!();
    }

    fn train(&mut self, _inputs : &[EvalInput], _targets : &[EvalOutput], _eta : f32) -> f32 {
        unimplemented!();
        // self.batch_worker.net.lock().unwrap().train(inputs, targets, eta)
    }

    fn save(&mut self, _filename : &Path) -> Result<(), Box<Error>> {
        unimplemented!();
        // self.batch_worker.net.lock().unwrap().save(filename)
    }

    fn load(&mut self, _filename : &Path) -> Result<(), Box<Error>>{
        unimplemented!();
        // self.batch_worker.net.lock().unwrap().load(filename)
    }
}