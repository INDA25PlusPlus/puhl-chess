use super::board::*;
use super::chess_board::*;
use super::precompute_masks::*;
use super::dir::*;
use super::piece::*;

type MoveGenFn = fn(&ChessBoard, usize) -> BitBoard;

pub fn get_pieces_attacking_king(chess_board: &ChessBoard, by_side: PieceColor) -> BitBoard {
    let bb_king = chess_board.pieces[PieceType::King as usize] & chess_board.all_pieces[PieceColor::opposite(by_side) as usize];
    assert!(bb_king != 0);
    let king_square = bb_king.trailing_zeros() as usize;

    get_pieces_attacking_square(chess_board, king_square, by_side, chess_board.all_pieces())
}

pub fn get_move_generator(piece_type: PieceType) -> MoveGenFn {
    match piece_type {
        PieceType::Pawn   => get_legal_moves_pawn,
        PieceType::Knight => get_legal_moves_knight,
        PieceType::Bishop => get_legal_moves_bishop,
        PieceType::Rook   => get_legal_moves_rook,
        PieceType::Queen  => get_legal_moves_queen,
        PieceType::King   => get_legal_moves_king,
    }
}

fn get_legal_moves_pawn(chess_board: &ChessBoard, square: usize) -> BitBoard {
    let mut pseudo_legal_moves = BBMASKS.pieces.attacks[chess_board.current_color as usize][PieceType::Pawn as usize][square] 
                                    & chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize];
    if (BBMASKS.pieces.attacks[chess_board.current_color as usize][PieceType::Pawn as usize][square] & chess_board.en_passant_mask) != 0 
            && !does_en_passant_cause_check(chess_board, square) {
        pseudo_legal_moves |= chess_board.en_passant_mask;
    }

    // Checks if another piece is blocking the pawn move
    if (BBMASKS.pieces.pawn_moves[chess_board.current_color as usize][square] & chess_board.all_pieces()) == 0 {
        pseudo_legal_moves |= BBMASKS.pieces.pawn_moves[chess_board.current_color as usize][square];
        pseudo_legal_moves |= BBMASKS.pieces.pawn_double_moves[chess_board.current_color as usize][square] & !chess_board.all_pieces();
    }

    let mut check_blocking_moves = get_squares_blocking_check(&chess_board, square);
    if chess_board.en_passant_mask != 0 {
        let en_passant_square = chess_board.en_passant_mask.trailing_zeros() as usize;
        let opposite_color = PieceColor::opposite(chess_board.current_color) as usize;
        if BBMASKS.pieces.en_passant_attacks[opposite_color][en_passant_square] & check_blocking_moves != 0 {
            check_blocking_moves |= chess_board.en_passant_mask;
        }
    }
    pseudo_legal_moves & check_blocking_moves
}

fn get_legal_moves_bishop(chess_board: &ChessBoard, square: usize) -> BitBoard {
    let moves_on_empty_board = BBMASKS.pieces.attacks[chess_board.current_color as usize][PieceType::Bishop as usize][square];
    let pseudo_legal_moves = pseudo_legal_moves_sliding_piece(square, moves_on_empty_board, chess_board.all_pieces()) 
                                    & !chess_board.all_pieces[chess_board.current_color as usize];
    let check_blocking_moves = get_squares_blocking_check(&chess_board, square);
    check_blocking_moves & pseudo_legal_moves
}

fn get_legal_moves_rook(chess_board: &ChessBoard, square: usize) -> BitBoard {
    let moves_on_empty_board = BBMASKS.pieces.attacks[chess_board.current_color as usize][PieceType::Rook as usize][square];
    let pseudo_legal_moves = pseudo_legal_moves_sliding_piece(square, moves_on_empty_board, chess_board.all_pieces()) 
                                    & !chess_board.all_pieces[chess_board.current_color as usize];
    let check_blocking_moves = get_squares_blocking_check(&chess_board, square);
    check_blocking_moves & pseudo_legal_moves
}

fn get_legal_moves_queen(chess_board: &ChessBoard, square: usize) -> BitBoard{
    get_legal_moves_bishop(chess_board, square) | get_legal_moves_rook(chess_board, square)
}

