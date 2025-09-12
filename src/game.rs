use crate::board::*;
use crate::chess_board::*;
use crate::precompute_masks::*;
use crate::dir::*;
use crate::piece::*;

const INDEX64: [usize; 64] = [
    0,  1, 48,  2, 57, 49, 28,  3,
    61, 58, 50, 42, 38, 29, 17,  4,
    62, 55, 59, 36, 53, 51, 43, 22,
    45, 39, 33, 30, 24, 18, 12,  5,
    63, 47, 56, 27, 60, 41, 37, 16,
    54, 35, 52, 21, 44, 32, 23, 11,
    46, 26, 40, 15, 34, 20, 31, 10,
    25, 14, 19,  9, 13,  8,  7,  6
];

// https://www.chessprogramming.org/BitScan
fn bit_scan_forward(bb: Board) -> usize {
    let debruijn64: Board = 0x03f79d71b4cb0a89;
    assert!(bb != 0);
    INDEX64[(((bb & (bb.wrapping_neg())).wrapping_mul(debruijn64)) >> 58) as usize]
}

// https://www.chessprogramming.org/Blockers_and_Beyond
fn get_piece_moves(square: usize, mut potential_squares: Board, all_pieces: Board) -> Board {
    let mut remaining_to_check = potential_squares & all_pieces;
    while remaining_to_check != 0 {
        let slot = bit_scan_forward(remaining_to_check);
        let dir = Dir::FROM_SQUARES_PAIRS[square][slot].unwrap();
        potential_squares &= !ATTACKS_MASKS.rays[slot][dir as usize];
        remaining_to_check &= remaining_to_check - 1;
    }
    potential_squares
}

pub trait PieceMover {
    // fn positions(&self) -> Board;
    /// Returns the legal moves of this piece
    fn get_moves(&self, chess_board: ChessBoard, square: usize) -> Board;
}

pub trait BlockCheckMover: PieceMover {
    // Returns all moves for piece which prevents check
    // Returns all squares which prevents check
    //      OBS: if from multiple angles then you can't block it
    fn get_moves(&self, chess_board: ChessBoard, square_index: usize) -> Board {
        let rank = rank_index(square_index);
        let file = file_index(square_index);
        let square = single_cell_board(rank as isize, file as isize);
        // Remove piece square from current setup
        let all_other_pieces = (chess_board.all_white | chess_board.all_black) & !square;
    
        let current_side = if chess_board.white_turn { chess_board.all_white } else { chess_board.all_black };
        let opposite_side = if chess_board.white_turn { chess_board.all_black } else {chess_board.all_white };

        for pieace_type in ALL_PIECES {
            get_piece_moves(square_index, ATTACKS_MASKS.pieces[pieace_type as usize][square_index], all_other_pieces);
            match pieace_type
        }
        chess_board.pieaces;

        todo!()
    }
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