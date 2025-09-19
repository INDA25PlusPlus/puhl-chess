use crate::chess_board::*;
use crate::promotion::*;
use crate::types::*;

use crate::core::piece::*;
use crate::core::board::*;

/// Represents the type of a chess move
/// "Promotion" always implies a capture of pawn
/// PieceType in Normal is the captured piece type, if None then there was no capture 
#[derive(Debug)]
pub enum MoveType {
    Normal( Option<PieceType> ),
    Promotion,
    Castling,
    EnPassant,
}

/// Contains the result from a chess move
#[derive(Debug)]
pub enum MoveResult {
    ChessBoard (ChessBoard),
    PawnPromotionResolver (PawnPromotionResolver)
}

/// Represents a chess move
/// Contains the move source index the move destination index and a reference to the board the move was made on
#[derive(Debug)]
pub struct Move<'a> {

    pub src: Index,
    pub dst: Index,
    pub chess_board: &'a ChessBoard,
}

impl<'a> Move<'a> {
    /// Performs the move on a CLONE of the referenced chess board
    /// Returns a MoveResult and a MoveType
    /// MoveResult will just be the CLONED chess board with the move perfomed, except when there is a promotion
    ///     Then it will return a PawnPromotionResolver
    pub fn make_move(&self) -> (MoveResult, MoveType) {
        let bb_dst = (1 as BitBoard) << self.dst.get();
        let mut chess_board_clone = (*self.chess_board).clone();

        let move_type = self.get_move_type();
        chess_board_clone.inner.make_move(self.src.get(), bb_dst);

        if chess_board_clone.inner.need_to_resolve_promotion() {
            (MoveResult::PawnPromotionResolver(PawnPromotionResolver { chess_board: chess_board_clone }), move_type)
        } else {
            chess_board_clone.inner.toggle_current_color();
            (MoveResult::ChessBoard(chess_board_clone), move_type)
        }
    }
    
    fn get_move_type(&self) -> MoveType {
        let bb_src = self.src.as_bb();
        let bb_dst = self.dst.as_bb();

        if self.chess_board.inner.is_castle(bb_src, bb_dst) {
            MoveType::Castling
        }
        else if self.chess_board.inner.is_en_passant(bb_src, bb_dst) {
            MoveType::EnPassant
        }
        else if self.chess_board.inner.is_capture(bb_src, bb_dst) {
            let piece_type = self.chess_board.inner.get_piece_type(bb_dst);
            MoveType::Normal(Some(piece_type))
        } else {
            MoveType::Normal(None)
        }
    }
}

