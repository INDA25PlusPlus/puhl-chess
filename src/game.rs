use std::thread::current;

use crate::board::*;
use crate::chess_board::*;
use crate::precompute_masks::*;
use crate::dir::*;
use crate::piece::*;

// https://www.chessprogramming.org/Blockers_and_Beyond
fn get_piece_moves(square: usize, mut potential_squares: Board, all_pieces: Board) -> Board {
    let mut remaining_to_check = potential_squares & all_pieces;
    while remaining_to_check != 0 {
        let slot: usize = remaining_to_check.trailing_zeros() as usize;
        let dir = Dir::FROM_SQUARES_PAIRS[square][slot].unwrap();
        potential_squares &= !ATTACKS_MASKS.rays[slot][dir as usize];
        remaining_to_check &= remaining_to_check - 1;
    }
    potential_squares
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