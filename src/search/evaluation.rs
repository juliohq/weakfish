use chess::{
    Board,
    BoardStatus,
    Piece,
    Square,
    MoveGen,
    Color,
    BitBoard,
    ALL_FILES,
    ALL_RANKS,
};

use crate::constants::{
    CHECKMATE,
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    CENTER_DIST,
    PAWNS,
    BISHOPS,
    KNIGHTS,
    ROOKS,
    QUEENS,
    KINGS,
    value,
};

pub fn evaluate(b: &Board) -> f32 {
    let mut score = 0f32;
    
    // Mate/stalemate
    if b.status() == BoardStatus::Checkmate {
        return -CHECKMATE;
    } else if b.status() == BoardStatus::Stalemate {
        return 0f32;
    }

    // Convenience variables
    let me = !b.side_to_move();
    let their = b.side_to_move();

    let my_pieces = *b.color_combined(me);
    let their_pieces = *b.color_combined(their);

    // My material
    for square in my_pieces {
        let piece = b.piece_on(square).unwrap();
        
        score += value(piece);
        
        // Threats against the opponent
        let attacked_squares = match b.piece_on(square).unwrap() {
            Piece::Pawn => {
                chess::get_pawn_attacks(square, me, their_pieces)
            },
            Piece::Bishop => {
                chess::get_bishop_rays(square)
            },
            Piece::Knight => {
                chess::get_knight_moves(square)
            },
            Piece::Rook => {
                chess::get_rook_rays(square)
            },
            Piece::Queen => {
                chess::get_rook_rays(square) & chess::get_bishop_rays(square)
            },
            Piece::King => {
                chess::get_king_moves(square)
            },
        };
        let threats = their_pieces & attacked_squares;
        
        for threat in threats {
            score += value(b.piece_on(threat).unwrap());
        }
        
        // Attacked squares
        match piece {
            Piece::Pawn => {
                score += chess::get_pawn_moves(square, me, their_pieces).count() as f32;
            },
            Piece::Bishop => {
                score += chess::get_bishop_rays(square).count() as f32;
            },
            Piece::Knight => {
                score += chess::get_knight_moves(square).count() as f32;
            },
            Piece::Rook => {
                score += chess::get_rook_rays(square).count() as f32;
            },
            Piece::Queen => {
                score += (chess::get_bishop_rays(square) & chess::get_rook_rays(square)).count() as f32;
            },
            Piece::King => {
                score += chess::get_knight_moves(square).count() as f32;
            },
        }

        // Center distance
        score -= CENTER_DIST[square.to_index()] as f32 * 0.3;
        
        // Piece-Square Tables
        let idx = square.to_index();
        let tab_val = match piece {
            Piece::Pawn => {
                PAWNS[idx]
            },
            Piece::Bishop => {
                BISHOPS[idx]
            },
            Piece::Knight => {
                KNIGHTS[idx]
            },
            Piece::Rook => {
                ROOKS[idx]
            },
            Piece::Queen => {
                QUEENS[idx]
            },
            Piece::King => {
                KINGS[idx]
            },
        };
        
        score += tab_val as f32;
        
        // Isolated pawns
        if b.piece_on(square) == Some(Piece::Pawn) {
            let adjacents = chess::get_adjacent_files(square.get_file());
            let mut is_isolated = true;
            
            for adj_sq in adjacents {
                if b.piece_on(adj_sq) == Some(Piece::Pawn) {
                    is_isolated = false;
                    break;
                }
            }
            
            if is_isolated {
                score += 0.5f32;
            }
        }
    }
    
    // Check doubled pawns
    let mut my_doubled_pawns = 0.0;
    let mut their_doubled_pawns = 0.0;
    
    for file in ALL_FILES.iter() {
        for rank in ALL_RANKS.iter() {
            let square = Square::make_square(*rank, *file);
            
            if b.piece_on(square) == Some(Piece::Pawn) {
                if b.color_on(square).unwrap() == me {
                    my_doubled_pawns += 0.5f32;
                } else {
                    their_doubled_pawns += 0.5f32;
                }
            }
        }
        score -= my_doubled_pawns;
        my_doubled_pawns = 0.0;
        score += their_doubled_pawns;
        their_doubled_pawns = 0.0;
    }
    
    // Their material
    for square in their_pieces {
        let piece = b.piece_on(square).unwrap();
        
        score -= value(piece);
        
        // Threats against me
        let attacked_squares = match b.piece_on(square).unwrap() {
            Piece::Pawn => {
                chess::get_pawn_attacks(square, their, my_pieces)
            },
            Piece::Bishop => {
                chess::get_bishop_rays(square)
            },
            Piece::Knight => {
                chess::get_knight_moves(square)
            },
            Piece::Rook => {
                chess::get_rook_rays(square)
            },
            Piece::Queen => {
                chess::get_rook_rays(square) & chess::get_bishop_rays(square)
            },
            Piece::King => {
                chess::get_king_moves(square)
            },
        };
        let threats = my_pieces & attacked_squares;
        
        for threat in threats {
            score -= value(b.piece_on(threat).unwrap());
        }

        // Squares the opponent attacks
        match piece {
            Piece::Pawn => {
                score -= chess::get_pawn_moves(square, their, my_pieces).count() as f32;
            },
            Piece::Bishop => {
                score -= chess::get_bishop_rays(square).count() as f32;
            },
            Piece::Knight => {
                score -= chess::get_knight_moves(square).count() as f32;
            },
            Piece::Rook => {
                score -= chess::get_rook_rays(square).count() as f32;
            },
            Piece::Queen => {
                score -= (chess::get_bishop_rays(square) & chess::get_rook_rays(square)).count() as f32;
            },
            Piece::King => {
                score -= chess::get_knight_moves(square).count() as f32;
            },
        }

        // Center distance
        score += CENTER_DIST[square.to_index()] as f32 * 0.3;
        
        // Piece-Square Tables
        let idx = 63 - square.to_index();
        let tab_val = match piece {
            Piece::Pawn => {
                PAWNS[idx]
            },
            Piece::Bishop => {
                BISHOPS[idx]
            },
            Piece::Knight => {
                KNIGHTS[idx]
            },
            Piece::Rook => {
                ROOKS[idx]
            },
            Piece::Queen => {
                QUEENS[idx]
            },
            Piece::King => {
                KINGS[idx]
            },
        };
        
        score -= tab_val as f32;
        
        // Isolated pawns
        if b.piece_on(square) == Some(Piece::Pawn) {
            let adjacents = chess::get_adjacent_files(square.get_file());
            let mut is_isolated = true;
            
            for adj_sq in adjacents {
                if b.piece_on(adj_sq) == Some(Piece::Pawn) {
                    is_isolated = false;
                    break;
                }
            }
            
            if is_isolated {
                score += 0.5f32;
            }
        }
    }
    
    // Mobility
    // Their
    score -= MoveGen::new_legal(&b).count() as f32 * 0.1;
    
    // Ours
    if let Some(nb) = b.null_move() {
        score += MoveGen::new_legal(&nb).count() as f32 * 0.1;
    }
    
    // Board control = pieces square + attacked squares
    
    score
}
