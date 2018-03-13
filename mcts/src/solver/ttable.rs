use solver::hash::*;
use indexmap::*;
use bitboard::*;

#[derive(Copy, Clone)]
pub struct TEntry {
    pub bound : TBound,
    pub best_move : Move,
}

impl TEntry {
    pub fn lower(bound : i32, best_move : Move) -> TEntry {
        TEntry{
            bound : TBound::lower(bound),
            best_move,
        }
    }
    pub fn upper(bound : i32, best_move : Move) -> TEntry {
        TEntry{
            bound : TBound::upper(bound),
            best_move,
        }
    }
    pub fn exact(bound : i32, best_move : Move) -> TEntry {
        TEntry{
            bound : TBound::exact(bound),
            best_move,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TBound {
    Lower(i32),
    Upper(i32),
    Exact(i32),
}

impl TBound {
    pub fn lower(bound : i32) -> TBound {
        TBound::Lower(bound)
    }
    pub fn upper(bound : i32) -> TBound {
        TBound::Upper(bound)
    }
    pub fn exact(bound : i32) -> TBound {
        TBound::Exact(bound)
    }
}

const TTABLE_SEGMENTS : usize = 6;

#[derive(Clone)]
pub struct TTable {
    max_size : usize,
    data : [ZobristMap<(u64, u64), TEntry>; TTABLE_SEGMENTS],
}

impl TTable {
    pub fn new(max_size : usize) -> TTable {
        TTable {
            max_size,
            data : [
                ZobristMap::default(),
                ZobristMap::default(),
                ZobristMap::default(),
                ZobristMap::default(),
                ZobristMap::default(),
                ZobristMap::default()
            ],
        }
    }

    pub fn store(&mut self, position : (u64, u64), bound : TEntry) {
        let e = (position.0 | position.1).count_zeros();
        let e = (e / 5) as usize;
        assert!(e < 6);
        if self.data[e].contains_key(&position) {
            self.data[e][&position] = bound;
        } else {
            if self.max_size == 0 {
                for i in (0..TTABLE_SEGMENTS).rev() {
                    let n = self.data[i].len();
                    if n > 0 {
                        self.data[i].clear();
                        self.max_size += n;
                        break;
                    }
                }
            }
            self.data[e].insert(position, bound);
            self.max_size -= 1;
        }
    }

    pub fn check(&self, position : (u64, u64)) -> Option<TEntry> {
        let e = (position.0 | position.1).count_zeros();
        let e = (e / 5) as usize;
        assert!(e < 6);

        self.data[e].get(&position).map(|&e| e)
    }
}