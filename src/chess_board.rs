use crate::types::*;
use crate::square::*;
use crate::core::chess_board as internal;

pub use crate::core::board::{ BOARD_SIZE, BOARD_FILES, BOARD_RANKS };
pub use crate::core::piece::{ PieceType, PieceColor };


#[derive(Debug)]
pub enum GameState {
    Win(PieceColor),
    Draw,
    Playing,
}

#[derive(Debug)]
pub struct ChessBoardInfo {
    pub player_turn: PieceColor,
    pub is_current_player_in_check: bool,
    pub game_state: GameState
}

#[derive(Clone, Debug)]
pub struct ChessBoard {
    pub inner: internal::ChessBoard,
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



