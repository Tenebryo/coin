use std::io::prelude::*;
use std::path::Path;
use std::fs::File;
use std::fs;

use std::sync::Arc;

use std::error::Error;
use std::result::Result;

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;

use tf::Code;
use tf::Graph;
use tf::ImportGraphDefOptions;
use tf::Session;
use tf::SessionOptions;
use tf::Status;
use tf::StepWithGraph;
use tf::Tensor;
use tf::Operation;

use bitboard::*;

pub type EvalInput  = [Board;1];
pub type EvalPrior = [f32;64];
pub type EvalScore = f32;

#[derive(Clone)]
pub struct EvalOutput(pub EvalPrior, pub EvalScore);
impl EvalOutput {
    pub fn new() -> EvalOutput {
        EvalOutput([0.0;64],0.0)
    }
}

impl Serialize for EvalOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut split1 = [0.0;32];
        let mut split2 = [0.0;32];

        for i in 0..32 {
            split1[i] = self.0[i];
            split2[i] = self.0[i+32];
        }

        let tmp = ((split1, split2), self.1);

        tmp.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EvalOutput {
    fn deserialize<D>(deserializer: D) -> Result<EvalOutput, D::Error>
        where D: Deserializer<'de>
    {
        let mut tmp = <(([f32;32],[f32;32]),f32)>::deserialize(deserializer)?;

        let mut ret = EvalOutput([0.0;64],0.0);
        ret.0[..32].swap_with_slice(&mut (tmp.0).0);
        ret.0[32..].swap_with_slice(&mut (tmp.0).1);
        ret.1 = tmp.1;

        Ok(ret)
    }
}

impl Into<([f32;64],f32)> for EvalOutput {
    fn into(self) -> ([f32;64],f32) {
        (self.0, self.1)
    }
}

impl From<([f32;64],f32)> for EvalOutput {
    fn from(f : ([f32;64],f32)) -> EvalOutput {
        EvalOutput(f.0, f.1)
    }
}


pub trait Evaluator: Sized + Clone {
    fn evaluate(&mut self, input : &EvalInput) -> EvalOutput;
    fn train(&mut self, input : &[EvalInput], target : &[EvalOutput], eta : f32) -> f32;
    fn save(&mut self, filename : &Path) -> Result<(), Box<Error>>;
    fn load(&mut self, filename : &Path) -> Result<(), Box<Error>>;
}

pub struct CoinNet {
    session       : Session,
    graph         : Arc<Graph>,
    input         : Operation,
    output_p      : Operation,
    output_v      : Operation,
    train_sgd     : Operation,
    train_adm     : Operation,
    train_mtn     : Operation,
    target_p      : Operation,
    target_z      : Operation,
    learning_rate : Operation,
    lambda        : Operation,
    loss          : Operation,
    save          : Operation,
    restore       : Operation,
    file_name     : Operation,
}

impl CoinNet {
    pub fn new(fname : &str) -> Result<CoinNet, Box<Error>> {

        if !Path::new(fname).exists() {
                return Err(Box::new(Status::new_set(Code::NotFound,
                                                    &format!("ERROR: Model file '{}' does not exist.", fname))
                    .unwrap()));
        }

        let mut graph = Graph::new();
        let mut proto = Vec::new();
        File::open(fname)?.read_to_end(&mut proto)?;
        graph.import_graph_def(&proto, &ImportGraphDefOptions::new())?;
        let mut session = Session::new(&SessionOptions::new(), &graph)?;

        let mut step = StepWithGraph::new();
        step.add_target(&graph.operation_by_name_required("CoinNet/init")?);
        session.run(&mut step)?;

        Ok(CoinNet{
            session       : session,
            input         : graph.operation_by_name_required("CoinNet/input")?,
            output_p      : graph.operation_by_name_required("CoinNet/output_p")?,
            output_v      : graph.operation_by_name_required("CoinNet/output_v")?,
            train_sgd     : graph.operation_by_name_required("CoinNet/train_sgd")?,
            train_adm     : graph.operation_by_name_required("CoinNet/train_adm")?,
            train_mtn     : graph.operation_by_name_required("CoinNet/train_mtn")?,
            target_p      : graph.operation_by_name_required("CoinNet/target_p")?,
            target_z      : graph.operation_by_name_required("CoinNet/target_z")?,
            learning_rate : graph.operation_by_name_required("CoinNet/learning_rate")?,
            lambda        : graph.operation_by_name_required("CoinNet/lambda")?,
            loss          : graph.operation_by_name_required("CoinNet/loss")?,
            save          : graph.operation_by_name_required("CoinNet/saver/SaveV2")?,
            // restore       : graph.operation_by_name_required("CoinNet/saver/RestoreV2")?,
            restore       : graph.operation_by_name_required("CoinNet/saver/restore_all")?,
            file_name     : graph.operation_by_name_required("CoinNet/saver/Const")?,
            graph         : Arc::new(graph),
        })
    }

