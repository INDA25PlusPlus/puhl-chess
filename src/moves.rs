use crate::chess_board::*;
use crate::board::*;
use crate::piece;
use crate::precompute_masks::*;
use crate::piece::*;

fn remove_from_all(chess_board: &mut ChessBoard, bb_square: Board) {
    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King ] {
        chess_board.pieces[piece_type as usize] &= !bb_square;
    }
    chess_board.all_pieces[PieceColor::White as usize] &= !bb_square;
    chess_board.all_pieces[PieceColor::Black as usize] &= !bb_square;
}

pub fn move_pawn(chess_board: &mut ChessBoard, square: usize, piece_move: Board) {
    let bb_square = single_square_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_color = if chess_board.white_turn { PieceColor::White } else { PieceColor::Black };
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Pawn as usize] &= !bb_square;
    chess_board.all_pieces[piece_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Pawn as usize] |= piece_move;
    chess_board.all_pieces[piece_color as usize] |= piece_move;

    // en passant
    if piece_move & chess_board.en_passant_mask != 0 {
        // The attacked piece could only be a pawn
        chess_board.pieces[PieceType::Pawn as usize] &= !BBMASKS.pieces.en_passant_attack[PieceColor::opposite(piece_color) as usize][chess_board.en_passant_mask.trailing_zeros() as usize];
        chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] &= !BBMASKS.pieces.en_passant_attack[PieceColor::opposite(piece_color) as usize][chess_board.en_passant_mask.trailing_zeros() as usize];
    }

    chess_board.en_passant_mask = 0;

    // double move
    if piece_move & BBMASKS.pieces.pawn_double_moves[piece_color as usize][square] != 0 {
        chess_board.en_passant_mask = BBMASKS.pieces.pawn_moves[piece_color as usize][square];
    }
    if (piece_index == square_index(0, 0)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(0, 7)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::QueenSide;
    }
    if (piece_index == square_index(7, 0)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(7, 7)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::QueenSide;
    }
    
    // TODO: promotion
}

pub fn move_knight(chess_board: &mut ChessBoard, square: usize, piece_move: Board) {
    let bb_square = single_square_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_color = if chess_board.white_turn { PieceColor::White } else { PieceColor::Black };
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Knight as usize] &= !bb_square;
    chess_board.all_pieces[piece_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Knight as usize] |= piece_move;
    chess_board.all_pieces[piece_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    if (piece_index == square_index(0, 0)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(0, 7)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::QueenSide;
    }
    if (piece_index == square_index(7, 0)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(7, 7)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::QueenSide;
    }
}

pub fn move_bishop(chess_board: &mut ChessBoard, square: usize, piece_move: Board) {
    let bb_square = single_square_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_color = if chess_board.white_turn { PieceColor::White } else { PieceColor::Black };
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Bishop as usize] &= !bb_square;
    chess_board.all_pieces[piece_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Bishop as usize] |= piece_move;
    chess_board.all_pieces[piece_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    if (piece_index == square_index(0, 0)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(0, 7)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::QueenSide;
    }
    if (piece_index == square_index(7, 0)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(7, 7)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::QueenSide;
    }
}
pub fn move_rook(chess_board: &mut ChessBoard, square: usize, piece_move: Board) {
    let bb_square = single_square_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_color = if chess_board.white_turn { PieceColor::White } else { PieceColor::Black };
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Rook as usize] &= !bb_square;
    chess_board.all_pieces[piece_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Rook as usize] |= piece_move;
    chess_board.all_pieces[piece_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    // TODO: Update castling availability
    // TODO: Make more generic / precompute
    if (square == square_index(0, 0)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::KingSide;
    }
    if (square == square_index(0, 7)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::QueenSide;
    }
    if (square == square_index(7, 0)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::KingSide;
    }
    if (square == square_index(7, 7)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::QueenSide;
    }

    if (piece_index == square_index(0, 0)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(0, 7)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::QueenSide;
    }
    if (piece_index == square_index(7, 0)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(7, 7)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::QueenSide;
    }
}

