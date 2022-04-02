use chess::{
    Board,
    Color,
};

use std::str::FromStr;
use std::collections::HashMap;

use crate::constants::CHECKMATE;
use crate::search::negamax;
use crate::search::transposition::{TT, TableEntry};

pub struct Weakfish {
    pub is_interruption: bool,
}

impl Weakfish {
    pub fn new() -> Weakfish {
        Weakfish {
            is_interruption: false,
        }
    }

    pub fn go(&self, pos: String, depth: u8, table: &TT, hashmap: &mut HashMap<u64, TableEntry>) -> String {
        let board = Board::from_str(pos.as_str()).unwrap();
    
        let best = negamax::negamax(self, &board, depth, -CHECKMATE, CHECKMATE, if board.side_to_move() == Color::White { 1f32 } else { -1f32 }, 0, table, hashmap).0.unwrap();
        return best.to_string();
    }

    pub fn quit(&mut self) {
        self.is_interruption = true;
    }
}