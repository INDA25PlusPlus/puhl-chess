use crate::chess_board::*;
use crate::mv::*;

#[derive(Debug)]
pub struct PawnPromotionResolver {
    pub chess_board: ChessBoard,
}

impl PawnPromotionResolver {
    pub fn resolve_knight(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Knight)
    }

    pub fn resolve_bishop(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Bishop)
    }

    pub fn resolve_rook(&self) -> (ChessBoard, MoveType){
        self.resolve(PieceType::Rook)
    }

    pub fn resolve_queen(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Queen)
    }

    fn resolve(&self, piece_type: PieceType) -> (ChessBoard, MoveType) {
        let mut chess_board_clone = self.chess_board.clone();
        chess_board_clone.inner.resolve_promotion(piece_type);
        chess_board_clone.inner.toggle_current_color();
        (chess_board_clone, MoveType::Promotion)
    }
}