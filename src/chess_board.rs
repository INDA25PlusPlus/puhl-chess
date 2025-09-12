use crate::board::*;
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
    // TODO: Maybe return result instead of panic on failure, even though it will essentially only be used for testing purposes
    pub fn new(fen: &str) -> ChessBoard {
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
            let mut square_index: usize = BOARD_SIZE;
            for rank in placement.split("/") {
                for chr in rank.chars() {
                    if let Some(skips) = chr.to_digit(10) {
                        square_index -= skips as usize;
                        continue;
                    }
                    square_index -= 1;
                    assert!(square_index < BOARD_SIZE);
                    // TODO: also update all_white / all_black
                    let square = single_square_board(rank_index(square_index) as isize, (file_index(square_index)) as isize);

                    let piece_type = match chr {
                            'P' => &mut chess_board.pieces[Piece::PawnWhite as usize],
                            'p' => &mut chess_board.pieces[Piece::PawnBlack as usize],
                            'N' | 'n' => &mut chess_board.pieces[Piece::Knight as usize],
                            'B' | 'b' => &mut chess_board.pieces[Piece::Bishop as usize],
                            'R' | 'r' => &mut chess_board.pieces[Piece::Rook as usize],
                            'Q' | 'q' => &mut chess_board.pieces[Piece::Queen as usize],
                            'K' | 'k' => &mut chess_board.pieces[Piece::King as usize],
                            _ => unreachable!(),
                    };
                    let all_board= match chr.is_uppercase() {
                        true => &mut chess_board.all_white,
                        false => &mut chess_board.all_black, 
                    };

                    *all_board |= square;
                    *piece_type |= square;
                }
            }
        }

        fn handle_turn_encoding(turn: &str, chess_board: &mut ChessBoard) {
            if turn == "w" { chess_board.white_turn = true } else { chess_board.white_turn = false }
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
            let file = en_passant.next().unwrap() as u8; // first char, e.g. 'e'
            let rank = en_passant.next().unwrap() as u8; // second char, e.g. '3'

            let file = (file - b'a') as usize;
            let rank = (rank - b'1') as usize;

            chess_board.en_passant_mask = single_square_board(rank as isize, (BOARD_FILES - file - 1) as isize);
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
        chess_board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_decoding() {
        // Chess start position
        let chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(chess_board.pieces[Piece::PawnWhite as usize], 0x000000000000ff00);
        assert_eq!(chess_board.pieces[Piece::PawnBlack as usize], 0x00ff000000000000);
        assert_eq!(chess_board.pieces[Piece::Knight as usize], 0x4200000000000042);
        assert_eq!(chess_board.pieces[Piece::Bishop as usize], 0x2400000000000024);
        assert_eq!(chess_board.pieces[Piece::Rook as usize], 0x8100000000000081);
        assert_eq!(chess_board.pieces[Piece::Queen as usize], 0x1000000000000010);
        assert_eq!(chess_board.pieces[Piece::King as usize], 0x0800000000000008);
        assert_eq!(chess_board.all_white, 0x000000000000ffff);
        assert_eq!(chess_board.all_black, 0xffff000000000000);
        assert_eq!(chess_board.white_turn, true);
        assert_eq!(chess_board.en_passant_mask, 0);
        assert_eq!(chess_board.half_moves, 0);
        assert_eq!(chess_board.full_moves, 1);

        let chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(chess_board.pieces[Piece::PawnWhite as usize], 0x000000000800F700);
        assert_eq!(chess_board.pieces[Piece::PawnBlack as usize], 0x00ff000000000000);
        assert_eq!(chess_board.pieces[Piece::Knight as usize], 0x4200000000000042);
        assert_eq!(chess_board.pieces[Piece::Bishop as usize], 0x2400000000000024);
        assert_eq!(chess_board.pieces[Piece::Rook as usize], 0x8100000000000081);
        assert_eq!(chess_board.pieces[Piece::Queen as usize], 0x1000000000000010);
        assert_eq!(chess_board.pieces[Piece::King as usize], 0x0800000000000008);
        assert_eq!(chess_board.all_white, 0x000000000800F7FF);
        assert_eq!(chess_board.all_black, 0xffff000000000000);
        assert_eq!(chess_board.white_turn, false);
        assert_eq!(chess_board.en_passant_mask, 0x0000000000080000);
        assert_eq!(chess_board.half_moves, 0);
        assert_eq!(chess_board.full_moves, 1);
    }
}