fn get_legal_moves_knight(chess_board: &ChessBoard, square: usize) -> BitBoard {
    let pseudo_legal_moves = BBMASKS.pieces.attacks[chess_board.current_color as usize][PieceType::Knight as usize][square] & !chess_board.all_pieces[chess_board.current_color as usize];
    let check_blocking_moves = get_squares_blocking_check(&chess_board, square);
    pseudo_legal_moves & check_blocking_moves
}

// NOTE: Could also just calculate every square opposite side is attacking and take the intersection between it and the king attacks bit mask
// TODO: Maybe check if that is faster
fn get_legal_moves_king(chess_board: &ChessBoard, square: usize) -> BitBoard {
    // Calculate all legal moves except castling
    let bb_square = get_single_bit_board(rank_index(square) as isize, file_index(square) as isize);
    let mut legal_moves = BBMASKS.pieces.attacks[chess_board.current_color as usize][PieceType::King as usize][square] & !chess_board.all_pieces[chess_board.current_color as usize];
    let mut remaining_checks = legal_moves;
    while remaining_checks != 0 {
        let potential_move = pop_lsb(&mut remaining_checks);
        let bb_potential_move = (1 as BitBoard) << potential_move;
        if get_pieces_attacking_square(&chess_board, potential_move, PieceColor::opposite(chess_board.current_color), chess_board.all_pieces() & !bb_square) != 0 {
            legal_moves &= !bb_potential_move;
        }
    }

    // Not allowed to castle if king is being checked
    if get_pieces_attacking_square(&chess_board, square, PieceColor::opposite(chess_board.current_color), chess_board.all_pieces()) != 0 {
        return legal_moves;
    }

    // Calculate castling
    let rank = rank_index(square);
    let file = file_index(square);
    let mut castling_availability = CastlingAvailability::None;
    for (castling_side, offset_factor) in [ (CastlingAvailability::KingSide, -1), (CastlingAvailability::QueenSide, 1) ] {
        castling_availability |= if chess_board.castling_availability[chess_board.current_color as usize].contains(castling_side) {
            let mut allow_castle = true;
            // Check if other pieces are in between king and rook
            allow_castle &= (BBMASKS.pieces.castling_in_between[chess_board.current_color as usize][castling_side.bits()][square] & chess_board.all_pieces()) == 0;
            for offset in [1 * offset_factor, 2 * offset_factor] {
                let index = square_index(rank, (file as i32 + offset) as usize);
                allow_castle &= get_pieces_attacking_square(&chess_board, index, PieceColor::opposite(chess_board.current_color), chess_board.all_pieces()) == 0;
            }
            if allow_castle { castling_side } else { CastlingAvailability::None }
        } else {
            CastlingAvailability::None
        }
    }
    
    legal_moves | BBMASKS.pieces.castling_moves[chess_board.current_color as usize][castling_availability.bits()][square]
}

// https://www.chessprogramming.org/Blockers_and_Beyond
// Returns the pseudo legal moves for a sliding piece
// NOTE: Includes attack to same color pieces which callee has to remove if so wishes
fn pseudo_legal_moves_sliding_piece(square: usize, mut potential_moves: BitBoard, occupied_squares: BitBoard) -> BitBoard {
    let mut remaining_to_check = potential_moves & occupied_squares;

    while remaining_to_check != 0 {
        let slot = pop_lsb(&mut remaining_to_check);
        let dir = Dir::FROM_SQUARES_PAIRS[square][slot].unwrap();
        potential_moves &= !BBMASKS.rays[slot][dir as usize];
    }
    potential_moves
}

// https://www.chessprogramming.org/Blockers_and_Beyond
// Returns the pseudo legal attacks for a sliding piece
// NOTE: Includes attack to same color pieces which callee has to remove if so wishes
fn pseudo_legal_attacks_sliding_piece(square: usize, potential_moves: BitBoard, occupied_squares: BitBoard) -> BitBoard {
    pseudo_legal_moves_sliding_piece(square, potential_moves & occupied_squares, occupied_squares)
}

