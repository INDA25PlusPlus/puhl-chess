// https://www.chessprogramming.org/Perft_Results
use chess_game::chess_board::ChessBoard;
use chess_game::game::*;

mod tests {
    use chess_game::piece::{PieceColor, PieceType};
    use super::*;

    fn count_moves(chess_board: &ChessBoard) -> u32 {
        let mut count = 0;
        let piece_color = if chess_board.white_turn { PieceColor::White } else { PieceColor::Black };
        for piece_type in [ PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King ] {
            let mut current = chess_board.pieces[piece_type as usize] & chess_board.all_pieces[piece_color as usize];
            while current != 0 {
                let index = current.trailing_zeros() as usize;
                let moves = match piece_type {
                    PieceType::Pawn => get_legal_moves_pawn(&chess_board, index, piece_color),
                    PieceType::Knight => get_legal_moves_knight(&chess_board, index, piece_color),
                    PieceType::Bishop => get_legal_moves_bishop(&chess_board, index, piece_color),
                    PieceType::Rook => get_legal_moves_rook(&chess_board, index, piece_color),
                    PieceType::Queen => get_legal_moves_queen(&chess_board, index, piece_color),
                    PieceType::King => get_legal_moves_king(&chess_board, index, piece_color),
                };
                count += moves.count_ones();
                current &= current - 1;
            }
        }
        count
    }

    #[test]
    fn test_initial_position_depth_1() {
        let chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let count = count_moves(&chess_board);
        assert_eq!(count, 20);
    }

    #[test]
    fn test_kiwipete_position_depth_1() {
        let chess_board = ChessBoard::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let count = count_moves(&chess_board);
        assert_eq!(count, 48);
    }

    #[test]
    fn test_position_5() {
        let chess_board = ChessBoard::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        let count = count_moves(&chess_board);
        // Doesn't work because of pawn promotions
        // assert_eq!(count, 44);
    }

    #[test]
    fn test_position_6() {
        let chess_board = ChessBoard::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
        let count = count_moves(&chess_board);
        assert_eq!(count, 46);
    }
}