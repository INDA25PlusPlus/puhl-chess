use crate::board::*;
use crate::chess_board;
use crate::piece::*;
use bitflags::bitflags;


bitflags! {
    pub struct CastlingAvailability: usize {
        const None = 0;
        const WhiteKingSide  = 1;
        const WhiteQueenSide = 2;
        const BlackKingSide  = 4;
        const BlackQueenSide = 8;
    }
}

pub struct ChessBoard {
    pub all_white: Board,
    pub all_black: Board,

    pub white_turn: bool,
    pub castling_availability: CastlingAvailability,
    pub en_passant_mask: Board,     // Contains the square a pawn has just passed while moving two squares
    pub half_moves: u32,            // Half moves since last pawn move or capture. Used for fify-move rule
    pub full_moves: u32,            // Full moves since start

    pub pieces: [Board; PIECE_COUNT],
}

impl ChessBoard {
    /// Assumes a valid fen string, otherwise program will panic
    // TODO: Maybe return result instead of panik on failure, even through it will only be used for testing purposes
    pub fn new(fen: &str) {
        let mut chess_board: ChessBoard = ChessBoard { 
            all_white: 0, 
            all_black: 0, 
            white_turn: true, 
            castling_availability: CastlingAvailability::None, 
            en_passant_mask: 0, 
            half_moves: 0, 
            full_moves: 0, 
            pieces: [0; PIECE_COUNT]
        };

        pub fn handle_placement_encoding(placement: &str, chess_board: &mut ChessBoard) {
            let mut square_index: usize = BOARD_SIZE - 1;
            for rank in placement.split("/") {
                for piece_type in rank.chars() {
                    assert!(square_index < BOARD_SIZE);
                    if let Some(skips) = piece_type.to_digit(10) {
                        square_index -= skips as usize;
                        continue;
                    }
                    let piece_board = match (piece_type) {
                            'P' => &mut chess_board.pieces[Piece::PawnWhite as usize],
                            'p' => &mut chess_board.pieces[Piece::PawnBlack as usize],
                            'N' | 'n' => &mut chess_board.pieces[Piece::Knight as usize],
                            'B' | 'b' => &mut chess_board.pieces[Piece::Bishop as usize],
                            'R' | 'r' => &mut chess_board.pieces[Piece::Rook as usize],
                            'Q' | 'q' => &mut chess_board.pieces[Piece::Queen as usize],
                            'K' | 'k' => &mut chess_board.pieces[Piece::King as usize],
                            _ => unreachable!(),
                    };
                    let square = single_square_board(rank_index(square_index) as isize, (BOARD_FILES - file_index(square_index) - 1) as isize);
                    *piece_board |= square;
                    square_index -= 1;
                }
            }
        }

        fn handle_turn_encoding(turn: &str, chess_board: &mut ChessBoard) {
            if (turn == "w") { chess_board.white_turn = true } else { chess_board.white_turn = false },
        }

        fn handle_castling_availability_encoding(availabilities: &str, chess_board: &mut ChessBoard) {
            for availability in availabilities.chars() {
                chess_board.castling_availability |= match availability {
                    'K' => CastlingAvailability::WhiteKingSide,
                    'Q' => CastlingAvailability::WhiteQueenSide,
                    'k' => CastlingAvailability::BlackKingSide,
                    'q' => CastlingAvailability::BlackQueenSide,
                    '-' => CastlingAvailability::None,
                    _ => CastlingAvailability::None,
                };
            }
        }

        fn handle_en_passant_encoding(en_passant: &str, chess_board: &mut ChessBoard) {
            if en_passant == "-" {
                return;
            }
            let mut en_passant = en_passant.chars();
            let rank = en_passant.next().unwrap() as u8; // second char, e.g. '3'
            let file = en_passant.next().unwrap() as u8; // first char, e.g. 'e'

            let rank = rank.wrapping_sub(b'1') as isize;
            let file = file.wrapping_sub(b'a') as isize;

            chess_board.en_passant_mask = single_square_board(rank, file);
        }

        for (i, field) in fen.split_whitespace().enumerate() {
            match i {
                0 => handle_placement_encoding(field, &mut chess_board),
                1 => handle_turn_encoding(field, &mut chess_board),
                2 => handle_castling_availability_encoding(field, &mut chess_board),
                3 => handle_en_passant_encoding(field, &mut chess_board),
                4 => { chess_board.half_moves = field.parse().unwrap() },
                5 => { chess_board.full_moves = field.parse().unwrap() },
                _ => unreachable!("Given FEN string is invalid: {}", fen)
            }
        }
    }
}