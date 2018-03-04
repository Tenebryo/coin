

pub enum Algorithm {
    MCTS,
    MTDF,
    PVS,
    BNS,
}

pub struct CoinCfg {
    mode : Algorithm,
    heuristic_directory : String,
}