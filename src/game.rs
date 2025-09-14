use std::thread::current;

use crate::board::*;
use crate::chess_board::*;
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
fn attacks_to_square_sliding(chess_board: ChessBoard, square_index: usize, by_side: PieceColor, potential_pieces: Board) -> Board {
    let opposite_side = PieceColor::opposite(by_side);
    // let all_pieces = chess_board.all_pieces[PieceColor::White as usize] | chess_board.all_pieces[PieceColor::Black as usize];
    let mut attacks: Board = 0;

    // Non sliding pieces
    // for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::King] {
    //     let pieces = chess_board.pieces[piece_type as usize] & chess_board.all_pieces[by_side as usize];
    //     attacks |= BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square_index] & pieces
    // }

    for piece_type in [PieceType::Bishop, PieceType::Rook] {
        let pieces = (chess_board.pieces[piece_type as usize] | chess_board.pieces[PieceType::Queen as usize]) & chess_board.all_pieces[by_side as usize];
        attacks |= get_piece_attacks(square_index, BBMASKS.pieces.attacks[opposite_side as usize][piece_type as usize][square_index], potential_pieces) & pieces
    }

    attacks
}

fn all_squares_which_block_check(chess_board: ChessBoard, square_index: usize, piece_color: PieceColor) -> Board {
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
    let attacks_sliding = attacks_to_square_sliding(chess_board, king_index, PieceColor::opposite(piece_color), potential_pieces);
    let attacks_sliding_count = attacks_sliding.count_ones();

    // No attacks to king => no move will allow check
    if attacks_count + attacks_sliding_count == 0 { return Board::MAX }
    // More than one attack on king => we have to move the king, so no move is possible for this piece
    if attacks_count + attacks_sliding_count > 1 { return 0; };
    if attacks_count == 1 { return attacks; }

    // The ifs makes sure attacks only has one bitset
    let attack_index = attacks_sliding.trailing_zeros() as usize;
    let dir = Dir::FROM_SQUARES_PAIRS[king_index][attack_index].unwrap();   // Should never return None
    // Create segment between king and attacking piece
    // TODO: Maybe precompute these segments
    (BBMASKS.rays[king_index][dir as usize] & BBMASKS.rays[attack_index][Dir::opposite(dir) as usize]) | attacks_sliding
}

// fn all_squares_that_block_check(chess_board: ChessBoard, square_index: usize) -> Board {
//     let rank = rank_index(square_index);
//     let file = file_index(square_index);
//     let square = single_square_board(rank as isize, file as isize);

//     // Should not be called with the king
//     assert_eq!(square & chess_board.pieces[Piece::King as usize], 0);

//     // Remove piece square from board
//     let current_side = if chess_board.white_turn { chess_board.all_white } else { chess_board.all_black } & !square;
//     let opposite_side = if chess_board.white_turn { chess_board.all_black } else {chess_board.all_white } & !square;

//     // Contains the positions positions the piece can go to without its own king being in check
//     let mut current_mask = Board::MAX;

//     // The pieces king
//     let king_square = chess_board.pieces[Piece::King as usize] & current_side;
//     let king_index = (king_square & king_square.wrapping_neg()) as usize;

//     // Go through all the pieces types
//     for piece_type in [Piece::PawnWhite, Piece::PawnBlack, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen] {
//         // Goes through all the individual pieces on each piece types board
//         let mut piece_board: Board = chess_board.pieces[piece_type as usize] & opposite_side;
//         while piece_board != 0 {
//             // Singles out the LSB in board
//             let square = (piece_board & piece_board.wrapping_neg()) as Board;
//             let square_index = piece_board.trailing_zeros() as usize;
//             // Removes the LSB in board
//             piece_board &= piece_board - 1;

