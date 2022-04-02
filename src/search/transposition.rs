use chess::{
    Board,
    Piece,
    BitBoard,
    Square,
    Color,
};
use rand::prelude::*;

pub const WHITE_PAWN: usize = 1;
pub const WHITE_BISHOP: usize = 2;
pub const WHITE_KNIGHT: usize = 3;
pub const WHITE_ROOK: usize = 4;
pub const WHITE_QUEEN: usize = 5;
pub const WHITE_KING: usize = 6;
pub const BLACK_PAWN: usize = 7;
pub const BLACK_BISHOP: usize = 8;
pub const BLACK_KNIGHT: usize = 9;
pub const BLACK_ROOK: usize = 10;
pub const BLACK_QUEEN: usize = 11;
pub const BLACK_KING: usize = 12;

pub const PIECES: [usize; 12] = [
    WHITE_PAWN,
    WHITE_BISHOP,
    WHITE_KNIGHT,
    WHITE_ROOK,
    WHITE_QUEEN,
    WHITE_KING,
    BLACK_PAWN,
    BLACK_BISHOP,
    BLACK_KNIGHT,
    BLACK_ROOK,
    BLACK_QUEEN,
    BLACK_KING,
];

#[derive(Copy, Clone)]
pub struct TT {
    pub table: [[u64; 13]; 64],
    pub black_to_move: u64,
}

pub enum Bound {
    Lower,
    Exact,
    Upper,
}

pub struct TableEntry {
    pub depth: u8,
    pub score: f32,
    pub age: u8,
    pub bound: Bound,
}

impl TableEntry {
    fn new(depth: u8, score: f32, age: u8, bound: Bound) -> TableEntry {
        TableEntry {
            depth: depth,
            score: score,
            age: age,
            bound: bound,
        }
    }
}

impl TT {
    pub fn new() -> TT {
        TT {
            table: [[0u64; 13]; 64],
            black_to_move: 0u64,
        }
    }

    pub fn from_board(board: &Board) -> TT {
        // Fill the 12x64 table
        let mut table: [[u64; 13]; 64] = [[0u64; 13]; 64];

        for sq in BitBoard::new(std::u64::MAX) {
            for piece in PIECES.iter() {
                table[sq.to_index()][*piece] = rand::thread_rng().gen::<u64>();
            }
        }
        
        TT {
            table: table,
            black_to_move: rand::thread_rng().gen::<u64>(),
        }
    }

    pub fn update(&mut self, board: &Board) {
        // Fill the 12x64 table
        let mut table: [[u64; 13]; 64] = [[0u64; 13]; 64];

        for sq in BitBoard::new(std::u64::MAX) {
            for piece in PIECES.iter() {
                table[sq.to_index()][*piece] = rand::thread_rng().gen::<u64>();
            }
        }

        self.table = table;
        self.black_to_move = rand::thread_rng().gen::<u64>();
    }

    pub fn hash(self, board: &Board) -> u64 {
        let mut hash = 0u64;

        if board.side_to_move() == Color::Black {
            hash ^= self.black_to_move;
        }

        for sq in BitBoard::new(std::u64::MAX) {
            if let Some(piece) = board.piece_on(sq) {
                hash ^= self.table[sq.to_index()][piece_index(piece, sq, board)];
            }
        }

        hash
    }
}

pub fn piece_index(piece: Piece, square: Square, board: &Board) -> usize {
    if let Some(piece) = board.piece_on(square) {
        if let Some(color) = board.color_on(square) {
            return piece.to_index() + 1 + if board.color_on(square).unwrap() == Color::White { 0 } else { 6 };
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}