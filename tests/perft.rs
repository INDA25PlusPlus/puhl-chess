// https://www.chessprogramming.org/Perft
use chess_game::chess_board::*;

mod tests {
    use super::*;
    
    fn count_moves(chess_board: &ChessBoard, depth: usize) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut count: u64 = 0;
        for rank in 0..BOARD_RANKS {
            for file in 0..BOARD_FILES {
                let rank = Rank::new(rank).expect("Rank out of bounds");
                let file = File::new(file).expect("File out of bounds");
                let moves = chess_board.square(rank, file).get_moves();
                let moves = match moves {
                    None => continue,
                    Some(m) => m,
                };
                for chess_move in moves {
                    let (chess_board, _)= chess_move.make_move();
                    match chess_board {
                        MoveResult::ChessBoard(chess_board) => { 
                            count += count_moves(&chess_board, depth - 1);
                        }
                        MoveResult::PawnPromotionResolver(pawn_promotion_resolver) => {
                            for promotion_piece in [ PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen ] {
                                let (chess_board, _)= match promotion_piece {
                                    PieceType::Knight => pawn_promotion_resolver.resolve_knight(),
                                    PieceType::Bishop => pawn_promotion_resolver.resolve_bishop(),
                                    PieceType::Rook => pawn_promotion_resolver.resolve_rook(),
                                    PieceType::Queen => pawn_promotion_resolver.resolve_queen(),
                                    _ => unreachable!()
                                };
                                count += count_moves(&chess_board, depth - 1);
                            }
                        }
                    }
                }
            }
        }
        return count;
    }

    // ======= https://www.chessprogramming.org/Perft_Results =======
    fn test_position_helper(fen: &str, max_depth: usize, results: Vec<u64>) {
        let chess_board = ChessBoard::new(Some(fen));
        for (depth , result) in std::iter::zip(1..max_depth, results ){
            let count = count_moves(&chess_board, depth);
            assert_eq!(count, result);
        }
    }

    #[test]
    fn test_inital_position() {
        test_position_helper("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 4, vec![ 20, 400, 8902, 197281 ]);
    }

    #[test]
    fn test_position_2() {
        test_position_helper("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", 3, vec![ 48, 2039, 97862 ]);
    }

    #[test]
    fn test_position_3() {
        test_position_helper("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 4, vec![ 14, 191, 2812, 43238 ]);
    }

    #[test]
    fn test_position_4() {
        test_position_helper("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", 3, vec![ 6, 264, 9467 ]);
    }

    #[test]
    fn test_position_5() {
        test_position_helper("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", 3, vec![ 44, 1486, 62379 ]);
    }

    #[test]
    fn test_position_6() {
        test_position_helper("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", 3, vec![ 46, 2079, 89890 ]);
    }

    #[test]
    fn test_pawn_promotion() {
        let mut chess_board = ChessBoard::new(Some("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1"));
        let count = count_moves(&mut chess_board, 4);
        assert_eq!(count, 422333);
    }
}