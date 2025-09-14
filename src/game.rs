use std::thread::current;

use crate::board::*;
use crate::chess_board;
use crate::chess_board::*;
use crate::piece;
use crate::precompute_masks::*;
use crate::dir::*;
use crate::piece::*;

// https://www.chessprogramming.org/Blockers_and_Beyond
// Returns the pseudo legal moves for a sliding piece
fn get_piece_moves(square: usize, mut potential_squares: Board, all_pieces: Board) -> Board {
    let mut remaining_to_check = potential_squares & all_pieces;
    while remaining_to_check != 0 {
        let slot: usize = remaining_to_check.trailing_zeros() as usize;
        let dir = Dir::FROM_SQUARES_PAIRS[square][slot].unwrap();
        potential_squares &= !BBMASKS.rays[slot][dir as usize];
        remaining_to_check &= remaining_to_check - 1;
    }
    potential_squares
}

// Returns the pseudo attacked squares from a sliding pieces
// This can be done more efficient using, but I didn't feel like it: https://www.chessprogramming.org/Classical_Approach
fn get_piece_attacks(square: usize, mut potential_squares: Board, all_pieces: Board) -> Board {
    potential_squares &= all_pieces;
    let mut remaining_to_check = potential_squares;
    while remaining_to_check != 0 {
        let slot: usize = remaining_to_check.trailing_zeros() as usize;
        let dir = Dir::FROM_SQUARES_PAIRS[square][slot].unwrap();
        potential_squares &= !BBMASKS.rays[slot][dir as usize];
        remaining_to_check &= remaining_to_check - 1;
    }
    potential_squares
}

// https://www.chessprogramming.org/Square_Attacked_By#AnyAttackBySide
// https://www.chessprogramming.org/Checks_and_Pinned_Pieces_%28Bitboards%29
// Attacks to a square from sliding pieces 
fn attacks_to_square_sliding(chess_board: &ChessBoard, square_index: usize, by_side: PieceColor, potential_pieces: Board) -> Board {
    let opposite_side = PieceColor::opposite(by_side);
    // let all_pieces = chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize];
    let mut attacks: Board = 0;

    // None sliding pieces
    // for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::King] {
    //     let pieces = chess_board.pieces[piece_type as usize] & chess_board.all_pieces[by_side as usize];
    //     attacks |= BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square_index] & pieces
    // }

    for piece_type in [PieceType::Bishop, PieceType::Rook] {
        let pieces = (chess_board.pieces[piece_type as usize] | chess_board.pieces[PieceType::Queen as usize]) & chess_board.all_pieces[by_side as usize];
        attacks |= get_piece_attacks(square_index, BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square_index], potential_pieces) & pieces;
    }

    attacks
}

fn attacks_to_square(chess_board: &ChessBoard, square_index: usize, by_side: PieceColor, potential_pieces: Board) -> Board {
    let opposite_side = PieceColor::opposite(by_side);
    // let all_pieces = chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize];
    let mut attacks: Board = 0;

    // None sliding pieces
    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::King] {
        let pieces = chess_board.pieces[piece_type as usize] & chess_board.all_pieces[by_side as usize];
        attacks |= BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square_index] & pieces
    }

    // Sliding pieces
    for piece_type in [PieceType::Bishop, PieceType::Rook] {
        let pieces = (chess_board.pieces[piece_type as usize] | chess_board.pieces[PieceType::Queen as usize]) & chess_board.all_pieces[by_side as usize];
        attacks |= get_piece_attacks(square_index, BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square_index], potential_pieces) & pieces
    }

    attacks
}