fn get_pieces_attacking_square(chess_board: &ChessBoard, square: usize, by_side: PieceColor, potential_pieces: BitBoard) -> BitBoard {
    let opposite_side = PieceColor::opposite(by_side);
    // let all_pieces = chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize];
    let mut attacks: BitBoard = 0;

    // None sliding pieces
    for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::King] {
        let pieces = chess_board.pieces[piece_type as usize] & chess_board.all_pieces[by_side as usize];
        attacks |= BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square] & pieces
    }

    // Sliding pieces
    for piece_type in [PieceType::Bishop, PieceType::Rook] {
        let pieces = (chess_board.pieces[piece_type as usize] | chess_board.pieces[PieceType::Queen as usize]) & chess_board.all_pieces[by_side as usize];
        let moves_one_empty_board = BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square];
        attacks |= pseudo_legal_attacks_sliding_piece(square, moves_one_empty_board, potential_pieces) & pieces
    }

    attacks
}

fn get_squares_blocking_check(chess_board: &ChessBoard, square: usize) -> BitBoard {
    let bb_square = (1 as BitBoard) << square;

    // Should not be called with the king
    assert_eq!(bb_square & chess_board.pieces[PieceType::King as usize], 0);

    // Should only return one king
    let bb_king: BitBoard = chess_board.pieces[PieceType::King as usize] & chess_board.all_pieces[chess_board.current_color as usize];
    assert_ne!(bb_king, 0);

    let king_square = bb_king.trailing_zeros() as usize;
    // We don't include square because if it is pinned, then we should treat the whole line between king and the attacking piece as moveable
    let occupied_squares = (chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize]) & !bb_square;
    
    let attacking_squares = get_pieces_attacking_square(chess_board, king_square, PieceColor::opposite(chess_board.current_color), occupied_squares);
    let attackers_count = attacking_squares.count_ones();
    if attackers_count == 0 { return BitBoard::MAX }
    if attackers_count > 1 {return 0 }

    let sliding_pieces = chess_board.pieces[PieceType::Queen as usize] 
                            | chess_board.pieces[PieceType::Bishop as usize] 
                            | chess_board.pieces[PieceType::Rook as usize];
    
    if attacking_squares & sliding_pieces == 0 { // The attacking piece is a NON sliding piece
        attacking_squares
    } else {    // The attacking piece is a sliding piece
        assert_eq!(attacking_squares.count_ones(), 1);
        let attacking_square = attacking_squares.trailing_zeros() as usize;
        let dir = Dir::FROM_SQUARES_PAIRS[king_square][attacking_square].unwrap();   // Should never return None
        // Create segment between king and attacking piece
        // TODO: Precompute these segments
        attacking_squares | (BBMASKS.rays[king_square][dir as usize] & BBMASKS.rays[attacking_square][Dir::opposite(dir) as usize])
    } 
}

