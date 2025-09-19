/// Simple chess program showing how to use the chess library interface
use std::{io};
use puhl_chess::*;

fn read_two_numbers() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (a, b) = (nums[0], nums[1]);

    (a, b)
}

fn read_one_number() -> i32 {
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let c: i32 = input.trim().parse().unwrap();
    
    c
}

fn main() {
    let mut chess_board = ChessBoard::new(None).unwrap();

    loop {
        let (rank, file) = read_two_numbers();
        // Retrievees the square which the user specifed
        let square = chess_board.square(Rank::new(rank as usize).unwrap(), File::new(file as usize).unwrap());
        // Retrieves state info about the chess board
        let info = chess_board.info();
        // Checks if it a draw or win
        match info.game_state {
            GameState::Draw => println!("Draw!"),
            GameState::Win(color) => println!("{:?} side won!", color),
            GameState::Playing => (),
        }
        // Prints some stuff about the board state
        println!("{:?}", info.player_turn);
    
        // Prints some informatin about the retrieved square
        println!("{}", square.dark_color());
        println!("{:?}", square.piece_color());
        println!("{:?}", square.piece_type());
    
        // Retrieves the legal moves which the piece on the square can do    
        let moves = square.get_moves();
        let moves = match moves {
            None => unreachable!(),
            Some(m) => m,
        };
    
        let idx = read_one_number() as usize;
        // Selects the move which user specified
        let chess_move = &moves[idx];
        // Prints some information about the chess move
        println!("Source Rank: {:?}", chess_move.src.get_rank());
        println!("Source File: {:?}", chess_move.src.get_file());
        println!("Destination Rank: {:?}", chess_move.dst.get_rank());
        println!("Destination File: {:?}", chess_move.dst.get_file());
        // Makes the chess move and returns a clone of the chess board with the move performed
        let (result, move_type) = chess_move.make_move();
        println!("{:?}", move_type);
        match result {
            // Resolves the promotion based on what the user specified
            MoveResult::PawnPromotionResolver(resolver) => {
                let idx = read_one_number() as usize;
                let (new_chess_board, move_type) = match idx {
                    0 => resolver.resolve_knight(), 
                    1 => resolver.resolve_bishop(), 
                    2 => resolver.resolve_rook(), 
                    3 => resolver.resolve_queen(), 
                    _ => unreachable!()
                };
                println!("{:?}", move_type);
                chess_board = new_chess_board;
            }
            // Sets the old board equal to the new clone and do the loop again
            MoveResult::ChessBoard(new_chess_board) => {
                chess_board = new_chess_board
            }
        }
    }
}