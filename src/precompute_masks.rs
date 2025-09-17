use crate::board::*;
use crate::dir::*;
use crate::piece::*;
use crate::chess_board::*;

pub const BBMASKS: BBMasks = BBMasks::new(); 

// Contains some precomputed bit board patterns
pub struct BBMasks {
    pub rays: rays_masks::Rays,
    pub lines: lines_masks::Lines,
    pub pieces: pieces_masks::Pieces,
}

impl BBMasks {
    const fn new() -> Self {
        let rays = rays_masks::generate();
        let lines = lines_masks::generate(&rays);
        let pieces = pieces_masks::generate(&lines);
        BBMasks { rays, lines, pieces }
    }
}

const FILE_H: BitBoard = 0x0101010101010100;
const DIAG_A8_H1: BitBoard = 0x0102040810204000;
const RANK_1: BitBoard = 0x000000000000007F;
const DIAG_H8_A1: BitBoard = 0x0040201008040201;
const FILE_A: BitBoard = 0x0080808080808080;
const DIAG_A1_H8: BitBoard = 0x0002040810204080;
const RANK_8: BitBoard = 0xFE00000000000000;
const DIAG_H1_A8: BitBoard = 0x8040201008040200;

mod rays_masks {
    use super::*;

    pub type Rays = BySquare<[BitBoard; DIR_COUNT]>;

    // Generates one ray for every cardinal direction (Dir) for every square on the board
    pub const fn generate() -> Rays {
        const fn generate_north(i: usize) -> BitBoard {
            FILE_H << i
        }

        const fn generate_north_east(rank: usize, file: usize) -> BitBoard {
            let right_shift: usize = BOARD_FILES - file - 1;
            (((DIAG_A8_H1 << (right_shift * BOARD_FILES)) >> (right_shift * BOARD_FILES))
                << (rank * BOARD_FILES)) >> right_shift
        }

        const fn generate_east(rank: usize, file: usize) -> BitBoard {
            (RANK_1 >> (BOARD_FILES - file - 1)) << (rank * BOARD_RANKS)
        }

        const fn generate_south_east(rank: usize, file: usize) -> BitBoard {
            let right_shift: usize = BOARD_FILES - file - 1;
            (((DIAG_H8_A1 >> (right_shift * BOARD_FILES)) << (right_shift * BOARD_FILES))
                >> ((BOARD_RANKS - rank - 1) * BOARD_FILES)) >> right_shift
        }

        const fn generate_south(i: usize) -> BitBoard {
            FILE_A >> (BOARD_SIZE - i - 1)
        }

        const fn generate_south_west(rank: usize, file: usize) -> BitBoard {
            (((DIAG_A1_H8 >> (file * BOARD_FILES)) << (file * BOARD_FILES))
                >> ((BOARD_RANKS - rank - 1) * BOARD_FILES)) << file
        }

        const fn generate_west(rank: usize, file: usize) -> BitBoard {
            (RANK_8 << file) >> ((BOARD_FILES - rank - 1) * BOARD_FILES)
        }

        const fn generate_north_west(rank: usize, file: usize) -> BitBoard {
            (((DIAG_H1_A8 << (file * BOARD_FILES)) >> (file * BOARD_FILES))
                << (rank * BOARD_FILES)) << file
        }

        let mut result: Rays = [[0; DIR_COUNT]; BOARD_SIZE];

        let mut i: usize = 0;
        while i < BOARD_SIZE {
            let rank = rank_index(i);
            let file = file_index(i);
            result[i][Dir::North as usize]      = generate_north(i);
            result[i][Dir::NorthEast as usize]  = generate_north_east(rank, file);
            result[i][Dir::East as usize]       = generate_east(rank, file);
            result[i][Dir::SouthEast as usize]  = generate_south_east(rank, file);
            result[i][Dir::South as usize]      = generate_south(i);
            result[i][Dir::SouthWest as usize]  = generate_south_west(rank, file);
            result[i][Dir::West as usize]       = generate_west(rank, file);
            result[i][Dir::NorthWest as usize]  = generate_north_west(rank, file);
            i += 1;
        }

        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const RAYS: Rays = generate();

        #[test]
        fn test_rays() {
            assert_eq!(RAYS[square_index(4, 3)][Dir::North as usize], 0x0808080000000000);
            assert_eq!(RAYS[square_index(4, 3)][Dir::NorthEast as usize], 0x0102040000000000);
            assert_eq!(RAYS[square_index(4, 3)][Dir::East as usize], 0x0000000700000000);
            assert_eq!(RAYS[square_index(4, 3)][Dir::SouthEast as usize], 0x0000000004020100);
            assert_eq!(RAYS[square_index(4, 3)][Dir::South as usize], 0x0000000008080808);
            assert_eq!(RAYS[square_index(4, 3)][Dir::SouthWest as usize], 0x0000000010204080);
            assert_eq!(RAYS[square_index(4, 3)][Dir::West as usize], 0x000000F000000000);
            assert_eq!(RAYS[square_index(4, 3)][Dir::NorthWest as usize], 0x4020100000000000);
        }

        #[test]
        fn test_rays_edges() {
            assert_eq!(RAYS[square_index(0, 0)][Dir::South as usize], 0);
            assert_eq!(RAYS[square_index(7, 7)][Dir::NorthWest as usize], 0);
            assert_eq!(RAYS[square_index(5, 0)][Dir::East as usize], 0);
            assert_eq!(RAYS[square_index(0, 7)][Dir::SouthWest as usize], 0);
            assert_eq!(RAYS[square_index(7, 5)][Dir::South as usize], 0x0020202020202020);
            assert_eq!(RAYS[square_index(0, 0)][Dir::NorthWest as usize], 0x8040201008040200);
            assert_eq!(RAYS[square_index(5, 7)][Dir::NorthEast as usize], 0x2040000000000000);
            assert_eq!(RAYS[square_index(7, 6)][Dir::SouthEast as usize], 0x0020100804020100);
        }
    }
}

