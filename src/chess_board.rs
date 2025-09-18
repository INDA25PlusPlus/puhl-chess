use crate::core::board::*;
use crate::core::move_generation::*;
use crate::core::chess_board as internal;

pub use crate::core::board::{ BOARD_SIZE, BOARD_FILES, BOARD_RANKS };
pub use crate::core::piece::{ PieceType, PieceColor };

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

pub type Rank = BoundedUsize<{BOARD_RANKS}>;
pub type File = BoundedUsize<{BOARD_FILES}>;
pub type Index = BoundedUsize<{BOARD_SIZE}>;

#[derive(Clone)]
pub struct ChessBoard {
    inner: internal::ChessBoard,
}

impl ChessBoard {
    const INITIAL_POSITION_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn new(fen: Option<&str>) -> Self {
        let inner = internal::ChessBoard::new(match fen {
            None => ChessBoard::INITIAL_POSITION_FEN,
            Some(fen) => fen
        });
        Self { inner }
    }

    pub fn square(&self, rank: Rank, file: File) -> Square<'_> {
        Square { chess_board: self, rank, file }
    }
}

pub struct PawnPromotionResolver {
    pub chess_board: ChessBoard,
}

impl PawnPromotionResolver {
    pub fn resolve_knight(&self) -> ChessBoard {
        let mut chess_board_clone = self.chess_board.clone();
        chess_board_clone.inner.resolve_promotion(PieceType::Knight);
        chess_board_clone.inner.toggle_current_color();
        chess_board_clone
    }

    pub fn resolve_bishop(&self) -> ChessBoard {
        let mut chess_board_clone = self.chess_board.clone();
        chess_board_clone.inner.resolve_promotion(PieceType::Bishop);
        chess_board_clone.inner.toggle_current_color();
        chess_board_clone
    }

    pub fn resolve_rook(&self) -> ChessBoard {
        let mut chess_board_clone = self.chess_board.clone();
        chess_board_clone.inner.resolve_promotion(PieceType::Rook);
        chess_board_clone.inner.toggle_current_color();
        chess_board_clone
    }

    pub fn resolve_queen(&self) -> ChessBoard {
        let mut chess_board_clone = self.chess_board.clone();
        chess_board_clone.inner.resolve_promotion(PieceType::Queen);
        chess_board_clone.inner.toggle_current_color();
        chess_board_clone
    }
}

pub enum MoveResult {
    ChessBoard (ChessBoard),
    PawnPromotionResolver (PawnPromotionResolver)
}

pub struct Move<'a> {
    src: Index,
    dst: Index,
    chess_board: &'a ChessBoard,
}

impl<'a> Move<'a> {
    pub fn make_move(&self) -> MoveResult {
        let bb_dst = (1 as BitBoard) << self.dst.get();
        let mut chess_board_clone = (*self.chess_board).clone();
        chess_board_clone.inner.make_move(self.src.get(), bb_dst);

        if chess_board_clone.inner.need_to_resolve_promotion() {
            MoveResult::PawnPromotionResolver(PawnPromotionResolver { chess_board: chess_board_clone })
        } else {
            chess_board_clone.inner.toggle_current_color();
            MoveResult::ChessBoard(chess_board_clone)
        }
    }
}

pub struct Square<'a> {
    chess_board: &'a ChessBoard,
    rank: Rank,
    file: File,
}

impl<'a> Square<'a> {
    fn as_index(&self) -> Index {
        // Abort if this returns null, this should not happen
        Index::new(self.rank.get() * BOARD_FILES + self.file.get()).unwrap()
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
        if !self.chess_board.inner.square_with_moveable_piece(bb_square) {
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