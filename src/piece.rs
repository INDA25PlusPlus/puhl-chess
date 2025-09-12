#[derive(Copy, Clone, Debug, PartialEq)]

// TODO: don't separate white and black pawns
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

pub const ALL_PIECES: [Piece; PIECE_COUNT] = [Piece::PawnWhite, Piece::PawnBlack, Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen, Piece::King];