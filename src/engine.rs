use chess::{Board, Color, ChessMove};

use std::str::FromStr;
use std::collections::HashMap;

use crate::uci;
use crate::uci::{Status, Memory};
use crate::search::transposition::{TT, TableEntry};
use crate::constants::CHECKMATE;
use crate::search::negamax;

pub struct Weakfish {
    pub is_interruption: bool,
}

impl Weakfish {
    pub fn new() -> Weakfish {
        Weakfish {
            is_interruption: false,
        }
    }

    pub fn run(&mut self) {
        let mut mem = Memory::new();
        
        // Creates the transposition table
        let mut tt = TT::new();
        let mut hash_table: HashMap<u64, TableEntry> = HashMap::new();
        tt.update(&Board::default());

        println!("Weakfish v0.1.0 by juliohq 2022");

        loop {
            let input = uci::get_input();
            
            match uci::parse(input, &mut mem) {
                Status::Continue => {},
                Status::Go(depth) => {
                    let move_str = self.go(mem.pos.clone(), 7, &tt, &mut hash_table);
                    uci::best_move(move_str);
                },
                Status::Position(fen, moves) => {
                    let mut board = Board::from_str(fen.as_str()).unwrap();
                    
                    for m in moves {
                        board = board.make_move_new(ChessMove::from_str(m.as_str()).unwrap());
                    }
                    
                    mem.pos = board.to_string();
                },
                Status::Quit => {
                    self.quit();
                    break;
                }
            }
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