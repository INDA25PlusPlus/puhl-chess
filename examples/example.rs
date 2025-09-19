use std::{io};
use chess_game::*;

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
    let mut chess_board = ChessBoard::new(Some("k7/7R/8/8/8/8/N4B2/3K4 w - - 0 1")).unwrap();

    loop {
        let (rank, file) = read_two_numbers();
        let square = chess_board.square(Rank::new(rank as usize).unwrap(), File::new(file as usize).unwrap());
        let info = chess_board.info();
        match info.game_state {
            GameState::Draw => println!("Draw!"),
            GameState::Win(color) => println!("{:?} side won!", color),
            GameState::Playing => (),
        }
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
        let chess_move = &moves[idx];
        println!("Rank: {:?}", chess_move.src.get_rank());
        println!("File: {:?}", chess_move.src.get_file());
        let (result, move_type) = chess_move.make_move();
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
}