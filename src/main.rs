mod engine;
mod uci;
mod search;
mod constants;

use chess::{Board, ChessMove};
use uci::{Status, Memory};
use search::transposition::{TT, TableEntry};

use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    // Create the engine instance
    let mut weakfish = engine::Weakfish::new();

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
                let move_str = weakfish.go(mem.pos.clone(), 7, &tt, &mut hash_table);
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
                weakfish.quit();
                break;
            }
        }
    }
}