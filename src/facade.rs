use crate::chess_board;
use crate::board;

// use crate::move_generation::get_legal_moves_bishop; 

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BoundedUsize<const MAX: usize>(usize);

// None inclusive bound
impl<const MAX: usize> BoundedUsize<MAX> {
    pub fn new(value: usize) -> Option<BoundedUsize<MAX>> {
        if value < MAX {
            Some(BoundedUsize(value))
        } else {
            None
        }
    }

    pub fn get(self) -> usize {
        self.0
    }
}

pub type Rank = BoundedUsize<{board::BOARD_RANKS}>;
pub type File = BoundedUsize<{board::BOARD_FILES}>;

pub struct ChessBoard {
    inner: chess_board::ChessBoard,
}

impl ChessBoard {
    const INITIAL_POSITION_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn new(fen: Option<&str>) -> Self {
        let inner = chess_board::ChessBoard::new(match fen {
            None => ChessBoard::INITIAL_POSITION_FEN,
            Some(fen) => fen
        });
        Self { inner }
    }

    pub fn square(&self, rank: Rank, file: File) -> Square<'_> {
        Square { chess_board: self, rank, file }
    }
}

pub struct Move<'a> {
    src: (Rank, File),
    dst: (Rank, File),
    chess_board: &'a ChessBoard,
}

impl<'a> Move<'a> {
    pub fn make_move(&self) -> ChessBoard {
        todo!()
    }
}

pub struct Square<'a> {
    chess_board: &'a ChessBoard,
    rank: Rank,
    file: File,
}

impl<'a> Square<'a> {
    pub fn get_moves(&self) -> Vec<Move<'a>> {
        // get_legal_moves_bishop(self.chess_board, square, piece_color);
        todo!()
    }
}