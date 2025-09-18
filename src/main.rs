use chess_game::chess_board::*;

fn main() {
    let chess_board = ChessBoard::new(None);
    chess_board.square(Rank::new(0).unwrap(), File::new(0).unwrap());
    // let moves = .get_moves(); 
    // moves
    // let chess_board = moves[0].make_move();
}