mod lines_masks {
    use super::*;

    pub struct Lines {
        pub ranks: [BitBoard; BOARD_SIZE],
        pub files: [BitBoard; BOARD_SIZE],
        pub diagonals: [BitBoard; BOARD_SIZE],
        pub anti_diagonals: [BitBoard; BOARD_SIZE],
    }

    // Generates horizontal, vertical and diagonal lines for every square on the board
    pub const fn generate(rays: &rays_masks::Rays) -> Lines {
        let mut ranks = [0; BOARD_SIZE];
        let mut files = [0; BOARD_SIZE];
        let mut diagonals = [0; BOARD_SIZE];
        let mut anti_diagonals = [0; BOARD_SIZE];

        let mut i = 0;
        while i < BOARD_SIZE {
            ranks[i] = rays[i][Dir::West as usize]  | rays[i][Dir::East as usize];
            files[i] = rays[i][Dir::North as usize] | rays[i][Dir::South as usize];
            diagonals[i]      = rays[i][Dir::NorthEast as usize]  | rays[i][Dir::SouthWest as usize];
            anti_diagonals[i] = rays[i][Dir::NorthWest as usize]  | rays[i][Dir::SouthEast as usize];
            i += 1;
        }
        
        Lines {
            ranks,
            files,
            diagonals,
            anti_diagonals
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const LINES: Lines = generate(&rays_masks::generate());

        #[test]
        fn test_lines() {
            assert_eq!(LINES.ranks[square_index(3, 4)], 0x00000000EF000000);
            assert_eq!(LINES.files[square_index(3, 4)], 0x1010101000101010);
            assert_eq!(LINES.diagonals[square_index(3, 4)], 0x0102040800204080);
            assert_eq!(LINES.anti_diagonals[square_index(3, 4)], 0x0080402000080402);
        }

        #[test]
        fn test_lines_edges() {
            assert_eq!(LINES.ranks[square_index(0, 0)], 0x00000000000000FE);
            assert_eq!(LINES.files[square_index(7, 7)], 0x0080808080808080);
            assert_eq!(LINES.diagonals[square_index(6, 7)], 0x4000000000000000);
            assert_eq!(LINES.anti_diagonals[square_index(0, 4)], 0x0000000080402000);
        }
    }
}

mod pieces_masks {
    use super::*;

    pub struct Pieces {
        // Contains the squares which a piece could go to to capture another piece
        // The squares which a piece can attack and the square a piece can move to is the same for all pieces except pawns;
        //      this is why there is a separate attribute for pawn moves
        pub attacks: ByColor<ByPiece<BySquare>>,
        pub pawn_moves: ByColor<BySquare>,
        pub pawn_double_moves: ByColor<BySquare>,
        pub en_passant_attacks: ByColor<BySquare>,
        pub castling_moves: ByColor<[BySquare; CASTLING_AVAILABILITY_SIZE]>,
        // The squares between the rook and the king
        pub castling_in_between: ByColor<[BySquare; CASTLING_AVAILABILITY_SIZE]>,
        pub castling_corners: ByColor<BySquare<CastlingAvailability>>,
    }