    pub fn eval(&mut self, b : Board, output_p : &mut [f32; 64], output_v : &mut f32) -> Result<(), Box<Error>> {
        let mut tn = Tensor::new(&[1, 192]);

        let (mut ps, mut os) = b.pieces();
        let (mut pm, mut om) = b.mobility();

        for i in 0..64 {
            let ii = 3*i;
            tn[ii+0] = (ps & 1) as f32;
            tn[ii+1] = (os & 1) as f32;
            tn[ii+2] = (pm & 1) as f32;
            ps >>= 1;
            os >>= 1;
            pm >>= 1;
            om >>= 1;
        }

        let mut step = StepWithGraph::new();
        step.add_input(&self.input, 0, &tn);
        let tok_output_p = step.request_output(&self.output_p, 0);
        let tok_output_v = step.request_output(&self.output_v, 0);
        self.session.run(&mut step)?;

        let t_output_p = step.take_output(tok_output_p)?;
        let t_output_v = step.take_output(tok_output_v)?;
        for i in 0..64 {
            output_p[i] = t_output_p[i];
        }

        *output_v = t_output_v[0];

        Ok(())
    }

    pub fn trn(&mut self, inputs : &[Board], targets : &[EvalOutput], eta : f32) -> Result<f32, Box<Error>> {
        assert!(inputs.len() == targets.len());

        let n = inputs.len() as u64;

        let mut input_tensor = Tensor::<f32>::new(&[n, 192]);
        let mut target_p_tensor = Tensor::<f32>::new(&[n, 64]);
        let mut target_z_tensor = Tensor::<f32>::new(&[n, 1]);

        for j in 0..(n as usize) {
            let jj = 192 * j;
            let jjj = 64 * j;

            let (mut ps, mut os) = inputs[j].pieces();
            let (mut pm, mut om) = inputs[j].mobility();

            for i in 0..64 {
                let ii = 3*i;
                input_tensor[jj+ii+0] = (ps & 1) as f32;
                input_tensor[jj+ii+1] = (os & 1) as f32;
                input_tensor[jj+ii+2] = (pm & 1) as f32;
                ps >>= 1;
                os >>= 1;
                pm >>= 1;

                target_p_tensor[jjj + i] = targets[j].0[i];
            }
            target_z_tensor[j] = targets[j].1;
        }

        let mut learning_rate = Tensor::new(&[]);
        learning_rate[0] = eta;

        let mut lambda = Tensor::new(&[]);
        lambda[0] = 0.0001f32;

        let mut step = StepWithGraph::new();

        /*  Feed in the inputs to the trainer. */
        step.add_input(&self.input, 0, &input_tensor);
        step.add_input(&self.target_p, 0, &target_p_tensor);
        step.add_input(&self.target_z, 0, &target_z_tensor);
        step.add_input(&self.learning_rate, 0, &learning_rate);
        step.add_input(&self.lambda, 0, &lambda);
        step.add_target(&self.train_sgd);

        let loss = step.request_output(&self.loss, 0);
        self.session.run(&mut step)?;

        Ok(step.take_output(loss)?[0])
    }
}

impl Clone for CoinNet {
    fn clone(&self) -> CoinNet {
        let mut session = Session::new(&SessionOptions::new(), &self.graph).unwrap();

        let mut step = StepWithGraph::new();
        step.add_target(&self.graph.operation_by_name_required("CoinNet/init").unwrap());
        session.run(&mut step).unwrap();

        CoinNet{
            session       : session,
            input         : self.input.clone(),
            output_p      : self.output_p.clone(),
            output_v      : self.output_v.clone(),
            train_sgd     : self.train_sgd.clone(),
            train_adm     : self.train_adm.clone(),
            train_mtn     : self.train_mtn.clone(),
            target_p      : self.target_p.clone(),
            target_z      : self.target_z.clone(),
            learning_rate : self.learning_rate.clone(),
            lambda        : self.lambda.clone(),
            loss          : self.loss.clone(),
            save          : self.save.clone(),
            restore       : self.restore.clone(),
            file_name     : self.file_name.clone(),
            graph         : self.graph.clone(),
        }
    }
}

impl Evaluator for CoinNet {
    fn evaluate(&mut self, input : &EvalInput) -> EvalOutput {
        let mut res = ([0.0;64], 0.0);
        self.eval(input[0], &mut res.0, &mut res.1).unwrap();
        EvalOutput(res.0, res.1)
    }

    fn train(&mut self, input : &[EvalInput], targets : &[EvalOutput], eta : f32) -> f32 {
        self.trn(&input.iter().map(|x| {x[0]}).collect::<Vec<_>>(), targets, eta).unwrap()
    }

    fn save(&mut self, filename : &Path) -> Result<(), Box<Error>> {

        let mut f_tensor = Tensor::<String>::new(&[]);

        f_tensor[0] = filename.to_str().unwrap().to_string();

        let mut step = StepWithGraph::new();
        step.add_input(&self.file_name, 0, &f_tensor);
        step.add_target(&self.save);
        self.session.run(&mut step).unwrap();
        Ok(())
    }

    fn load(&mut self, filename : &Path) -> Result<(), Box<Error>>{

        let mut f_tensor = Tensor::<String>::new(&[]);

        f_tensor[0] = filename.to_str().unwrap().to_string();

        let mut step = StepWithGraph::new();
        step.add_input(&self.file_name, 0, &f_tensor);
        step.add_target(&self.restore);
        self.session.run(&mut step).unwrap();

        Ok(())
    }
}