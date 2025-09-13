#[derive(Copy, Clone, Debug, PartialEq)]

#[repr(usize)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const PIECE_TYPE_COUNT: usize = 6;

#[repr(usize)]
pub enum PieceColor {
    White = 0,
    Black = 1,
}

pub const PIECE_COLOR_COUNT: usize = 2;