    // Generates the attack pattern for every piece on every square on an empty board
    pub const fn generate(lines: &lines_masks::Lines) -> Pieces {
        let (white_pawn, black_pawn) = generate_attacks_pawn();
        let knight = generate_attacks_knight();
        let mut bishop = [0; BOARD_SIZE];
        let mut rook = [0; BOARD_SIZE];
        let mut queen = [0; BOARD_SIZE];
        let king = generate_attacks_king();

        let en_passant_attacks = generate_en_passant();
        let pawn_moves  = generate_pawn_moves();
        let pawn_double_moves = generate_pawn_double_moves();
        let castling_moves = generate_castling_moves();
        let castling_in_between= generate_castling_in_between();
        let castling_corners= generate_castling_corners();
        
        let mut i = 0;
        while i < BOARD_SIZE {
            rook[i] = lines.ranks[i] | lines.files[i];
            bishop[i] = lines.diagonals[i] | lines.anti_diagonals[i];
            queen[i] = rook[i] | bishop[i];
            i += 1;
        }

        Pieces {
            attacks: [
                [ white_pawn, knight, bishop, rook, queen, king, ],
                [ black_pawn, knight, bishop, rook, queen, king, ],
            ],
            pawn_moves: pawn_moves, 
            pawn_double_moves: pawn_double_moves,
            en_passant_attacks: en_passant_attacks,
            castling_moves: castling_moves,
            castling_in_between: castling_in_between,
            castling_corners: castling_corners,
        }
    }

    const fn generate_en_passant() -> ByColor<BySquare> {
        let mut white: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut black: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];

        let white_rank = 2;
        let black_rank = 5;
        
        let mut file = 0;
        while file < BOARD_FILES {
            white[square_index(white_rank, file) as usize] = get_single_bit_board((white_rank + 1) as isize, file as isize);
            black[square_index(black_rank, file) as usize] = get_single_bit_board((black_rank - 1) as isize, file as isize);
            file += 1;
        }

        [ white, black ]
    }

    const fn generate_pawn_moves() -> ByColor<BySquare> {
        let mut white: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut black: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];

        let mut index: usize = 0;
        while index < BOARD_SIZE {
            let rank = rank_index(index) as isize;
            let file = file_index(index) as isize;
            white[index] = get_single_bit_board(rank + 1, file);
            black[index] = get_single_bit_board(rank - 1, file);
            index += 1;
        }

