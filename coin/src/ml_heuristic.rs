use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::result::Result;
use std::path::Path;
use std::process::exit;
use tensorflow::Code;
use tensorflow::Graph;
use tensorflow::ImportGraphDefOptions;
use tensorflow::Session;
use tensorflow::SessionOptions;
use tensorflow::Status;
use tensorflow::StepWithGraph;
use tensorflow::Tensor;
use tensorflow::Operation;

use heuristic::Heuristic;
use bitboard::Board;
use bitboard::Move;
use bitboard::Turn;
use bitboard::MoveList;
use bitboard::MoveOrder;

const value_input_name : &'static str = "Placeholder:0";
const value_output_name : &'static str = "Tanh_2:0";

//policy network must be reworked, training went very wrong the first time.
//thus these are unstable/deprecated constants put in as placeholders for the
//future values.
const policy_input_name : &'static str = "Placeholder:0";
const policy_output_name : &'static str = "Tanh_2:0";


pub struct MLHeuristic {
    v_sess      : Session,
    p_sess      : Session,
    v_in        : Operation,
    v_out       : Operation,
    p_in        : Operation,
    p_out       : Operation,
}

impl MLHeuristic {
    pub fn new(value_path : String, policy_path : String) -> MLHeuristic {
        let mut value_graph = Graph::new();
        let mut policy_graph = Graph::new();
        let mut proto = Vec::new();
        
        File::open(value_path).unwrap().read_to_end(&mut proto).unwrap();
        value_graph.import_graph_def(&proto, &ImportGraphDefOptions::new()).unwrap();
        
        proto.clear();
        File::open(policy_path).unwrap().read_to_end(&mut proto).unwrap();
        policy_graph.import_graph_def(&proto, &ImportGraphDefOptions::new()).unwrap();
        
        let mut vsession = Session::new(&SessionOptions::new(), &value_graph).unwrap();
        let mut psession = Session::new(&SessionOptions::new(), &policy_graph).unwrap();
        
        //operation names are constant for now
        let vin_op = value_graph.operation_by_name_required(value_input_name).unwrap();
        let vout_op = value_graph.operation_by_name_required(value_output_name).unwrap();
        let pin_op = policy_graph.operation_by_name_required(policy_input_name).unwrap();
        let pout_op = policy_graph.operation_by_name_required(policy_output_name).unwrap();
        
        MLHeuristic {
            v_sess      : vsession,
            p_sess      : psession,
            v_in        : vin_op,
            v_out       : vout_op,
            p_in        : pin_op,
            p_out       : pout_op,
        }
    }
}

impl Heuristic for MLHeuristic {
    
    fn evaluate(&mut self, b : Board, t : Turn) -> i32 {
    
        if b.is_done() {
            return ((b.count_pieces(Turn::BLACK) as i32) - (b.count_pieces(Turn::WHITE) as i32)) * 8096;
        }
    
        let mut input_tensor = Tensor::new(&[192u64]);
    
        let mut black = b.pieces(Turn::BLACK);
        let mut white = b.pieces(Turn::WHITE);
        let mut mobil = b.mobility(t);
        
        for i in 0..64 {
            input_tensor[i]     = if (black & 1) != 0 {1.0f32} else {0.0f32};
            input_tensor[i+64]  = if (white & 1) != 0 {1.0f32} else {0.0f32};
            input_tensor[i+128] = if (mobil & 1) != 0 {1.0f32} else {0.0f32};
            
            black >>= 1;
            white >>= 1;
            mobil >>= 1;
        }
    
        let mut step = StepWithGraph::new();
        step.add_input(&self.v_in, 0, &input_tensor);
        step.add_target(&self.v_out);
        let out_tok = step.request_output(&self.v_out, 0);
        self.v_sess.run(&mut step);
        
        let r : f32 = step.take_output(out_tok).unwrap().data()[0];
        
        (r * 1024.0) as i32
    }
    
    /// Uses a policy network to gauge the efficacy of each move in order to
    /// pre-order them for alpha-beta search. 
    fn order_moves(&mut self, b : Board, n : usize, rmvs : &MoveList, omvs : &mut MoveOrder) {
        
    }
}
