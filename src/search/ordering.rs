use chess::{
    Board,
    MoveGen,
    ChessMove,
    Color,
    BitBoard,
    BoardStatus,
    Square,
    Piece,
    EMPTY,
    ALL_PIECES,
};

use crate::constants::{
    CHECKMATE,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
};

use crate::search::evaluation;
use crate::constants::value;

use rand::prelude::*;

fn mvvlva(board: &Board, movegen: &mut MoveGen) -> Vec<ChessMove> {
    let mut moves = vec![];
    
    // Pawns first, king last
    for piece in ALL_PIECES.iter().rev() {
        movegen.set_iterator_mask(*board.pieces(*piece) & board.color_combined(!board.side_to_move()));
        
        for m in &mut *movegen {
            moves.push(m);
        }
    }
    
    moves
}

pub fn order_moves(board: &Board, mut movegen: MoveGen) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = vec![];
    
    // Captures
    movegen.set_iterator_mask(*board.color_combined(!board.side_to_move()));
    moves.append(&mut mvvlva(board, &mut movegen));

    // Checkmate
    if board.status() == BoardStatus::Checkmate {
        // Handle checkers
        movegen.set_iterator_mask(*board.checkers());
        
        moves.append(&mut mvvlva(board, &mut movegen));
    }
    
    // Iterate over the rest of the moves
    movegen.set_iterator_mask(!EMPTY);
    
    for m in &mut movegen {
        moves.push(m);
    }

    // Shuffle
    // moves.shuffle(&mut rand::thread_rng());
    
    moves
}