use crate::core::move_generation::get_pieces_attacking_king;
use super::board::*;
use super::piece::*;
use super::precompute_masks::*;

#[derive(Clone, Debug)]
pub struct ChessBoard {
    pub all_pieces: ByColor<BitBoard>,

    pub current_color: PieceColor,
    pub castling_availability: ByColor<CastlingAvailability>,
    pub en_passant_mask: BitBoard,     // Contains the square a pawn has just passed while moving two squares
    pub promotion_mask: BitBoard,
    pub half_moves: u32,            // Half moves since last pawn move or capture. Used for fify-move rule
    pub full_moves: u32,            // Full moves since start

    pub pieces: ByPiece<BitBoard>,
}

impl ChessBoard {
    /// Assumes a valid fen string, otherwise program will panic
    // TODO: Maybe return result instead of panic on failure, even though it will essentially only be used for testing purposes
    pub fn new(fen: &str) -> ChessBoard {
        let mut chess_board: ChessBoard = ChessBoard { 
            all_pieces: [0, 0],
            current_color: PieceColor::White, 
            castling_availability: [CastlingAvailability::None; PIECE_COLOR_COUNT],
            en_passant_mask: 0, 
            promotion_mask: 0,
            half_moves: 0, 
            full_moves: 0, 
            pieces: [0; PIECE_TYPE_COUNT],
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
                    let square = get_single_bit_board(rank_index(square_index) as isize, (file_index(square_index)) as isize);

                    let piece_type = match chr {
                            'P' | 'p' => &mut chess_board.pieces[PieceType::Pawn as usize],
                            'N' | 'n' => &mut chess_board.pieces[PieceType::Knight as usize],
                            'B' | 'b' => &mut chess_board.pieces[PieceType::Bishop as usize],
                            'R' | 'r' => &mut chess_board.pieces[PieceType::Rook as usize],
                            'Q' | 'q' => &mut chess_board.pieces[PieceType::Queen as usize],
                            'K' | 'k' => &mut chess_board.pieces[PieceType::King as usize],
                            _ => unreachable!(),
                    };
                    let all_board= match chr.is_uppercase() {
                        true => &mut chess_board.all_pieces[PieceColor::White as usize],
                        false => &mut chess_board.all_pieces[PieceColor::Black as usize], 
                    };

                    *all_board |= square;
                    *piece_type |= square;
                }
            }
        }

        fn handle_turn_encoding(turn: &str, chess_board: &mut ChessBoard) {
            if turn == "w" { chess_board.current_color = PieceColor::White } else { chess_board.current_color = PieceColor::Black }
        }

        fn handle_castling_availability_encoding(availabilities: &str, chess_board: &mut ChessBoard) {
            for availability in availabilities.chars() {
                match availability {
                    'K' => chess_board.castling_availability[PieceColor::White as usize] |= CastlingAvailability::KingSide,
                    'Q' => chess_board.castling_availability[PieceColor::White as usize] |= CastlingAvailability::QueenSide,
                    'k' => chess_board.castling_availability[PieceColor::Black as usize] |= CastlingAvailability::KingSide,
                    'q' => chess_board.castling_availability[PieceColor::Black as usize] |= CastlingAvailability::QueenSide,
                    '-' => { chess_board.castling_availability[PieceColor::White as usize] = CastlingAvailability::None;
                             chess_board.castling_availability[PieceColor::Black as usize] = CastlingAvailability::None;
                    },
                    _ => (),
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

            chess_board.en_passant_mask = get_single_bit_board(rank as isize, (BOARD_FILES - file - 1) as isize);
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

    pub const fn all_pieces(&self) -> BitBoard {
        self.all_pieces[PieceColor::White as usize] | self.all_pieces[PieceColor::Black as usize]
    }

    pub const fn need_to_resolve_promotion(&self) -> bool {
        self.promotion_mask != 0
    }

    // If square has a piece which has the same color as the current player
    // TODO: Switch name
    pub const fn has_square_movable_piece(&self, bb_square: BitBoard) -> bool {
        bb_square & self.all_pieces[self.current_color as usize] != 0
    }

    pub const fn has_square_piece(&self, bb_square: BitBoard) -> bool {
        bb_square & self.all_pieces() != 0
    }

    // Assumes bb_square is a valid piece
    pub fn get_piece_type(&self, bb_square: BitBoard) -> PieceType {
        assert!(self.has_square_piece(bb_square));
        for piece_type in [ PieceType::Pawn, PieceType::Knight, PieceType::Bishop,
                                       PieceType::Rook, PieceType::Queen, PieceType:: King ] {
            if self.pieces[piece_type as usize] & bb_square != 0 { return piece_type; }
        }
        unreachable!()
    }

    // Assumes bb_square is a valid piece
    pub fn get_piece_color(&self, bb_square: BitBoard) -> PieceColor {
        assert!(self.has_square_movable_piece(bb_square));
        if self.all_pieces[PieceColor::White as usize] & bb_square != 0 {
            PieceColor::White
        } else {
            PieceColor::Black
        }
    }

    pub fn is_current_player_in_check(&self) -> bool {
        get_pieces_attacking_king(self, PieceColor::opposite(self.current_color)) != 0
    }
}

impl ChessBoard {
    pub fn toggle_current_color(&mut self) {
        self.current_color = PieceColor::opposite(self.current_color);
    }

    pub fn make_move(&mut self, square: usize, bb_move: BitBoard) {
        assert!(bb_move != 0);
        assert!(square < BOARD_SIZE);

        let bb_square = (1 as BitBoard) << square;
        let move_square = bb_move.trailing_zeros() as usize;
        let piece_type = self.get_piece_type(bb_square);

        self.clear_destination(bb_move);
        self.move_piece(bb_square, bb_move, piece_type);
        self.update_castling_rights(square, move_square, piece_type);
        self.update_en_passant(square, bb_move, move_square, piece_type);
    }

    // Remove destination from all bit boards
    fn clear_destination(&mut self, bb_move: BitBoard) {
        for piece_type in [PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King ] {
            self.pieces[piece_type as usize] &= !bb_move;
        }

        // Only clear opposite color because you can't stack same color pieces
        self.all_pieces[PieceColor::opposite(self.current_color) as usize] &= !bb_move;
    }

    fn move_piece(&mut self, bb_square: BitBoard, bb_move: BitBoard, piece_type: PieceType) {
        // Clear source piece
        self.pieces[piece_type as usize] &= !bb_square;
        self.all_pieces[self.current_color as usize] &= !bb_square;
        // Add new destination piece
        self.pieces[piece_type as usize] |= bb_move;
        self.all_pieces[self.current_color as usize] |= bb_move;
    }

    fn update_castling_rights(&mut self, square: usize, move_square: usize, piece_type: PieceType) {
        // Removes castling availability if capture of enemy rook
        let opposite_color = PieceColor::opposite(self.current_color) as usize;
        self.castling_availability[opposite_color] &= !BBMASKS.pieces.castling_corners[opposite_color][move_square];

        // Remove castling availability if rook moves
        let removed_castling_availability = BBMASKS.pieces.castling_corners[self.current_color as usize][square];
        self.castling_availability[self.current_color as usize] &= !removed_castling_availability;

        if piece_type == PieceType::King {
            // Move rook if castling
            let mask = BBMASKS.pieces.castling_rook_moves[self.current_color as usize][self.castling_availability[self.current_color as usize].bits()][move_square];
            assert!(self.castling_availability[self.current_color as usize].bits() <= 3);
            self.pieces[PieceType::Rook as usize] ^= mask;
            self.all_pieces[self.current_color as usize] ^= mask;

            // Clear own castling availability
            self.castling_availability[self.current_color as usize] = CastlingAvailability::None;
        }
    }

    fn update_en_passant(&mut self, square: usize, bb_move: BitBoard, move_square: usize, piece_type: PieceType) {
        if piece_type == PieceType::Pawn {
            // En passant
            if bb_move & self.en_passant_mask != 0 {
                // The attacked piece could only be a pawn
                let en_passant_index = self.en_passant_mask.trailing_zeros() as usize;
                let opposite_color = PieceColor::opposite(self.current_color) as usize;
                let mask = !BBMASKS.pieces.en_passant_attacks[opposite_color][en_passant_index];
 
                self.pieces[PieceType::Pawn as usize] &= mask;
                self.all_pieces[opposite_color] &= mask;
            }
            self.en_passant_mask = 0;

            // Double move
            if bb_move & BBMASKS.pieces.pawn_double_moves[self.current_color as usize][square] != 0 {
                self.en_passant_mask = BBMASKS.pieces.pawn_moves[self.current_color as usize][square];
            }

            // Promotion
            if rank_index(move_square) == 0 || rank_index(move_square) == 7 {
                self.promotion_mask = bb_move;
            }
        } else {
            self.en_passant_mask = 0;
        }
    }
}

impl ChessBoard {
    pub fn resolve_promotion(&mut self, piece_type: PieceType) {
        assert!(self.promotion_mask != 0);
        assert!(piece_type == PieceType::Knight || piece_type == PieceType::Bishop || piece_type == PieceType::Rook || piece_type == PieceType::Queen);

        // Remove the pawn
        self.pieces[PieceType::Pawn as usize] &= !self.promotion_mask;
        // Add the new piece
        self.pieces[piece_type as usize] |= self.promotion_mask;
        // Remove promotion mask
        self.promotion_mask = 0;
    }
}

impl ChessBoard {
    pub fn is_en_passant(&self, bb_square: BitBoard, bb_move: BitBoard) -> bool {
        assert!(bb_move != 0);
        assert!(bb_square != 0);

        let piece_type = self.get_piece_type(bb_square);
        piece_type == PieceType::Pawn 
            && !self.has_square_piece(bb_move) 
                && (bb_move & self.en_passant_mask) != 0
    }

    pub fn is_castle(&self, bb_square: BitBoard, bb_move: BitBoard) -> bool {
        assert!(bb_move != 0);
        assert!(bb_square != 0);
        
        let square = bb_square.trailing_zeros() as usize;
        let piece_type = self.get_piece_type(bb_square);
        let mask = BBMASKS.pieces.castling_moves[self.current_color as usize][self.castling_availability[self.current_color as usize].bits()][square];
        piece_type == PieceType::King 
            && (mask & bb_move) != 0
    }

    pub fn is_capture(&self, bb_square: BitBoard, bb_move: BitBoard) -> bool {
        assert!(bb_move != 0);
        assert!(bb_square != 0);

        self.has_square_piece(bb_move)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_decoding() {
        // Chess start position
        let chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(chess_board.pieces[PieceType::Pawn as usize], 0x00ff00000000ff00);
        assert_eq!(chess_board.pieces[PieceType::Knight as usize], 0x4200000000000042);
        assert_eq!(chess_board.pieces[PieceType::Bishop as usize], 0x2400000000000024);
        assert_eq!(chess_board.pieces[PieceType::Rook as usize], 0x8100000000000081);
        assert_eq!(chess_board.pieces[PieceType::Queen as usize], 0x1000000000000010);
        assert_eq!(chess_board.pieces[PieceType::King as usize], 0x0800000000000008);
        assert_eq!(chess_board.all_pieces[PieceColor::White as usize], 0x000000000000ffff);
        assert_eq!(chess_board.all_pieces[PieceColor::Black as usize], 0xffff000000000000);
        assert_eq!(chess_board.current_color, PieceColor::White);
        assert_eq!(chess_board.en_passant_mask, 0);
        assert_eq!(chess_board.half_moves, 0);
        assert_eq!(chess_board.full_moves, 1);

        let chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");
        assert_eq!(chess_board.pieces[PieceType::Pawn as usize], 0x00ff00000800F700);
        assert_eq!(chess_board.pieces[PieceType::Knight as usize], 0x4200000000000042);
        assert_eq!(chess_board.pieces[PieceType::Bishop as usize], 0x2400000000000024);
        assert_eq!(chess_board.pieces[PieceType::Rook as usize], 0x8100000000000081);
        assert_eq!(chess_board.pieces[PieceType::Queen as usize], 0x1000000000000010);
        assert_eq!(chess_board.pieces[PieceType::King as usize], 0x0800000000000008);
        assert_eq!(chess_board.all_pieces[PieceColor::White as usize], 0x000000000800F7FF);
        assert_eq!(chess_board.all_pieces[PieceColor::Black as usize], 0xffff000000000000);
        assert_eq!(chess_board.current_color, PieceColor::Black);
        assert_eq!(chess_board.en_passant_mask, 0x0000000000080000);
        assert_eq!(chess_board.half_moves, 0);
        assert_eq!(chess_board.full_moves, 1);

        // https://lichess.org/editor/8/3k3p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RP5/3BQ3_w_-_-_0_1?color=white
        let chess_board = ChessBoard::new("8/3k3p/2n2Pp1/2bq1bK1/p2P2PR/P1p2P2/1RP5/3BQ3 w - - 0 1");
        assert_eq!(chess_board.pieces[PieceType::Pawn as usize], 0x1060092A42000);
        assert_eq!(chess_board.pieces[PieceType::Knight as usize], 0x0000200000000000);
        assert_eq!(chess_board.pieces[PieceType::Bishop as usize], 0x0000002400000010);
        assert_eq!(chess_board.pieces[PieceType::Rook as usize], 0x0000000001004000);
        assert_eq!(chess_board.pieces[PieceType::Queen as usize], 0x0000001000000008);
        assert_eq!(chess_board.pieces[PieceType::King as usize], 0x0010000200000000);
        assert_eq!(chess_board.all_pieces[PieceColor::White as usize], 0x0000040213846018);
        assert_eq!(chess_board.all_pieces[PieceColor::Black as usize], 0x0011223480200000);
        assert_eq!(chess_board.current_color, PieceColor::White);
        assert_eq!(chess_board.en_passant_mask, 0);
        assert_eq!(chess_board.half_moves, 0);
        assert_eq!(chess_board.full_moves, 1);
    }
}