fn all_squares_which_block_check(chess_board: &ChessBoard, square_index: usize, piece_color: PieceColor) -> Board {
    let rank = rank_index(square_index);
    let file = file_index(square_index);
    let square = single_square_board(rank as isize, file as isize);

    // Should not be called with the king
    assert_eq!(square & chess_board.pieces[PieceType::King as usize], 0);

    // Should only return one king
    let king_square: Board = chess_board.pieces[PieceType::King as usize] & chess_board.all_pieces[piece_color as usize];
    let king_index = king_square.trailing_zeros() as usize;
    // We don't include square because if it is pinned, then we should treat the whole line between king and the attacking piece as moveable
    let potential_pieces = (chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize]) & !square;
    
    // === Non sliding pieces ===
    let mut attacks = BBMASKS.pieces.attacks[piece_color as usize][PieceType::Knight as usize][king_index]
                       & chess_board.pieces[PieceType::Knight as usize];
    attacks |= BBMASKS.pieces.attacks[piece_color as usize][PieceType::Pawn as usize][king_index]
                       & chess_board.pieces[PieceType::Pawn as usize];
    attacks &= chess_board.all_pieces[PieceColor::opposite(piece_color) as usize];
    let attacks_count = attacks.count_ones();

    // === Sliding pieces ===
    let attacks_sliding = attacks_to_square_sliding(&chess_board, king_index, PieceColor::opposite(piece_color), potential_pieces);
    let attacks_sliding_count = attacks_sliding.count_ones();

    // No attacks to king => no move will allow check
    if attacks_count + attacks_sliding_count == 0 { return Board::MAX }
    // More than one attack on king => we have to move the king, so no move is possible for this piece
    if attacks_count + attacks_sliding_count > 1 { return 0; };
    if attacks_count == 1 { return attacks; }

    // The ifs makes sure attacks only has one bit set
    let attack_index = attacks_sliding.trailing_zeros() as usize;
    let dir = Dir::FROM_SQUARES_PAIRS[king_index][attack_index].unwrap();   // Should never return None
    // Create segment between king and attacking piece
    // TODO: Maybe precompute these segments
    (BBMASKS.rays[king_index][dir as usize] & BBMASKS.rays[attack_index][Dir::opposite(dir) as usize]) | attacks_sliding
}

pub fn get_legal_moves_bishop(chess_board: &ChessBoard, square: usize, piece_color: PieceColor) -> Board {
    let pseudo_legal_moves = get_piece_moves(square, BBMASKS.pieces.attacks[piece_color as usize][PieceType::Bishop as usize][square], chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize]) & !chess_board.all_pieces[piece_color as usize];
    let check_blocking_moves = all_squares_which_block_check(&chess_board, square, piece_color);
    check_blocking_moves & pseudo_legal_moves
}

pub fn get_legal_moves_rook(chess_board: &ChessBoard, square: usize, piece_color: PieceColor) -> Board {
    let pseudo_legal_moves = get_piece_moves(square, BBMASKS.pieces.attacks[piece_color as usize][PieceType::Rook as usize][square], chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize]) & !chess_board.all_pieces[piece_color as usize];
    let check_blocking_moves = all_squares_which_block_check(&chess_board, square, piece_color);
    check_blocking_moves & pseudo_legal_moves
}

pub fn get_legal_moves_queen(chess_board: &ChessBoard, square: usize, piece_color: PieceColor) -> Board{
    get_legal_moves_bishop(chess_board, square, piece_color) | get_legal_moves_rook(chess_board, square, piece_color)
}

pub fn get_legal_moves_knight(chess_board: &ChessBoard, square: usize, piece_color: PieceColor) -> Board {
    let pseudo_legal_moves = BBMASKS.pieces.attacks[piece_color as usize][PieceType::Knight as usize][square] & !chess_board.all_pieces[piece_color as usize];
    let check_blocking_moves = all_squares_which_block_check(&chess_board, square, piece_color);
    pseudo_legal_moves & check_blocking_moves
}

// TODO: Rename square index to just square and square to square_bit_board everywhere?
pub fn get_legal_moves_pawn(chess_board: &ChessBoard, square: usize, piece_color: PieceColor) -> Board {
    let mut pseudo_legal_moves = BBMASKS.pieces.attacks[piece_color as usize][PieceType::Pawn as usize][square] & (chess_board.all_pieces[PieceColor::opposite(piece_color) as usize] | chess_board.en_passant_mask);
    if (BBMASKS.pieces.pawn_moves[piece_color as usize][square] & (chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize])) == 0 {
        pseudo_legal_moves |= BBMASKS.pieces.pawn_moves[piece_color as usize][square];
        pseudo_legal_moves |= BBMASKS.pieces.pawn_double_moves[piece_color as usize][square] & !(chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize]);
    }
    let check_blocking_moves = all_squares_which_block_check(&chess_board, square, piece_color);
    pseudo_legal_moves & check_blocking_moves
}