// Returns true if doing en passant with a pawn causes check on its own king
fn does_en_passant_cause_check(chess_board: &ChessBoard, square: usize) -> bool {
    assert_ne!(chess_board.en_passant_mask, 0);

    let bb_square = (1 as BitBoard) << square;
    let king = chess_board.pieces[PieceType::King as usize] & chess_board.all_pieces[chess_board.current_color as usize];
    assert_ne!(king, 0);

    let king_index = king.trailing_zeros() as usize;
    let dir = Dir::FROM_SQUARES_PAIRS[king_index][square];
    let dir = match dir {
        Some(x) if matches!(x, Dir::East | Dir::West) => x,
        _ => return false,
    };

    let attacked_pawn = BBMASKS.pieces.en_passant_attacks[PieceColor::opposite(chess_board.current_color) as usize][chess_board.en_passant_mask.trailing_zeros() as usize];
    // Remove the attacked pawn and the attacking pawn, then check if theres a horizontal check from either a rook or a queen
    // https://www.chessprogramming.org/En_passant
    let reduced_pieces = chess_board.all_pieces() & !(attacked_pawn | bb_square);
    let attacks = pseudo_legal_attacks_sliding_piece(king_index, BBMASKS.rays[king_index][dir as usize] & reduced_pieces, reduced_pieces);
    if (attacks & (chess_board.pieces[PieceType::Rook as usize] | chess_board.pieces[PieceType::Queen as usize]) 
                & chess_board.all_pieces[PieceColor::opposite(chess_board.current_color) as usize]) == 0 { 
        return false; 
    }

    true
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_get_piece_moves() {
        // TODO: Replace with loop; Maybe use create helper module for this
        let index = square_index(3, 3);
        let moves_on_empty_board = BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Bishop as usize][index];
        let moves = pseudo_legal_moves_sliding_piece(index, moves_on_empty_board, 0xFFFF00000000FFFF);
        assert_eq!(moves, 0x0041221400142200);

        let index = square_index(3, 3);
        let moves_on_empty_board = BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Queen as usize][index];
        let moves = pseudo_legal_moves_sliding_piece(index, moves_on_empty_board, 0xEAA9489994605561);
        assert_eq!(moves, 0x0001021C141C2A49);

        let index = square_index(3, 4);
        let moves_on_empty_board = BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Rook as usize][index];
        let moves = pseudo_legal_moves_sliding_piece(index, moves_on_empty_board, 0xEAA9489994605561);
        assert_eq!(moves, 0x00000010EC101000);
    }

    #[test]
    fn test_get_piece_attacks() {
        let index = square_index(3, 3);
        let moves = pseudo_legal_moves_sliding_piece(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Bishop as usize][index] & 0xFFFF00000000FFFF, 0xFFFF00000000FFFF);
        assert_eq!(moves, 0x0041000000002200);

        let index = square_index(3, 3);
        let moves = pseudo_legal_moves_sliding_piece(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Queen as usize][index] & 0xEAA9489994605561, 0xEAA9489994605561);
        assert_eq!(moves, 0x0001001814000041);

        let index = square_index(3, 4);
        let moves = pseudo_legal_moves_sliding_piece(index, BBMASKS.pieces.attacks[PieceColor::White as usize][PieceType::Rook as usize][index] & 0xEAA9489994605561, 0xEAA9489994605561);
        assert_eq!(moves, 0x0000001084001000);    
    }

    #[test]
    fn test_all_squares_which_block_check() {
        // https://lichess.org/editor/4k3/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RP5/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(4, 2));
        assert_eq!(squares, 0x0008080808080808);

        // https://lichess.org/editor/8/7p/2n2Pp1/2bqkbK1/p2P2PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bqkbK1/p2P2PR/P1p2P2/1RP5/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(4, 2));
        assert_eq!(squares, 0);

        // https://lichess.org/editor/8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p1kP2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p1kP2/1RP5/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(4, 2));
        assert_eq!(squares, 0x0000000000000808);

        // https://lichess.org/editor/8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RPk4/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RPk4/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(4, 2));
        assert_eq!(squares, 0x0000000000000008);

        // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(3, 3));
        assert_eq!(squares, 0x0008080808080808);

        // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/5P1p/2n3p1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(3, 3));
        assert_eq!(squares, 0);

        // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4k3/6Pp/2n1R1p1/K1bq4/p2Pb1PR/P1p2P2/2P5/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(3, 3));
        assert_eq!(squares, 0x0008080000000000);

        // https://lichess.org/editor/8/6Pp/2n1R1p1/K1bq4/p2PbkPR/P1p2P2/2P5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/6Pp/2n1R1p1/K1bq4/p2PbkPR/P1p2P2/2P5/3BQ3 b - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(3, 3));
        assert_eq!(squares, BitBoard::MAX);

        // https://lichess.org/editor/1q6/3k2pp/2n1R3/2b2K2/p2Pb1BR/P1p2P2/2P5/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("1q6/3k2pp/2n1R3/2b2K2/p2Pb1BR/P1p2P2/2P5/4Q3 w - - 0 1").unwrap();
        let squares = get_squares_blocking_check(&chess_board, square_index(2, 2));
        assert_eq!(squares, 0x0000000008000000);
    }

    #[test]
    fn test_get_legal_moves_bishop() {
        // https://lichess.org/editor/1q2B3/3k2pp/2n1R3/2b2K2/p2Pb2R/P1p2P2/2P5/4Q3_w_-_-_0_1?color=white 
        let chess_board = ChessBoard::new("1q2B3/3k2pp/2n1R3/2b2K2/p2Pb2R/P1p2P2/2P5/4Q3 b - - 0 1").unwrap();
        let squares = get_legal_moves_bishop(&chess_board, square_index(3, 3));
        assert_eq!(squares, 0);

        // https://lichess.org/editor/1q2B3/3k2pp/2n1R1b1/2b2K2/p2P3R/P1p2P2/2P5/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("1q2B3/3k2pp/2n1R1b1/2b2K2/p2P3R/P1p2P2/2P5/4Q3 b - - 0 1").unwrap();
        let squares = get_legal_moves_bishop(&chess_board, square_index(5, 1));
        assert_eq!(squares, 0x0800000000000000);

        let chess_board = ChessBoard::new("1q6/3k1Bpp/2n1R1b1/2b2K2/p2P3R/P1p2P2/2P5/4Q3 b - - 0 1").unwrap();
        let squares = get_legal_moves_bishop(&chess_board, square_index(5, 1));
        assert_eq!(squares, 0x0004000500000000);

        let chess_board = ChessBoard::new("1k6/3KnRpp/8/2bB4/p4P1R/PPpq1b2/2P5/4Q3 w - - 0 1").unwrap();
        let squares = get_legal_moves_bishop(&chess_board, square_index(4, 4));
        assert_eq!(squares, 0);

        // https://lichess.org/editor/6k1/1K2nRpp/8/3B4/p4P1R/PPpq1b2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6k1/1K2nRpp/8/3B4/p4P1R/PPpq1b2/2P3b1/4Q3 w - - 0 1").unwrap();
        let squares = get_legal_moves_bishop(&chess_board, square_index(4, 4));
        assert_eq!(squares, 0x0000200008040000);
    }

    #[test]
    fn test_get_legal_moves_rook() {
        // https://lichess.org/editor/3R2k1/1K4pp/3n4/3B4/p4P1R/PPpq1b2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("3R2k1/1K4pp/3n4/3B4/p4P1R/PPpq1b2/2P3b1/4Q3 w - - 0 1").unwrap();
        let squares = get_legal_moves_rook(&chess_board, square_index(7, 4));
        assert_eq!(squares, 0x0000100000000000);

        // https://lichess.org/editor/6k1/1K4pp/3n4/3B4/p4P1R/PPpqRb2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6k1/1K4pp/3n4/3B4/p4P1R/PPpqRb2/2P3b1/4Q3 w - - 0 1").unwrap();
        let squares = get_legal_moves_rook(&chess_board, square_index(2, 3));
        assert_eq!(squares, 0);

        // https://lichess.org/editor/6k1/1K4pp/3n4/3B4/p4P1R/PPpqRb2/2P3b1/4Q3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4R1k1/6pp/3nr3/3B4/p4q1R/PPpK4/2P3b1/4Q3 b - - 0 1").unwrap();
        let squares = get_legal_moves_rook(&chess_board, square_index(5, 3));
        assert_eq!(squares, 0);

        // https://lichess.org/editor/2Q1r1k1/6pp/3n4/8/p3Bq1R/PPpK4/2P3b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("2Q1r1k1/6pp/3n4/8/p3Bq1R/PPpK4/2P3b1/8 b - - 0 1").unwrap();
        let squares = get_legal_moves_rook(&chess_board, square_index(7, 3));
        assert_eq!(squares, 0x3400000000000000);
    }

    #[test]
    fn test_get_legal_moves_queen() {
        // If both rook and bishop are correct, then queen should be correct too
        // https://lichess.org/editor/4r2p/5Q1p/3k4/2n1P1q1/p4B2/PPpK1R2/6b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4r2p/5Q1p/3k4/2n1P1q1/p4B2/PPpK1R2/6b1/8 b - - 0 1").unwrap();
        let squares = get_legal_moves_rook(&chess_board, square_index(7, 3));
        assert_eq!(squares, 0x0000000800000000);
    }

    #[test]
    fn test_get_legal_moves_knight() {
        // https://lichess.org/editor/4r2p/1k2Q2p/8/3nP1q1/p4B2/PPpK1R2/6b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4r2p/1k2Q2p/8/3nP1q1/p4B2/PPpK1R2/6b1/8 b - - 0 1").unwrap();
        let squares = get_legal_moves_knight(&chess_board, square_index(4, 4));
        assert_eq!(squares, 0x0028000000000000);

        // https://lichess.org/editor/4r2p/1k1nQ2p/8/4P1q1/p4B2/PPpK1R2/6b1/8_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("4r2p/1k1nQ2p/8/4P1q1/p4B2/PPpK1R2/6b1/8 b - - 0 1").unwrap();
        let squares = get_legal_moves_knight(&chess_board, square_index(6, 4));
        assert_eq!(squares, 0);
    }

    #[test]
    fn test_get_legal_moves_pawn() {
        // https://lichess.org/editor/1k2r2p/4Q2p/5n2/6q1/pPp1K3/P2RB3/2p3b1/3B4_w_-_b3_0_1?color=white
        let chess_board = ChessBoard::new("1k2r2p/4Q2p/5n2/6q1/pPp1K3/P2RB3/2p3b1/3B4 b - b3 0 1").unwrap();
        let squares = get_legal_moves_pawn(&chess_board, square_index(3, 5));
        assert_eq!(squares, 0x0000000000700000);

        // https://lichess.org/editor/6rp/3R1n1p/p1pQ2k1/8/q3K3/P3B2b/p1P5/3B4_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6rp/3R1n1p/p1pQ2k1/8/q3K3/P3B2b/p1P5/3B4 w - - 0 1").unwrap();
        let squares = get_legal_moves_pawn(&chess_board, square_index(1, 5));
        assert_eq!(squares, 0x0000000020000000);

        // https://lichess.org/editor/6rp/3R1n1p/p1pQ2k1/8/q1b5/P3B3/p1P1K3/3B4_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("6rp/3R1n1p/p1pQ2k1/8/q1b5/P3B3/p1P1K3/3B4 w - - 0 1").unwrap();
        let squares = get_legal_moves_pawn(&chess_board, square_index(1, 5));
        assert_eq!(squares, 0);
    }

    #[test]
    fn test_get_legal_moves_king() {
        // https://lichess.org/editor/6rp/5n1p/p1pQ2k1/8/q1b5/P3B3/p1P5/3BK2R_w_K_-_0_1?color=white
        let chess_board = ChessBoard::new("6rp/5n1p/p1pQ2k1/8/q1b5/P3B3/p1P5/3BK2R w K - 0 1").unwrap();
        let squares = get_legal_moves_king(&chess_board, square_index(0, 3));
        assert_eq!(squares, 0x0000000000001400);

        // https://lichess.org/editor/1r2n2p/5k1p/p1pQ4/8/2B2q2/P2bB3/p1P5/R3K3_w_Q_-_0_1?color=white
        let chess_board = ChessBoard::new("1r2n2p/5k1p/p1pQ4/8/2B2q2/P2bB3/p1P5/R3K3 w Q - 0 1").unwrap();
        let squares = get_legal_moves_king(&chess_board, square_index(0, 3));
        assert_eq!(squares, 0x0000000000001030);

        // https://lichess.org/editor/r3k2r/3p3p/p3B3/8/5b2/PQ5q/p7/R3K2R_b_KQkq_-_0_1?color=white
        let chess_board = ChessBoard::new("r3k2r/3p3p/p3B3/8/5b2/PQ5q/p7/R3K2R b KQkq - 0 1").unwrap();
        let squares = get_legal_moves_king(&chess_board, square_index(7, 3));
        assert_eq!(squares, 0x3408000000000000);

        // https://lichess.org/editor/8/8/8/8/8/8/8/RN2K3_w_Q_-_0_1?color=white
        let chess_board = ChessBoard::new("8/8/8/8/8/8/8/RN2K3 w Q - 0 1").unwrap();
        let squares = get_legal_moves_king(&chess_board, square_index(0, 3));
        assert_eq!(squares, 0x0000000000001C14);
    }
}