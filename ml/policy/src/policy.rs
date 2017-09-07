use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use std::error::Error;
use std::result::Result;

use bitboard::Board;

use tensorflow::Code;
use tensorflow::Graph;
use tensorflow::ImportGraphDefOptions;
use tensorflow::Session;
use tensorflow::SessionOptions;
use tensorflow::Status;
use tensorflow::StepWithGraph;
use tensorflow::Tensor;
use tensorflow::Operation;

use simulate::Game;

pub struct Policy {
    session       : Session,
    graph         : Graph,
    input         : Operation,
    output        : Operation,
    train         : Operation,
    expected      : Operation,
    learning_rate : Operation,
    cost          : Operation,
    save          : Operation,
    restore       : Operation,
    save_name     : Operation,
}

impl Policy {
    pub fn new(fname : &str) -> Result<Policy, Box<Error>> {

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
        step.add_target(&graph.operation_by_name_required("init")?);
        session.run(&mut step)?;

        Ok(Policy{
            session       : session,
            input         : graph.operation_by_name_required("policy/input")?,
            output        : graph.operation_by_name_required("policy/output")?,
            train         : graph.operation_by_name_required("policy/train")?,
            expected      : graph.operation_by_name_required("policy/expected")?,
            learning_rate : graph.operation_by_name_required("policy/learning_rate")?,
            cost          : graph.operation_by_name_required("policy/cost")?,
            save_name     : graph.operation_by_name_required("saver/Const:0")?,
            save          : graph.operation_by_name_required("saver/restore_all")?,
            restore       : graph.operation_by_name_required("saver/control_dependency:0")?,
            graph         : graph,
        })
    }

    pub fn eval(&mut self, b : Board, output : &mut [f32; 64]) -> Result<(), Box<Error>> {
        let mut tn = Tensor::new(&[1, 8, 8, 4]);

        let (mut ps, mut os) = b.pieces();
        let (mut pm, mut om) = b.mobility();

        for i in 0..64 {
            let ii = 4*i;
            tn[ii+0] = (ps & 1) as f32;
            tn[ii+1] = (os & 1) as f32;
            tn[ii+2] = (pm & 1) as f32;
            tn[ii+3] = (om & 1) as f32;
            ps >>= 1;
            os >>= 1;
            pm >>= 1;
            om >>= 1;
        }

        let mut step = StepWithGraph::new();
        step.add_input(&self.input, 0, &tn);
        let weights_i = step.request_output(&self.output, 0);
        self.session.run(&mut step)?;

        let weights = step.take_output(weights_i)?;
        for i in 0..64 {
            output[i] = weights[i];
        }

        Ok(())
    }

    pub fn train(&mut self, games : Vec<Game>, lr : f32) -> Result<f32, Box<Error>> {
        let mut n = games.iter().map(|g| g.len as u64 ).sum();

        let mut i_tensor = Tensor::new(&[n, 8, 8, 4]);
        let mut e_tensor = Tensor::new(&[n, 8, 8, 1]);

        for g in games {
            let mut r = g.result as f32;

            for p in 0..(g.len) {
                let ii = 256*p;
                let ei = 64*p;

                e_tensor[ei + g.moves[p].offset() as usize] = r;

                let (mut ps, mut os) = g.positions[p].pieces();
                let (mut pm, mut om) = g.positions[p].mobility();

                for i in 0..64 {
                    let j = 4*i;
                    i_tensor[ii + j + 0] = (ps & 1) as f32;
                    i_tensor[ii + j + 1] = (os & 1) as f32;
                    i_tensor[ii + j + 2] = (pm & 1) as f32;
                    i_tensor[ii + j + 3] = (om & 1) as f32;
                    ps >>= 1;
                    os >>= 1;
                    pm >>= 1;
                    om >>= 1;
                }

                r = -r;
            }
        }

        let mut learning_rate = Tensor::new(&[]);
        learning_rate[0] = lr;

        let mut step = StepWithGraph::new();
        step.add_input(&self.input, 0, &i_tensor);
        step.add_input(&self.expected, 0, &e_tensor);
        step.add_input(&self.learning_rate, 0, &learning_rate);
        step.add_target(&self.train);
        let cost = step.request_output(&self.cost, 0);
        self.session.run(&mut step)?;

        let c = step.take_output(cost)?[0];

        Ok(c)
    }
}