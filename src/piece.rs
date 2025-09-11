#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    PawnWhite,
    PawnBlack,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const PIECE_COUNT: usize = 7;