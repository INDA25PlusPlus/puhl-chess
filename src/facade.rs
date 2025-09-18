use crate::board::*;
use crate::board::BitBoard;
use crate::chess_board;
use crate::board;
use crate::move_generation::*;

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
pub type Index = BoundedUsize<{board::BOARD_SIZE}>;

#[derive(Clone)]
pub struct PuhlChess {
    inner: chess_board::ChessBoard,
}

impl PuhlChess {
    const INITIAL_POSITION_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn new(fen: Option<&str>) -> Self {
        let inner = chess_board::ChessBoard::new(match fen {
            None => PuhlChess::INITIAL_POSITION_FEN,
            Some(fen) => fen
        });
        Self { inner }
    }

    pub fn square(&self, rank: Rank, file: File) -> Square<'_> {
        Square { chess_board: self, rank, file }
    }

}

pub struct Move<'a> {
    src: Index,
    dst: Index,
    chess_board: &'a PuhlChess,
}

impl<'a> Move<'a> {
    pub fn make_move(&self) -> PuhlChess {
        let bb_dst = (1 as BitBoard) << self.dst.get();
        let mut chess_board_clone = (*self.chess_board).clone();
        chess_board_clone.inner.make_move(self.src.get(), bb_dst);
        chess_board_clone
    }
}

pub struct Square<'a> {
    chess_board: &'a PuhlChess,
    rank: Rank,
    file: File,
}

impl<'a> Square<'a> {
    fn as_index(&self) -> Index {
        // Abort if this returns null, this should not happen
        Index::new(self.rank.get() * board::BOARD_FILES + self.file.get()).unwrap()
    } 

    fn as_bb(&self) -> BitBoard {
        let index = self.as_index().get();
        assert!(index < 64);
        (1 as BitBoard) << index
    }
}

impl<'a> Square<'a> {
    pub fn get_moves(&self) -> Option<Vec<Move<'a>>> {
        let bb_square = self.as_bb();
        if !self.chess_board.inner.does_square_contain_piece(bb_square) {
            return None;
        }

        let piece_type = self.chess_board.inner.get_piece_type(bb_square);
        let mut bb_moves: BitBoard = get_move_generator(piece_type)(&self.chess_board.inner, self.as_index().get());

        let mut moves: Vec<Move<'a>> = vec![];
        while bb_moves != 0 {
            let index: usize = pop_lsb(&mut bb_moves);
            assert!(index < 64);
            // let rank = Rank::new(rank_index(index)).unwrap();
            // let file = File::new(file_index(index)).unwrap();
            
            moves.push(Move{
                src: self.as_index(),
                dst: Index::new(index).unwrap(),
                chess_board: self.chess_board
            });
        }
        Some(moves)
    }
}