        return [ white, black ]
    }

    const fn generate_pawn_double_moves() -> ByColor<BySquare> {
        let mut white: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut black: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];

        let white_rank = 1;
        let black_rank = 6;
        
        let mut file = 0;
        while file < BOARD_FILES {
            white[square_index(white_rank, file) as usize] = get_single_bit_board((white_rank + 2) as isize, file as isize);
            black[square_index(black_rank, file) as usize] = get_single_bit_board((black_rank - 2) as isize, file as isize);
            file += 1;
        }

        [ white, black ]
    }

    const fn generate_castling_moves() -> ByColor<[BySquare; CASTLING_AVAILABILITY_SIZE]> {
        let mut white = [[0 as BitBoard; BOARD_SIZE]; CASTLING_AVAILABILITY_SIZE];
        let mut black = [[0 as BitBoard; BOARD_SIZE]; CASTLING_AVAILABILITY_SIZE];

        const WHITE_IDX: usize = square_index(0, 3);
        white[CastlingAvailability::KingSide.bits()][WHITE_IDX] = get_single_bit_board(0, 1);
        white[CastlingAvailability::QueenSide.bits()][WHITE_IDX] = get_single_bit_board(0, 5);
        white[CastlingAvailability::KingSide.bits() | CastlingAvailability::QueenSide.bits()][WHITE_IDX] = get_single_bit_board(0, 1) | get_single_bit_board(0, 5);

        const BLACK_IDX: usize = square_index(7, 3);
        black[CastlingAvailability::KingSide.bits()][BLACK_IDX] = get_single_bit_board(7, 1);
        black[CastlingAvailability::QueenSide.bits()][BLACK_IDX] = get_single_bit_board(7, 5);
        black[CastlingAvailability::KingSide.bits() | CastlingAvailability::QueenSide.bits()][BLACK_IDX] = get_single_bit_board(7, 1) | get_single_bit_board(7, 5);

        [ white, black ]
    }

    const fn generate_castling_in_between() -> ByColor<[BySquare; CASTLING_AVAILABILITY_SIZE]> {
        let mut white = [[0 as BitBoard; BOARD_SIZE]; CASTLING_AVAILABILITY_SIZE];
        let mut black = [[0 as BitBoard; BOARD_SIZE]; CASTLING_AVAILABILITY_SIZE];

        const WHITE_IDX: usize = square_index(0, 3);
        white[CastlingAvailability::KingSide.bits()][WHITE_IDX] = 0x0000000000000006;
        white[CastlingAvailability::QueenSide.bits()][WHITE_IDX] = 0x0000000000000070;
        white[CastlingAvailability::KingSide.bits() | CastlingAvailability::QueenSide.bits()][WHITE_IDX] = 0x0000000000000076;

        const BLACK_IDX: usize = square_index(7, 3);
        black[CastlingAvailability::KingSide.bits()][BLACK_IDX] = 0x0600000000000000;
        black[CastlingAvailability::QueenSide.bits()][BLACK_IDX] = 0x7000000000000000;
        black[CastlingAvailability::KingSide.bits() | CastlingAvailability::QueenSide.bits()][BLACK_IDX] = 0x7600000000000000;

        [ white, black ]
    }

    const fn generate_castling_corners() -> ByColor<BySquare<CastlingAvailability>> {
        let mut castling_corners = [[CastlingAvailability::None; BOARD_SIZE]; PIECE_COLOR_COUNT];

        castling_corners[PieceColor::White as usize][square_index(0, 0)] = CastlingAvailability::KingSide;
        castling_corners[PieceColor::White as usize][square_index(0, 7)] = CastlingAvailability::QueenSide;
        castling_corners[PieceColor::Black as usize][square_index(7, 0)] = CastlingAvailability::KingSide;
        castling_corners[PieceColor::Black as usize][square_index(7, 7)] = CastlingAvailability::QueenSide;

        castling_corners
    }

    const fn generate_attacks_knight() -> [BitBoard; BOARD_SIZE] {
        let mut result: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];

        let mut row: isize = 0;
        while row < BOARD_RANKS as isize {
            let mut col: isize = 0;
            while col < BOARD_FILES as isize {
                result[square_index(row as usize, col as usize)] =
                    get_single_bit_board(row + 1, col + 2) | 
                    get_single_bit_board(row + 1, col - 2) | 
                    get_single_bit_board(row - 1, col + 2) |
                    get_single_bit_board(row - 1, col - 2) |
                    get_single_bit_board(row + 2, col + 1) |
                    get_single_bit_board(row + 2, col - 1) |
                    get_single_bit_board(row - 2, col + 1) |
                    get_single_bit_board(row - 2, col - 1);
                col += 1;
            }
            row += 1;
        }

        result
    }

    const fn generate_attacks_king() -> [BitBoard; BOARD_SIZE] {
        let mut result: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];

        let mut row: isize = 0;
        while row < BOARD_RANKS as isize {
            let mut col: isize = 0;
            while col < BOARD_FILES as isize {
                result[square_index(row as usize, col as usize)] =
                    get_single_bit_board(row, col + 1)     | 
                    get_single_bit_board(row, col - 1)     | 
                    get_single_bit_board(row - 1, col + 1) |
                    get_single_bit_board(row - 1, col)     |
                    get_single_bit_board(row - 1, col - 1) |
                    get_single_bit_board(row + 1, col + 1) |
                    get_single_bit_board(row + 1, col)     |
                    get_single_bit_board(row + 1, col - 1);
                col += 1;
            }
            row += 1;
        }

        result
    }

    const fn generate_attacks_pawn() -> ([BitBoard; BOARD_SIZE], [BitBoard; BOARD_SIZE]) {
        let mut white: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut black: [BitBoard; BOARD_SIZE] = [0; BOARD_SIZE];

        let mut index: usize = 0;
        while index < BOARD_SIZE {
            let rank = rank_index(index) as isize;
            let file = file_index(index) as isize;
            white[index] = get_single_bit_board(rank + 1, file + 1) | get_single_bit_board(rank + 1, file - 1);
            black[index] = get_single_bit_board(rank - 1, file + 1) | get_single_bit_board(rank - 1, file - 1);
            index += 1;
        }

        return (white, black)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const RAYS: rays_masks::Rays = rays_masks::generate();
        const LINES: lines_masks::Lines = lines_masks::generate(&RAYS);
        const PIECES: Pieces = generate(&LINES);

        #[test]
        fn test_pieces_attacks_generation() {
            // TODO: Replace with loop; Maybe use create helper module for this
            const ATTACKS: &[[[u64; BOARD_SIZE]; PIECE_TYPE_COUNT]; PIECE_COLOR_COUNT] = &PIECES.attacks;
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Knight as usize][square_index(3, 4)], 0x0000284400442800);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Knight as usize][square_index(0, 0)], 0x0000000000020400);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Pawn as usize][square_index(3, 4)], 0x0000002800000000);
            assert_eq!(ATTACKS[PieceColor::Black as usize][PieceType::Pawn as usize][square_index(3, 4)], 0x0000000000280000);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Pawn as usize][square_index(7, 3)], 0);
            assert_eq!(ATTACKS[PieceColor::Black as usize][PieceType::Pawn as usize][square_index(7, 3)], 0x0014000000000000);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Pawn as usize][square_index(0, 4)], 0x0000000000002800);
            assert_eq!(ATTACKS[PieceColor::Black as usize][PieceType::Pawn as usize][square_index(0, 4)], 0);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Rook as usize][square_index(3, 4)], 0x10101010EF101010);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Bishop as usize][square_index(3, 4)], 0x0182442800284482);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::Queen as usize][square_index(3, 4)], 0x11925438EF385492);
            assert_eq!(ATTACKS[PieceColor::White as usize][PieceType::King as usize][square_index(3, 4)], 0x0000003828380000);
        }

        #[test]
        fn test_pawn_moves_generation() {
            const MOVES: &[[BitBoard; BOARD_SIZE]; PIECE_COLOR_COUNT] = &PIECES.pawn_moves;
            assert_eq!(MOVES[PieceColor::White as usize][square_index(3, 4)], 0x0000001000000000);
            assert_eq!(MOVES[PieceColor::Black as usize][square_index(3, 4)], 0x0000000000100000);
            assert_eq!(MOVES[PieceColor::White as usize][square_index(7, 3)], 0);
            assert_eq!(MOVES[PieceColor::Black as usize][square_index(7, 3)], get_single_bit_board(6, 3));
            assert_eq!(MOVES[PieceColor::Black as usize][square_index(0, 4)], 0);
        }

        #[test]
        fn test_pawn_double_moves_generation() {
            const MOVES: &[[BitBoard; BOARD_SIZE]; PIECE_COLOR_COUNT] = &PIECES.pawn_double_moves;
            assert_eq!(MOVES[PieceColor::White as usize][square_index(3, 4)], 0);
            assert_eq!(MOVES[PieceColor::Black as usize][square_index(3, 4)], 0);
            assert_eq!(MOVES[PieceColor::White as usize][square_index(1, 3)], get_single_bit_board(3, 3));
            assert_eq!(MOVES[PieceColor::Black as usize][square_index(7, 3)], 0);
            assert_eq!(MOVES[PieceColor::Black as usize][square_index(6, 4)], get_single_bit_board(4, 4));
        }

        #[test]
        fn test_castle_moves_generation() {
            const MOVES: &[[[u64; BOARD_SIZE]; CASTLING_AVAILABILITY_SIZE]; PIECE_COLOR_COUNT] = &PIECES.castling_moves;
            assert_eq!(MOVES[PieceColor::White as usize][CastlingAvailability::KingSide.bits()][square_index(3, 4)], 0);
            assert_eq!(MOVES[PieceColor::White as usize][CastlingAvailability::KingSide.bits()][square_index(0, 3)], 0x2);
            assert_eq!(MOVES[PieceColor::White as usize][CastlingAvailability::KingSide.bits()][square_index(1, 3)], 0);
            assert_eq!(MOVES[PieceColor::White as usize][CastlingAvailability::QueenSide.bits()][square_index(0, 2)], 0);
            assert_eq!(MOVES[PieceColor::Black as usize][CastlingAvailability::KingSide.bits() | CastlingAvailability::QueenSide.bits()][square_index(7, 3)], 0x2200000000000000);
            assert_eq!(MOVES[PieceColor::Black as usize][CastlingAvailability::QueenSide.bits()][square_index(7, 3)], 0x2000000000000000);
        }
    }
}