// NOTE: Could also just calculate every square opposite side is attacking and take the intersection between it and the king attacks bit mask
// TODO: Maybe check if that is faster
pub fn get_legal_moves_king(chess_board: &ChessBoard, square: usize, piece_color: PieceColor) -> Board {
    // Calculate all legal "normal" moves
    let all_pieces = chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize];
    let bb_square = single_square_board(rank_index(square) as isize, file_index(square) as isize);
    let mut attacks = BBMASKS.pieces.attacks[piece_color as usize][PieceType::King as usize][square] & !chess_board.all_pieces[piece_color as usize];
    let mut remaining_checks = attacks;
    while remaining_checks != 0 {
        let index = remaining_checks.trailing_zeros() as usize;
        let bb_index= single_square_board(rank_index(index) as isize, file_index(index) as isize);
        if attacks_to_square(&chess_board, index, PieceColor::opposite(piece_color), (chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize]) & !bb_square) != 0 {
            attacks &= !bb_index;
        }
        // Remove LSb
        remaining_checks &= remaining_checks - 1;
    }

    // Calculate castling
    // Not allowed to castle if king is being checked
    if attacks_to_square(&chess_board, square, PieceColor::opposite(piece_color), chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize]) != 0 {
        return attacks;
    }

    let rank = rank_index(square);
    let file = file_index(square);
    let mut ca: CastlingAvailability = if chess_board.castling_availability[piece_color as usize].contains(CastlingAvailability::KingSide) {
        let mut allow_castle = true;
        for offset in [1, 2] {
            let index = square_index(rank, file - offset);
            allow_castle &= ((all_pieces & single_square_board(rank  as isize, (file - offset) as isize)) == 0) &
            (attacks_to_square(&chess_board, index, PieceColor::opposite(piece_color), all_pieces) == 0);
        }
        if allow_castle { CastlingAvailability::KingSide } else { CastlingAvailability::None }
    } else {
        CastlingAvailability::None
    };
    ca |= if chess_board.castling_availability[piece_color as usize].contains(CastlingAvailability::QueenSide) {
        let mut allow_castle = true;
        for offset in [1, 2] {
            let index = square_index(rank, file + offset);
            allow_castle &= ((all_pieces & single_square_board(rank as isize, (file + offset) as isize)) == 0) &
            (attacks_to_square(&chess_board, index, PieceColor::opposite(piece_color), all_pieces) == 0);
        }
        if allow_castle { CastlingAvailability::QueenSide } else { CastlingAvailability::None }
    } else {
        CastlingAvailability::None
    };

    attacks | BBMASKS.pieces.castling_moves[piece_color as usize][ca.bits()][square]
}

mod tests {
    use super::*;

    #[test]
    fn test_get_piece_moves() {
        // TODO: Replace with loop; Maybe use create helper module for this
        let index = square_index(3, 3);
        let moves = get_piece_moves(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Bishop as usize][index], 0xFFFF00000000FFFF);
        assert_eq!(moves, 0x0041221400142200);

