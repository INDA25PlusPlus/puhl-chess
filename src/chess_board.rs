use bitflags::bitflags;
use crate::chess_board;
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

impl Index {
    fn as_bb(&self) -> BitBoard {
        assert!(self.get() < 64);
        (1 as BitBoard) << self.get()
    }

}

pub enum GameState {
    Win(PieceColor),
    Draw,
    Playing,
}

pub struct ChessBoardInfo {
    player_turn: PieceColor,
    is_current_player_in_check: bool,
    game_state: GameState
}

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
        Square { chess_board: self, rank, file, }
    }

    pub fn info(&self) -> ChessBoardInfo {
        let mut count: usize = 0;
        for rank in 0..BOARD_RANKS {
            for file in 0..BOARD_FILES {
                let rank = Rank::new(rank).unwrap();
                let file = File::new(file).unwrap();
                let moves = self.square(rank, file).get_moves();
                let moves = match moves {
                    None => continue,
                    Some(m) => m,
                };
                
                count += moves.len();
            }
        }
        
        let is_current_player_in_check = self.inner.is_current_player_in_check();
        let game_state = if count == 0 {
            if is_current_player_in_check {
                GameState::Win(PieceColor::opposite(self.inner.current_color))
            } else {
                GameState::Draw     // Stalemate
            }
        } else {
            GameState::Playing
        };
        ChessBoardInfo {
            player_turn: self.inner.current_color,
            is_current_player_in_check: is_current_player_in_check,
            game_state:  game_state,
        }
    }
}

pub struct PawnPromotionResolver {
    pub chess_board: ChessBoard,
}

impl PawnPromotionResolver {
    pub fn resolve_knight(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Knight)
    }

    pub fn resolve_bishop(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Bishop)
    }

    pub fn resolve_rook(&self) -> (ChessBoard, MoveType){
        self.resolve(PieceType::Rook)
    }

    pub fn resolve_queen(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Queen)
    }

    fn resolve(&self, piece_type: PieceType) -> (ChessBoard, MoveType) {
        let mut chess_board_clone = self.chess_board.clone();
        chess_board_clone.inner.resolve_promotion(piece_type);
        chess_board_clone.inner.toggle_current_color();
        (chess_board_clone, MoveType::Promotion)
    }
}

pub enum MoveType {
    Normal( Option<PieceType> ),
    Promotion,
    Castling,
    EnPassant,
}

pub enum MoveResult {
    ChessBoard (ChessBoard),
    PawnPromotionResolver (PawnPromotionResolver)
}

pub struct Move<'a> {
    pub src: Index,
    pub dst: Index,
    chess_board: &'a ChessBoard,
}

impl<'a> Move<'a> {
    fn get_move_type(&self) -> MoveType {
        let bb_src = self.src.as_bb();
        let bb_dst = self.src.as_bb();

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
}

pub struct Square<'a> {
    chess_board: &'a ChessBoard,
    rank: Rank,
    file: File,
}

impl<'a> Square<'a> {
    pub fn dark_color(&self) -> bool {
        self.as_index().get() % 2 != 0 
    }

    pub fn piece_type(&self) -> Option<PieceType> {
        let bb_square = self.as_index().as_bb();
        if !self.chess_board.inner.has_square_piece(bb_square) {
            return None
        }
        Some(self.chess_board.inner.get_piece_type(bb_square))
    }

    pub fn piece_color(&self) -> Option<PieceColor> {
        let bb_square = self.as_index().as_bb();
        if !self.chess_board.inner.has_square_piece(bb_square) {
            return None
        }
        Some(self.chess_board.inner.get_piece_color(bb_square))
    }

    fn as_index(&self) -> Index {
        // Abort if this returns null, this should not happen
        Index::new(self.rank.get() * BOARD_FILES + self.file.get()).unwrap()
    } 

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