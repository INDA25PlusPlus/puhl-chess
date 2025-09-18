use std::{io};
use chess_game::chess_board::*;

fn read_two_numbers() -> (i32, i32) {
    // Read two numbers
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (a, b) = (nums[0], nums[1]);

    (a, b)
    // println!("First two numbers: {}, {}", a, b);
}

fn read_one_number() -> i32 {
    // Read one number
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let c: i32 = input.trim().parse().unwrap();
    c
}

fn main() {
    let mut chess_board = ChessBoard::new(Some("k7/7R/8/8/8/8/N4B2/3K4 w - - 0 1"));

    loop {
        let (rank, file) = read_two_numbers();
        let square = chess_board.square(Rank::new(rank as usize).unwrap(), File::new(file as usize).unwrap());
        let info = chess_board.info();
        println!("{:?}", info.player_turn);
    
        println!("{}", square.dark_color());
        println!("{:?}", square.piece_color());
        println!("{:?}", square.piece_type());
    
        let moves = square.get_moves();
        let moves = match moves {
            None => unreachable!(),
            Some(m) => m,
        };
    
        let idx = read_one_number() as usize;
        let (result, move_type) = moves[idx].make_move();
        println!("{:?}", move_type);
        match result {
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
            MoveResult::ChessBoard(new_chess_board) => {
                chess_board = new_chess_board
            }
        }
    }

    let square = chess_board.square(Rank::new(0).unwrap(), File::new(0).unwrap());
    let info = chess_board.info();
    println!("{:?}", info.is_current_player_in_check);
    // let moves = .get_moves(); 
    // moves
    // let chess_board = moves[0].make_move();
}