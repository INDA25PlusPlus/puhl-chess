use crate::board::*;
use crate::piece::*;

pub struct ChessBoard {
    pub all_white: Board,
    pub all_black: Board,
    pub white_turn: bool,

    pub pieaces: [Piece; PIECE_COUNT],
}