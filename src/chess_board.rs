use crate::types::*;
use crate::square::*;
use crate::core::chess_board as internal;

pub use crate::core::board::{ BOARD_SIZE, BOARD_FILES, BOARD_RANKS };
pub use crate::core::piece::{ PieceType, PieceColor };

/// Represenets the state of the game
/// Win contains the color the side that won, its implied the other side has lost
#[derive(Debug)]
pub enum GameState {
    Win(PieceColor),
    Draw,
    Playing,
}

/// Contains information about the state of the chess board
#[derive(Debug)]
pub struct ChessBoardInfo {
    pub player_turn: PieceColor,
    pub is_current_player_in_check: bool,
    pub game_state: GameState
}

/// Represents the state of the chess board
#[derive(Clone, Debug)]
pub struct ChessBoard {
    pub inner: internal::ChessBoard,
}

impl ChessBoard {
    const INITIAL_POSITION_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    /// Returns a new ChessBoard with the pieces positioned as described in the 'fen' string
    /// If the fen string is None, it will use the initial chess position
    /// Returns None if the given Fen string was invalid
    pub fn new(fen: Option<&str>) -> Option<Self> {
        let inner = internal::ChessBoard::new(match fen {
            None => ChessBoard::INITIAL_POSITION_FEN,
            Some(fen) => fen
        });

        match inner {
            None => None,
            Some(inner) => Some(Self{ inner }),
        }
    }

    /// Returns the square positioned at "rank" and "file" on the board
    /// IMPORTANT: File starts from the RIGHT side of the board, so file = 0 <=> file = h
    pub fn square(&self, rank: Rank, file: File) -> Square<'_> {
        Square { chess_board: self, rank, file, }
    }

    /// Returns some state info of the chess board
    pub fn info(&self) -> ChessBoardInfo {
        let mut count: usize = 0;
        // Check how many moves player has, if zero then its either stalemate or checkamte
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



