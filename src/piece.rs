#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceColor {
    White = 0,
    Black = 1,
}

impl PieceColor {
    pub const fn opposite(piece_color: PieceColor) -> PieceColor {
        match piece_color {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

pub const PIECE_COLOR_COUNT: usize = 2;