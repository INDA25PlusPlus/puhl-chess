// https://www.chessprogramming.org/Perft
use chess_game::core::chess_board::*;
use chess_game::core::move_generation::*;
use chess_game::core::board::*;
use chess_game::core::piece::*;

mod tests {
    use super::*;
    
    fn count_moves(chess_board: &ChessBoard, depth: usize) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut count: u64 = 0;
        for piece_type in [ PieceType::Pawn, PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen, PieceType::King ] {
            let mut current = chess_board.pieces[piece_type as usize] & chess_board.all_pieces[chess_board.current_color as usize];
            while current != 0 {
                let index = current.trailing_zeros() as usize;
                let mut moves = get_move_generator(piece_type)(chess_board, index);
                // count += moves.count_ones();
                while moves != 0 {
                    // Single out the last bit
                    let current_move = moves & moves.wrapping_neg();
                    let mut chess_board_clone = chess_board.clone();
                    match piece_type {
                        PieceType::Pawn => chess_board_clone.make_move(index, current_move),
                        // PieceType::Pawn => move_pawn(&mut chess_board_clone, index, current_move),
                        PieceType::Knight => chess_board_clone.make_move(index, current_move),
                        // PieceType::Knight => move_knight(&mut chess_board_clone, index, current_move),
                        PieceType::Bishop => chess_board_clone.make_move(index, current_move),
                        // PieceType::Bishop => move_bishop(&mut chess_board_clone, index, current_move),
                        PieceType::Rook => chess_board_clone.make_move(index, current_move),
                        PieceType::Queen => chess_board_clone.make_move(index, current_move),
                        // PieceType::Queen => move_queen(&mut chess_board_clone, index, current_move),
                        PieceType::King => chess_board_clone.make_move(index, current_move),
                    }
                    // chess_board_clone.current_color = PieceColor::opposite(chess_board_clone.current_color);
                    chess_board_clone.toggle_current_color();
                    if chess_board_clone.promotion_mask != 0 {
                        // Remove pawn
                        // chess_board_clone.pieces[PieceType::Pawn as usize] &= !chess_board_clone.promotion_mask;
                        // for new_piece in [ PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen ] {
                        //     // Add new piece
                        //     chess_board_clone.pieces[new_piece as usize] |= chess_board_clone.promotion_mask;

                        //     let promotion_mask = chess_board_clone.promotion_mask;
                        //     chess_board_clone.promotion_mask = 0;
                        //     count += count_moves(&chess_board_clone, depth - 1);
                        //     chess_board_clone.promotion_mask = promotion_mask;

                        //     // Remove new piece 
                        //     chess_board_clone.pieces[new_piece as usize] &= !chess_board_clone.promotion_mask;
                        // }
                        
                        // chess_board_clone.pieces[PieceType::Pawn as usize] &= !chess_board_clone.promotion_mask;
                        for new_piece in [ PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen ] {
                            // Add new piece
                            // chess_board_clone.pieces[new_piece as usize] |= chess_board_clone.promotion_mask;
                            let mut chess_board_clone = chess_board_clone.clone();
                            chess_board_clone.resolve_promotion(new_piece);

                            // let promotion_mask = chess_board_clone.promotion_mask;
                            // chess_board_clone.promotion_mask = 0;
                            count += count_moves(&chess_board_clone, depth - 1);
                            // chess_board_clone.promotion_mask = promotion_mask;

                            // Remove new piece 
                            // chess_board_clone.pieces[new_piece as usize] &= !chess_board_clone.promotion_mask;
                        }
                    } else {
                        count += count_moves(&chess_board_clone, depth - 1);
                    }
                    moves &= moves - 1;
                }  
                current &= current - 1;
            }
        }
        count
    }

    #[test]
    fn test_initial_position_depth_1() {
        let mut chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let count = count_moves(&mut chess_board, 2);
        assert_eq!(count, 400);
    }

    #[test]
    fn test_kiwipete_position_depth_1() {
        let mut chess_board = ChessBoard::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
        let count = count_moves(&mut chess_board, 1);
        assert_eq!(count, 48);
    }

    #[test]
    fn test_position_5() {
        let mut chess_board = ChessBoard::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
        let count = count_moves(&mut chess_board, 1);
        // Doesn't work because of pawn promotions
        assert_eq!(count, 44);
    }

    #[test]
    fn test_position_6() {
        let mut chess_board = ChessBoard::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10");
        let count = count_moves(&mut chess_board, 1);
        assert_eq!(count, 46);
    }

    #[test]
    fn test_multi_depth() {
        let mut chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let count = count_moves(&mut chess_board, 3);
        assert_eq!(count, 8902);

        let mut chess_board = ChessBoard::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
        let count = count_moves(&mut chess_board, 5);
        assert_eq!(count, 674624);

        let mut chess_board = ChessBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ");
        let count = count_moves(&mut chess_board, 4);
        assert_eq!(count, 197281);

        let mut chess_board = ChessBoard::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");
        let count = count_moves(&mut chess_board, 3);
        assert_eq!(count, 97862);

        let mut chess_board = ChessBoard::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        let count = count_moves(&mut chess_board, 1);
        assert_eq!(count, 6);

        let mut chess_board = ChessBoard::new("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1 ");
        let count = count_moves(&mut chess_board, 1);
        assert_eq!(count, 6);

        let mut chess_board = ChessBoard::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ");
        let count = count_moves(&mut chess_board, 3);
        assert_eq!(count, 89890);

        let mut chess_board = ChessBoard::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ");
        let count = count_moves(&mut chess_board, 3);
        assert_eq!(count, 62379);

        let mut chess_board = ChessBoard::new("rnbq3r/pp1Pbppp/2p5/8/2B3n1/8/PPP1N1kP/RNBQK2R w KQ - 1 8");
        let count = count_moves(&mut chess_board, 3);
        assert_eq!(count, 56032);

        let mut chess_board = ChessBoard::new("rnbq1k1r/pp1Pbp1p/2p5/8/2B3n1/8/PPP1N1pP/RNBQK2R w KQ - 1 8");
        let count = count_moves(&mut chess_board, 3);
        assert_eq!(count, 54734);
    }

    #[test]
    fn test_pawn_promotion() {
        let mut chess_board = ChessBoard::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        let count = count_moves(&mut chess_board, 3);
        assert_eq!(count, 9467);

        let mut chess_board = ChessBoard::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
        let count = count_moves(&mut chess_board, 4);
        assert_eq!(count, 422333);
    }
}