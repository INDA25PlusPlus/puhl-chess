use crate::chess_board::*;
use crate::board::*;
use crate::precompute_masks::*;
use crate::piece::*;

fn remove_from_all(chess_board: &mut ChessBoard, bb_square: BitBoard) {
    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King ] {
        chess_board.pieces[piece_type as usize] &= !bb_square;
    }
    chess_board.all_pieces[PieceColor::White as usize] &= !bb_square;
    chess_board.all_pieces[PieceColor::Black as usize] &= !bb_square;
}

pub fn move_pawn(chess_board: &mut ChessBoard, square: usize, piece_move: BitBoard) {
    let bb_square = get_single_bit_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    // chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Pawn as usize] &= !bb_square;
    chess_board.all_pieces[chess_board.current_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Pawn as usize] |= piece_move;
    chess_board.all_pieces[chess_board.current_color as usize] |= piece_move;

    // en passant
    if piece_move & chess_board.en_passant_mask != 0 {
        // The attacked piece could only be a pawn
        chess_board.pieces[PieceType::Pawn as usize] &= !BBMASKS.pieces.en_passant_attacks[PieceColor::opposite(chess_board.current_color) as usize][chess_board.en_passant_mask.trailing_zeros() as usize];
        chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize] &= !BBMASKS.pieces.en_passant_attacks[PieceColor::opposite(chess_board.current_color) as usize][chess_board.en_passant_mask.trailing_zeros() as usize];
    }

    chess_board.en_passant_mask = 0;

    // double move
    if piece_move & BBMASKS.pieces.pawn_double_moves[chess_board.current_color as usize][square] != 0 {
        chess_board.en_passant_mask = BBMASKS.pieces.pawn_moves[chess_board.current_color as usize][square];
    }
    if rank_index(piece_index) == 0 || rank_index(piece_index) == 7 {
        chess_board.promotion_mask = piece_move;
    }

    let opposite_color = PieceColor::opposite(chess_board.current_color) as usize;
    chess_board.castling_availability[opposite_color] &= !BBMASKS.pieces.castling_corners[opposite_color][piece_index];
    // TODO: promotion
}

pub fn move_knight(chess_board: &mut ChessBoard, square: usize, piece_move: BitBoard) {
    let bb_square = get_single_bit_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Knight as usize] &= !bb_square;
    chess_board.all_pieces[chess_board.current_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Knight as usize] |= piece_move;
    chess_board.all_pieces[chess_board.current_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    let opposite_color = PieceColor::opposite(chess_board.current_color) as usize;
    chess_board.castling_availability[opposite_color] &= !BBMASKS.pieces.castling_corners[opposite_color][piece_index];
}

pub fn move_bishop(chess_board: &mut ChessBoard, square: usize, piece_move: BitBoard) {
    let bb_square = get_single_bit_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Bishop as usize] &= !bb_square;
    chess_board.all_pieces[chess_board.current_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Bishop as usize] |= piece_move;
    chess_board.all_pieces[chess_board.current_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    let opposite_color = PieceColor::opposite(chess_board.current_color) as usize;
    chess_board.castling_availability[opposite_color] &= !BBMASKS.pieces.castling_corners[opposite_color][piece_index];
}
pub fn move_rook(chess_board: &mut ChessBoard, square: usize, piece_move: BitBoard) {
    let bb_square = get_single_bit_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Rook as usize] &= !bb_square;
    chess_board.all_pieces[chess_board.current_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Rook as usize] |= piece_move;
    chess_board.all_pieces[chess_board.current_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    let removed_castling_availability = BBMASKS.pieces.castling_corners[chess_board.current_color as usize][square];
    chess_board.castling_availability[chess_board.current_color as usize] &= !removed_castling_availability;

    let opposite_color = PieceColor::opposite(chess_board.current_color) as usize;
    chess_board.castling_availability[opposite_color] &= !BBMASKS.pieces.castling_corners[opposite_color][piece_index];
}

pub fn move_queen(chess_board: &mut ChessBoard, square: usize, piece_move: BitBoard) {
    let bb_square = get_single_bit_board(rank_index(square) as isize, file_index(square) as isize);
    // TODO: Maybe first check if it is a capture, because otherwise we don't need to clear from the rest of the boards
    // TODO: Instead of bool, CheckBoard.white_turn should be replaced with an PieceColor enum
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::Queen as usize] &= !bb_square;
    chess_board.all_pieces[chess_board.current_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::Queen as usize] |= piece_move;
    chess_board.all_pieces[chess_board.current_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    let opposite_color = PieceColor::opposite(chess_board.current_color) as usize;
    // ==================================== NOTE: FIX THIS, WHEN NOT THE MASK, THE REST OF THE BITS THAT ARE NOT USED IN THE MASK WILL GET SET
    chess_board.castling_availability[opposite_color] &= !BBMASKS.pieces.castling_corners[opposite_color][piece_index];
}

pub fn move_king(chess_board: &mut ChessBoard, square: usize, piece_move: BitBoard) {
    let bb_square = get_single_bit_board(rank_index(square) as isize, file_index(square) as isize);
    let piece_index = piece_move.trailing_zeros() as usize;

    // Clear destination piece (not possible for it to be the same color type)
    remove_from_all(chess_board, piece_move);
    chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize] &= !piece_move;
    // Clear source piece
    chess_board.pieces[PieceType::King as usize] &= !bb_square;
    chess_board.all_pieces[chess_board.current_color as usize] &= !bb_square;
    // Add new destination piece
    chess_board.pieces[PieceType::King as usize] |= piece_move;
    chess_board.all_pieces[chess_board.current_color as usize] |= piece_move;

    chess_board.en_passant_mask = 0;

    // Move rook if castling
    let mask = BBMASKS.pieces.castling_rook_moves[chess_board.current_color as usize][chess_board.castling_availability[chess_board.current_color as usize].bits()][piece_index];
    assert!(chess_board.castling_availability[chess_board.current_color as usize].bits() <= 3);
    chess_board.pieces[PieceType::Rook as usize] ^= mask;
    chess_board.all_pieces[chess_board.current_color as usize] ^= mask;

    // Clear own castling availability
    chess_board.castling_availability[chess_board.current_color as usize] = CastlingAvailability::None;

    // Remove one of opponent castling availability if king takes one of opponents rooks in the corner
    let opposite_color = PieceColor::opposite(chess_board.current_color) as usize;
    chess_board.castling_availability[opposite_color] &= !BBMASKS.pieces.castling_corners[opposite_color][piece_index];
}