//             // Checks all the pieces the opposite piece attacks
//             let pieces_attacked = ATTACKS_MASKS.pieces[piece_type as usize][square_index] & current_side;
//             // Continues with another pieces if it doesn't attack the king
//             if king_square & pieces_attacked == 0 { continue };
//             // 'Ands' the mask with what positions blocks the check from this opposite piece
//             //      We 'and' it because if there are multiple pieces attacking the king, then we have to block each of those pieces
//             match piece_type {
//                 Piece::Bishop | Piece::Rook | Piece::Queen => {
//                     let dir = Dir::FROM_SQUARES_PAIRS[king_index][square_index].unwrap();   // Should never return None
//                     current_mask &= (ATTACKS_MASKS.rays[king_index][dir as usize] & ATTACKS_MASKS.rays[square_index][Dir::opposite(dir) as usize]) | square;    // TODO: Precompute a segment table
//                 }
//                 Piece::PawnWhite | Piece::PawnBlack | Piece::Knight => {
//                     current_mask &= square;
//                 }
//                 _ => { unreachable!() }
//             }
//         }
//     }

//     current_mask
// }

pub trait PieceMover {
    // fn positions(&self) -> Board;
    /// Returns the legal moves of this piece
    fn get_moves(&self, chess_board: ChessBoard, square: usize) -> Board;
}

pub trait BlockCheckMover: PieceMover {
    // Returns all moves for piece which prevents check
    // Returns all squares which prevents check
    //      OBS: if from multiple angles then you can't block it
    // For king specifically, do get_pieces_moves for every enemy piece and union them together to check, then and with the kings eight pseudo possible moves
}

pub trait SlidingMover: PieceMover {
}

/* struct PawnMover;
impl PawnMover<{ ATTACKS_MASKS.pieces[Piece::PawnWhite as usize][0] }> {
    fn tmp() {
        ATTACKS_MASKS
    } 
} */

pub trait KnightMover:  {

}

pub trait BishopMover {

}

pub trait RookMover {

}

pub trait QueenMover {

}

pub trait KingMover {

}

pub trait PawnMover {
    // (1) Get all moves which blocks a check
    // (2) 
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
            let squares = all_squares_which_block_check(chess_board, square_index(4, 2), PieceColor::Black);
            assert_eq!(squares, 0x0008080808080808);

            // https://lichess.org/editor/8/7p/2n2Pp1/2bqkbK1/p2P2PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
            let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bqkbK1/p2P2PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
            let squares = all_squares_which_block_check(chess_board, square_index(4, 2), PieceColor::Black);
            assert_eq!(squares, 0);

            // https://lichess.org/editor/8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p1kP2/1RP5/3BQ3_w_-_-_0_1?color=white
            let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p1kP2/1RP5/3BQ3 w - - 0 1");
            let squares = all_squares_which_block_check(chess_board, square_index(4, 2), PieceColor::Black);
            assert_eq!(squares, 0x0000000000000808);

            // https://lichess.org/editor/8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RPk4/3BQ3_w_-_-_0_1?color=white
            let chess_board = ChessBoard::new("8/7p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RPk4/3BQ3 w - - 0 1");
            let squares = all_squares_which_block_check(chess_board, square_index(4, 2), PieceColor::Black);
            assert_eq!(squares, 0x0000000000000008);

            // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
            let chess_board = ChessBoard::new("4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
            let squares = all_squares_which_block_check(chess_board, square_index(3, 3), PieceColor::Black);
            assert_eq!(squares, 0x0008080808080808);

            // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
            let chess_board = ChessBoard::new("4k3/5P1p/2n3p1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
            let squares = all_squares_which_block_check(chess_board, square_index(3, 3), PieceColor::Black);
            assert_eq!(squares, 0);

            // https://lichess.org/editor/4k3/7p/2n2Pp1/K1bq4/p2Pb1PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
            let chess_board = ChessBoard::new("4k3/6Pp/2n1R1p1/K1bq4/p2Pb1PR/P1p2P2/2P5/3BQ3 w - - 0 1");
            let squares = all_squares_which_block_check(chess_board, square_index(3, 3), PieceColor::Black);
            assert_eq!(squares, 0x0008080000000000);

            // https://lichess.org/editor/8/6Pp/2n1R1p1/K1bq4/p2PbkPR/P1p2P2/2P5/3BQ3_w_-_-_0_1?color=white
            let chess_board = ChessBoard::new("8/6Pp/2n1R1p1/K1bq4/p2PbkPR/P1p2P2/2P5/3BQ3 w - - 0 1");
            let squares = all_squares_which_block_check(chess_board, square_index(3, 3), PieceColor::Black);
            assert_eq!(squares, Board::MAX);
        }
    }