        let index = square_index(3, 3);
        let moves = get_piece_moves(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Queen as usize][index], 0xEAA9489994605561);
        assert_eq!(moves, 0x0001021C141C2A49);

        let index = square_index(3, 4);
        let moves = get_piece_moves(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Rook as usize][index], 0xEAA9489994605561);
        assert_eq!(moves, 0x00000010EC101000);
    }

    #[test]
    fn test_get_piece_attacks() {
        let index = square_index(3, 3);
        let moves = get_piece_attacks(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Bishop as usize][index], 0xFFFF00000000FFFF);
        assert_eq!(moves, 0x0041000000002200);

        let index = square_index(3, 3);
        let moves = get_piece_attacks(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Queen as usize][index], 0xEAA9489994605561);
        assert_eq!(moves, 0x0001001814000041);

        let index = square_index(3, 4);
        let moves = get_piece_attacks(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Rook as usize][index], 0xEAA9489994605561);
        assert_eq!(moves, 0x0000001084001000);    
    }

    #[test]
    fn test_all_squares_which_block_check() {
        // https://lichess.org/editor/4k3/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(4, 2), PieceColor::Black);
        assert_eq!(squares, 0x0008080808080808);

        // https://lichess.org/editor/8/7p/2n2Pp1/2bqkbK1/p2P2PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bqkbK1/p2P2PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(4, 2), PieceColor::Black);
        assert_eq!(squares, 0);

        // https://lichess.org/editor/8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p1kP2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p1kP2/1RP5/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(4, 2), PieceColor::Black);
        assert_eq!(squares, 0x0000000000000808);

        // https://lichess.org/editor/8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RPk4/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RPk4/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(4, 2), PieceColor::Black);
        assert_eq!(squares, 0x0000000000000008);

        // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(3, 3), PieceColor::Black);
        assert_eq!(squares, 0x0008080808080808);

        // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/5P1p/2n3p1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(3, 3), PieceColor::Black);
        assert_eq!(squares, 0);

        // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/6Pp/2n1R1p1/K1bq4/p2Pb1PR/P1p2P2/2P5/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(3, 3), PieceColor::Black);
        assert_eq!(squares, 0x0008080000000000);

        // https://lichess.org/editor/8/6Pp/2n1R1p1/K1bq4/p2PbkPR/P1p2P2/2P5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/6Pp/2n1R1p1/K1bq4/p2PbkPR/P1p2P2/2P5/3BQ3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(3, 3), PieceColor::Black);
        assert_eq!(squares, Board::MAX);

        // https://lichess.org/editor/1q6/3k2pp/2n1R3/2b2K2/p2Pb1BR/P1p2P2/2P5/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("1q6/3k2pp/2n1R3/2b2K2/p2Pb1BR/P1p2P2/2P5/4Q3 w - - 0 1");
        let squares = all_squares_which_block_check(&chess_board, square_index(2, 2), PieceColor::White);
        assert_eq!(squares, 0x0000000008000000);
    }

    #[test]
    fn test_get_legal_moves_bishop() {
        // https://lichess.org/editor/1q2B3/3k2pp/2n1R3/2b2K2/p2Pb2R/P1p2P2/2P5/4Q3_w_-_-_0_1?color=white 
        let chess_board = ChessBoard::new("1q2B3/3k2pp/2n1R3/2b2K2/p2Pb2R/P1p2P2/2P5/4Q3 w - - 0 1");
        let squares = get_legal_moves_bishop(&chess_board, square_index(3, 3), PieceColor::Black);
        assert_eq!(squares, 0);

        // https://lichess.org/editor/1q2B3/3k2pp/2n1R1b1/2b2K2/p2P3R/P1p2P2/2P5/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("1q2B3/3k2pp/2n1R1b1/2b2K2/p2P3R/P1p2P2/2P5/4Q3 w - - 0 1");
        let squares = get_legal_moves_bishop(&chess_board, square_index(5, 1), PieceColor::Black);
        assert_eq!(squares, 0x0800000000000000);

        let chess_board = ChessBoard::new("1q6/3k1Bpp/2n1R1b1/2b2K2/p2P3R/P1p2P2/2P5/4Q3 w - - 0 1");
        let squares = get_legal_moves_bishop(&chess_board, square_index(5, 1), PieceColor::Black);
        assert_eq!(squares, 0x0004000500000000);

        let chess_board = ChessBoard::new("1k6/3KnRpp/8/2bB4/p4P1R/PPpq1b2/2P5/4Q3 w - - 0 1");
        let squares = get_legal_moves_bishop(&chess_board, square_index(4, 4), PieceColor::White);
        assert_eq!(squares, 0);

        // https://lichess.org/editor/6k1/1K2nRpp/8/3B4/p4P1R/PPpq1b2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6k1/1K2nRpp/8/3B4/p4P1R/PPpq1b2/2P3b1/4Q3 w - - 0 1");
        let squares = get_legal_moves_bishop(&chess_board, square_index(4, 4), PieceColor::White);
        assert_eq!(squares, 0x0000200008040000);
    }

    #[test]
    fn test_get_legal_moves_rook() {
        // https://lichess.org/editor/3R2k1/1K4pp/3n4/3B4/p4P1R/PPpq1b2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("3R2k1/1K4pp/3n4/3B4/p4P1R/PPpq1b2/2P3b1/4Q3 w - - 0 1");
        let squares = get_legal_moves_rook(&chess_board, square_index(7, 4), PieceColor::White);
        assert_eq!(squares, 0x0000100000000000);

        // https://lichess.org/editor/6k1/1K4pp/3n4/3B4/p4P1R/PPpqRb2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6k1/1K4pp/3n4/3B4/p4P1R/PPpqRb2/2P3b1/4Q3 w - - 0 1");
        let squares = get_legal_moves_rook(&chess_board, square_index(2, 3), PieceColor::White);
        assert_eq!(squares, 0);

        // https://lichess.org/editor/6k1/1K4pp/3n4/3B4/p4P1R/PPpqRb2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4R1k1/6pp/3nr3/3B4/p4q1R/PPpK4/2P3b1/4Q3 w - - 0 1");
        let squares = get_legal_moves_rook(&chess_board, square_index(5, 3), PieceColor::Black);
        assert_eq!(squares, 0);

        // https://lichess.org/editor/2Q1r1k1/6pp/3n4/8/p3Bq1R/PPpK4/2P3b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("2Q1r1k1/6pp/3n4/8/p3Bq1R/PPpK4/2P3b1/8 w - - 0 1");
        let squares = get_legal_moves_rook(&chess_board, square_index(7, 3), PieceColor::Black);
        assert_eq!(squares, 0x3400000000000000);
    }

    #[test]
    fn test_get_legal_moves_queen() {
        // If both rook and bishop are correct, then queen should be correct too
        // https://lichess.org/editor/4r2p/5Q1p/3k4/2n1P1q1/p4B2/PPpK1R2/6b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4r2p/5Q1p/3k4/2n1P1q1/p4B2/PPpK1R2/6b1/8 w - - 0 1");
        let squares = get_legal_moves_rook(&chess_board, square_index(7, 3), PieceColor::Black);
        assert_eq!(squares, 0x0000000800000000);
    }

    #[test]
    fn test_get_legal_moves_knight() {
        // https://lichess.org/editor/4r2p/1k2Q2p/8/3nP1q1/p4B2/PPpK1R2/6b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4r2p/1k2Q2p/8/3nP1q1/p4B2/PPpK1R2/6b1/8 w - - 0 1");
        let squares = get_legal_moves_knight(&chess_board, square_index(4, 4), PieceColor::Black);
        assert_eq!(squares, 0x0028000000000000);

        // https://lichess.org/editor/4r2p/1k1nQ2p/8/4P1q1/p4B2/PPpK1R2/6b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4r2p/1k1nQ2p/8/4P1q1/p4B2/PPpK1R2/6b1/8 w - - 0 1");
        let squares = get_legal_moves_knight(&chess_board, square_index(6, 4), PieceColor::Black);
        assert_eq!(squares, 0);
    }

    #[test]
    fn test_get_legal_moves_pawn() {
        // https://lichess.org/editor/1k2r2p/4Q2p/5n2/6q1/pPp1K3/P2RB3/2p3b1/3B4_w_-_b3_0_1?color=white
        let chess_board = ChessBoard::new("1k2r2p/4Q2p/5n2/6q1/pPp1K3/P2RB3/2p3b1/3B4 w - b3 0 1");
        let squares = get_legal_moves_pawn(&chess_board, square_index(3, 5), PieceColor::Black);
        assert_eq!(squares, 0x0000000000700000);

        // https://lichess.org/editor/6rp/3R1n1p/p1pQ2k1/8/q3K3/P3B2b/p1P5/3B4_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6rp/3R1n1p/p1pQ2k1/8/q3K3/P3B2b/p1P5/3B4 w - - 0 1");
        let squares = get_legal_moves_pawn(&chess_board, square_index(1, 5), PieceColor::White);
        assert_eq!(squares, 0x0000000020000000);

        // https://lichess.org/editor/6rp/3R1n1p/p1pQ2k1/8/q1b5/P3B3/p1P1K3/3B4_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6rp/3R1n1p/p1pQ2k1/8/q1b5/P3B3/p1P1K3/3B4 w - - 0 1");
        let squares = get_legal_moves_pawn(&chess_board, square_index(1, 5), PieceColor::White);
        assert_eq!(squares, 0);
    }

    #[test]
    fn test_get_legal_moves_king() {
        // https://lichess.org/editor/6rp/5n1p/p1pQ2k1/8/q1b5/P3B3/p1P5/3BK2R_w_K_-_0_1?color=white
        let chess_board = ChessBoard::new("6rp/5n1p/p1pQ2k1/8/q1b5/P3B3/p1P5/3BK2R w K - 0 1");
        let squares = get_legal_moves_king(&chess_board, square_index(0, 3), PieceColor::White);
        assert_eq!(squares, 0x0000000000001400);

        // https://lichess.org/editor/1r2n2p/5k1p/p1pQ4/8/2B2q2/P2bB3/p1P5/R3K3_w_Q_-_0_1?color=white
        let chess_board = ChessBoard::new("1r2n2p/5k1p/p1pQ4/8/2B2q2/P2bB3/p1P5/R3K3 w Q - 0 1");
        let squares = get_legal_moves_king(&chess_board, square_index(0, 3), PieceColor::White);
        assert_eq!(squares, 0x0000000000001030);

        // https://lichess.org/editor/r3k2r/3p3p/p3B3/8/5b2/PQ5q/p7/R3K2R_b_KQkq_-_0_1?color=white
        let chess_board = ChessBoard::new("r3k2r/3p3p/p3B3/8/5b2/PQ5q/p7/R3K2R b KQkq - 0 1");
        let squares = get_legal_moves_king(&chess_board, square_index(7, 3), PieceColor::Black);
        assert_eq!(squares, 0x3408000000000000);
    }
}