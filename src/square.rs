use crate::chess_board::*;
use crate::types::*;
use crate::mv::*;

use crate::core::board::*;
use crate::core::move_generation::*;

/// Struct representing a Square on the ChessBoard
pub struct Square<'a> {
    pub chess_board: &'a ChessBoard,
    pub rank: Rank,
    pub file: File,
}

impl<'a> Square<'a> {
    /// Returns if square should be light or dark
    /// For example white king start position should always be on dark square
    pub fn dark_color(&self) -> bool {
        self.as_index().get() % 2 != 0 
    }

    /// Returns the piece type on the square
    /// Returns None if no piece on the square
    pub fn piece_type(&self) -> Option<PieceType> {
        let bb_square = self.as_index().as_bb();
        if !self.chess_board.inner.has_square_piece(bb_square) {
            return None
        }
        Some(self.chess_board.inner.get_piece_type(bb_square))
    }

    /// Returns the color of the piece on the square
    /// Returns None if no piece on the square
    pub fn piece_color(&self) -> Option<PieceColor> {
        let bb_square = self.as_index().as_bb();
        if !self.chess_board.inner.has_square_piece(bb_square) {
            return None
        }
        Some(self.chess_board.inner.get_piece_color(bb_square))
    }

    /// Returns the square position as an index
    pub fn as_index(&self) -> Index {
        // Abort if this returns null, this should not happen
        Index::new(self.rank.get() * BOARD_FILES + self.file.get()).unwrap()
    } 

    /// Returns all legal moves as a vector
    /// Returns an empty vector if there are no legal moves
    /// Returns None if the selected piece does not contain a piece which the current side can move
    pub fn get_moves(&self) -> Option<Vec<Move<'a>>> {
        let bb_square = self.as_index().as_bb();
        if !self.chess_board.inner.has_square_movable_piece(bb_square) {
            return None;
        }

        let piece_type = self.chess_board.inner.get_piece_type(bb_square);
        let mut bb_moves: BitBoard = get_move_generator(piece_type)(&self.chess_board.inner, self.as_index().get());

        let mut moves: Vec<Move<'a>> = vec![];
        while bb_moves != 0 {
            let index: usize = pop_lsb(&mut bb_moves);
            assert!(index < 64);
            
            moves.push(Move{
                src: self.as_index(),
                dst: Index::new(index).unwrap(),
                chess_board: self.chess_board
            });
        }
        Some(moves)
    }
}