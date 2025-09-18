use chess_game::chess_board::*;

fn main() {
    let chess_board = ChessBoard::new(None);
    let moves = chess_board.square(Rank::new(0).unwrap(), File::new(0).unwrap()).get_moves(); 
    // let chess_board = moves[0].make_move();
}