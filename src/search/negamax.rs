use chess::{
    Board,
    MoveGen,
    ChessMove,
    BoardStatus,
    Color,
};

use std::collections::HashMap;
use rand::prelude::*;

use crate::engine::Weakfish;
use crate::search::transposition::{TT, TableEntry};
use crate::search::evaluation::evaluate;
use crate::search::ordering::order_moves;
use crate::constants::{CHECKMATE};

pub fn negamax(weakfish: &Weakfish, board: &Board, depth: u8, mut alpha: f32, mut beta: f32, turn_multiplier: f32, ply: u8, table: &TT, hashmap: &mut HashMap<u64, TableEntry>) -> (Option<ChessMove>, f32) {
    if weakfish.is_interruption {
        return (None, 0f32);
    }
    
    // Mate distance pruning
    let mated = CHECKMATE - ply as f32;

    if mated < beta {
        beta = mated;
        if alpha >= mated {
            return (None, mated);
        }
    }

    let mated = -CHECKMATE + ply as f32;

    if mated > alpha {
        alpha = mated;
        if beta <= mated {
            return (None, mated);
        }
    }
    
    //Lookup in transposition table
    let hash = table.hash(board);
    let mut score = 0f32;

    if let Some(entry) = hashmap.get(&hash) {
        return (None, entry.score);
    }

    if depth == 0 || board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
        return (None, evaluate(&board) + 1f32 - ply as f32);
    }
    
    let movegen = MoveGen::new_legal(&board);
    let movegen = order_moves(board, movegen);
    
    let mut best_so_far: Option<ChessMove> = None;
    let mut max_score = -CHECKMATE;

    for m in movegen {
        let b = board.make_move_new(m);
        score = -negamax(weakfish, &b, depth - 1, -beta, -alpha, -turn_multiplier, ply + 1, table, hashmap).1;

        if score > max_score {
            best_so_far = Some(m);
            max_score = score;
        }

        if depth == 7 {
            println!("{} {:0.1} | best {:0.1} ({})", m, score, max_score, if let Some(x) = best_so_far { x.to_string() } else { "None".to_string() });
        }

        alpha = if alpha >= score { alpha } else { score };
        
        if alpha >= beta {
            break;
        }
        // } else {
        //     hashmap.insert(hash, TableEntry { depth: depth, score: score, age: 0u8});
        // }
    }
    
    // Get last move if all moves are equivalent
    if best_so_far == None {
        let mut moves: Vec<ChessMove> = MoveGen::new_legal(board).collect();
        moves.shuffle(&mut rand::thread_rng());
        best_so_far = Some(*moves.last().unwrap());
    }
    
    hashmap.insert(hash, TableEntry { depth: depth, score: max_score, age: 0u8 });

    // if best_so_far != None && ply == 1 {
    //     println!("Ponder {} {:0.1}", best_so_far.unwrap(), max_score);
    // }
    (best_so_far, max_score)
}