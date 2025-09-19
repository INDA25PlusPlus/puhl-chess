use crate::chess_board::*;
use crate::mv::*;

/// Structure used for resolving a chess promotion
#[derive(Debug)]
pub struct PawnPromotionResolver {
    pub chess_board: ChessBoard,
}

// TODO: Merge the different resolves into one, by takinga bounded piece type as argument.
//          The bounded piece type should be the same concept as Rank, File, Index strucs;
//          It should only allow knight bishop rook and queen so we don't have to do error handling
impl PawnPromotionResolver {
    /// Resolves the promotion by replacing the promoted pawn with a knight
    /// Returns a CLONE of the chess_board and a MoveType; MoveType is always set to MoveType::Promotion
    pub fn resolve_knight(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Knight)
    }

    /// Resolves the promotion by replacing the promoted pawn with a bishop
    /// Returns a CLONE of the chess_board and a MoveType; MoveType is always set to MoveType::Promotion
    pub fn resolve_bishop(&self) -> (ChessBoard, MoveType) {
        self.resolve(PieceType::Bishop)
    }

    /// Resolves the promotion by replacing the promoted pawn with a rook
    /// Returns a CLONE of the chess_board and a MoveType; MoveType is always set to MoveType::Promotion
    pub fn resolve_rook(&self) -> (ChessBoard, MoveType){
        self.resolve(PieceType::Rook)
    }

    /// Resolves the promotion by replacing the promoted pawn with a queen
    /// Returns a CLONE of the chess_board and a MoveType; MoveType is always set to MoveType::Promotion
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