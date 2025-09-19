# Basic Chess Library

A small chess library written in Rust.

## How to import
Add this to your Rust project’s `Cargo.toml`:
```toml
[dependencies]
puhl_chess = { git = "https://github.com/INDA25PlusPlus/puhl-chess.git", branch = "main" }
```
## Features
- ✅ Legal move generation  
- ✅ En passant  
- ✅ Castling
- ✅ Promotion
- ✅ Checkmate
- ✅ Stalemate
- ❌ Fifty-move rule  
- ❌ Threefold repetition  
- ❌ Draw by insufficient material

## How to use
The library exposes a clean high-level API around the `ChessBoard` struct.  
Here are the most important types and functions:

### `ChessBoard`
- **`ChessBoard::new(fen: Option<&str>) -> ChessBoard`**  
  Create a new chess board, either from a FEN string or from the standard initial position.
- **`ChessBoard::square(&self, rank: Rank, file: File) -> Square`**  
  Access a square on the board.
- **`ChessBoard::info(&self) -> ChessBoardInfo`**  
  Returns information such as whose turn it is, whether the current player is in check, and if the game is over.

### `Square`
- **`Square::piece_type(&self) -> Option<PieceType>`**  
  Returns which piece (if any) is on the square.
- **`Square::piece_color(&self) -> Option<PieceColor>`**  
  Returns which side owns the piece on the square.
- **`Square::get_moves(&self) -> Option<Vec<Move>>`**  
  Returns all legal moves for the piece on this square.

### `Move`
- **`Move::make_move(&self) -> (MoveResult, MoveType)`**  
  Executes the move, returning either a new `ChessBoard` or a `PawnPromotionResolver`.  
  Also tells you the type of move (`Normal`, `Promotion`, `Castling`, `EnPassant`).

### `PawnPromotionResolver`
- Resolves promotions when a pawn reaches the back rank.  
- Example: `resolver.resolve_queen()` applies promotion to a queen and returns the updated board.

### Types
- **`PieceType`**: `Pawn`, `Knight`, `Bishop`, `Rook`, `Queen`, `King`  
- **`PieceColor`**: `White`, `Black`  
- **`Rank`, `File`, `Index`**: Bounds-checked board coordinates  
- **`GameState`**: `Win(color)`, `Draw`, `Playing`  

---

For a practical demonstration, check out the [example](https://github.com/INDA25PlusPlus/puhl-chess/blob/main/examples/example.rs).  
It shows how to set up a game, query moves, make them, and handle promotions.

## Testing
The library is tested on all positions in [Perft Results](https://www.chessprogramming.org/Perft_Results) up to a depth of 5.  
See [tests/perft.rs](https://github.com/INDA25PlusPlus/puhl-chess/blob/main/tests/perft.rs) for details.