pub fn move_queen(chess_board: &mut ChessBoard, square: usize, piece_move: Board) {
    let bb_square = single_square_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_color = if chess_board.white_turn { PieceColor::White } else { PieceColor::Black };
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Queen as usize] &= !bb_square;
    chess_board.all_pieces[piece_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Queen as usize] |= piece_move;
    chess_board.all_pieces[piece_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    if (piece_index == square_index(0, 0)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(0, 7)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::QueenSide;
    }
    if (piece_index == square_index(7, 0)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(7, 7)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::QueenSide;
    }
}

pub fn move_king(chess_board: &mut ChessBoard, square: usize, piece_move: Board) {
    let bb_square = single_square_board(rank_index(square) as isize, file_index(square) as isize);
    let piece_color = if chess_board.white_turn { PieceColor::White } else { PieceColor::Black };
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::King as usize] &= !bb_square;
    chess_board.all_pieces[piece_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::King as usize] |= piece_move;
    chess_board.all_pieces[piece_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    // Castling
    // TODO: Make more generic
    if (piece_move & BBMASKS.pieces.castling_moves[PieceColor::White as usize][CastlingAvailability::KingSide.bits()][square] != 0) {
        chess_board.pieces[PieceType::Rook as usize] &= !single_square_board(0, 0);
        chess_board.all_pieces[piece_color as usize] &= !single_square_board(0, 0);
        chess_board.pieces[PieceType::Rook as usize] |= single_square_board(0, 2);
        chess_board.all_pieces[piece_color as usize] |= single_square_board(0, 2);
    }
    if (piece_move & BBMASKS.pieces.castling_moves[PieceColor::White as usize][CastlingAvailability::QueenSide.bits()][square] != 0) {
        chess_board.pieces[PieceType::Rook as usize] &= !single_square_board(0, 7);
        chess_board.all_pieces[piece_color as usize] &= !single_square_board(0, 7);
        chess_board.pieces[PieceType::Rook as usize] |= single_square_board(0, 4);
        chess_board.all_pieces[piece_color as usize] |= single_square_board(0, 4);
    }
    if (piece_move & BBMASKS.pieces.castling_moves[PieceColor::Black as usize][CastlingAvailability::KingSide.bits()][square] != 0) {
        chess_board.pieces[PieceType::Rook as usize] &= !single_square_board(7, 0);
        chess_board.all_pieces[piece_color as usize] &= !single_square_board(7, 0);
        chess_board.pieces[PieceType::Rook as usize] |= single_square_board(7, 2);
        chess_board.all_pieces[piece_color as usize] |= single_square_board(7, 2);
    }
    if (piece_move & BBMASKS.pieces.castling_moves[PieceColor::Black as usize][CastlingAvailability::QueenSide.bits()][square] != 0) {
        chess_board.pieces[PieceType::Rook as usize] &= !single_square_board(7, 7);
        chess_board.all_pieces[piece_color as usize] &= !single_square_board(7, 7);
        chess_board.pieces[PieceType::Rook as usize] |= single_square_board(7, 4);
        chess_board.all_pieces[piece_color as usize] |= single_square_board(7, 4);
    }

    chess_board.castling_availability[piece_color as usize] = CastlingAvailability::None;

    if (piece_index == square_index(0, 0)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(0, 7)) {
        chess_board.castling_availability[PieceColor::White as usize] &= !CastlingAvailability::QueenSide;
    }
    if (piece_index == square_index(7, 0)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::KingSide;
    }
    if (piece_index == square_index(7, 7)) {
        chess_board.castling_availability[PieceColor::Black as usize] &= !CastlingAvailability::QueenSide;
    }
}

mod tests {
    use crate::chess_board;

    use super::*;

    // #[test]
    // fn test_basic_move() {
    //     let mut chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    //     move_knight(&mut chess_board, square_index(0, 1), single_square_board(2, 2));

    // }
    
    // fn test_pawn_move() {

    // }


}