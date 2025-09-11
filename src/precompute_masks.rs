use crate::board::*;
use crate::dir::*;
use crate::piece::*;

// TODO: Make a more consistent naming scheme
// TODO: Make easy way to visualize hard coded hex bit boards without taking to much space
// TODO: Don't make this a global constant
pub const ATTACKS_MASKS: AttacksMasks = make_lookup();

/// Contains the attack pattern on an empty board for every possible ray, line and piece
pub struct AttacksMasks {
    pub rays: rays_masks::Rays,
    pub lines: lines_masks::Lines,
    pub pieces: pieces_masks::Pieces,
}

const fn make_lookup() -> AttacksMasks {
    let rays = rays_masks::generate();
    let lines = lines_masks::generate(&rays);
    let pieces = pieces_masks::generate(&lines);
    AttacksMasks { rays, lines, pieces }
}

mod rays_masks {
    use super::*;

    // TODO: Maybe switch inner and outer array to make it more consistent with 'lines' and 'pieces'
    pub type Rays = [[Board; DIR_COUNT]; BOARD_SIZE];

    // Generates one ray for every cardinal direction (Dir) for every square on the board
    // TODO: Maybe switch the hardcoded hex bit boards to constants (ex. 0x0101010101010100 -> H1_H8_FILE)
    pub const fn generate() -> Rays {
        const fn generate_north(i: usize) -> Board {
            const NORTH: Board = 0x0101010101010100;    // h1 -> h8 file
            NORTH << i
        }

        const fn generate_north_east(rank: usize, file: usize) -> Board {
            const NORTH_EAST: Board = 0x0102040810204000;   // a8 -> h1 diagonal 
            let right_shift: usize = BOARD_FILES - file - 1;
            (((NORTH_EAST << (right_shift * BOARD_FILES)) >> (right_shift * BOARD_FILES))
                << (rank * BOARD_FILES)) >> right_shift
        }

        const fn generate_east(rank: usize, file: usize) -> Board {
            const EAST: Board = 0x000000000000007F; // a1 -> a8 rank
            (EAST >> (BOARD_FILES - file - 1)) << (rank * BOARD_RANKS)
        }

        const fn generate_south_east(rank: usize, file: usize) -> Board {
            const SOUTH_EAST: Board = 0x0040201008040201;   // h8 -> a1 anti-diagonal
            let right_shift: usize = BOARD_FILES - file - 1;
            (((SOUTH_EAST >> (right_shift * BOARD_FILES)) << (right_shift * BOARD_FILES))
                >> ((BOARD_RANKS - rank - 1) * BOARD_FILES)) >> right_shift
        }

        const fn generate_south(i: usize) -> Board {
            const SOUTH: Board = 0x0080808080808080;    // h8 -> a8 file
            SOUTH >> (BOARD_SIZE - i - 1)
        }

        const fn generate_south_west(rank: usize, file: usize) -> Board {
            const SOUTH_WEST: Board = 0x0002040810204080;   // a1 -> h8 diagonal
            (((SOUTH_WEST >> (file * BOARD_FILES)) << (file * BOARD_FILES))
                >> ((BOARD_RANKS - rank - 1) * BOARD_FILES)) << file
        }

        const fn generate_west(rank: usize, file: usize) -> Board {
            const WEST: Board = 0xFE00000000000000; // h1 -> h8 rank
            (WEST << file) >> ((BOARD_FILES - rank - 1) * BOARD_FILES)
        }

        const fn generate_north_west(rank: usize, file: usize) -> Board {
            const NORTH_WEST: Board = 0x8040201008040200; // a1 -> h8 anti-diagonal
            (((NORTH_WEST << (file * BOARD_FILES)) >> (file * BOARD_FILES))
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
        pub ranks: [Board; BOARD_SIZE],
        pub files: [Board; BOARD_SIZE],
        pub diagonals: [Board; BOARD_SIZE],
        pub anti_diagonals: [Board; BOARD_SIZE],
    }

    // Generates horizontal, vertical and diagonal lines for each square on the board
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

    // pub struct Pieces {
    //     pub pawn: [BoardPair; BOARD_SIZE],
    //     pub knight: [Board; BOARD_SIZE],
    //     pub bishop: [Board; BOARD_SIZE],
    //     pub rook: [Board; BOARD_SIZE],
    //     pub queen: [Board; BOARD_SIZE],
    //     pub king: [Board; BOARD_SIZE],
    // }
    pub type Pieces = [[Board; BOARD_SIZE]; PIECE_COUNT];

    // Generates the attack patter for every piece on every square on an empty board
    pub const fn generate(lines: &lines_masks::Lines) -> Pieces {
        let mut rook = [0; BOARD_SIZE];
        let mut bishop = [0; BOARD_SIZE];
        let mut queen = [0; BOARD_SIZE];
        let knight = generate_attacks_knight();
        let king = generate_attacks_king();
        let (white_pawn, black_pawn) = generate_attacks_pawn();
        
        let mut i = 0;
        while i < BOARD_SIZE {
            rook[i] = lines.ranks[i] | lines.files[i];
            bishop[i] = lines.diagonals[i] | lines.anti_diagonals[i];
            queen[i] = rook[i] | bishop[i];
            i += 1;
        }

        let mut pieces: Pieces = [[0; BOARD_SIZE]; PIECE_COUNT];
        pieces[Piece::PawnWhite as usize] = white_pawn;
        pieces[Piece::PawnBlack as usize] = black_pawn;
        pieces[Piece::Knight as usize] = knight;
        pieces[Piece::Bishop as usize] = bishop;
        pieces[Piece::Rook as usize] = rook;
        pieces[Piece::Queen as usize] = queen;
        pieces[Piece::King as usize] = king;

        pieces
    }

    const fn generate_attacks_knight() -> [Board; BOARD_SIZE] {
        let mut result: [Board; BOARD_SIZE] = [0; BOARD_SIZE];

        let mut row: isize = 0;
        while row < BOARD_RANKS as isize {
            let mut col: isize = 0;
            while col < BOARD_FILES as isize {
                result[square_index(row as usize, col as usize)] =
                    single_cell_board(row + 1, col + 2) | 
                    single_cell_board(row + 1, col - 2) | 
                    single_cell_board(row - 1, col + 2) |
                    single_cell_board(row - 1, col - 2) |
                    single_cell_board(row + 2, col + 1) |
                    single_cell_board(row + 2, col - 1) |
                    single_cell_board(row - 2, col + 1) |
                    single_cell_board(row - 2, col - 1);
                col += 1;
            }
            row += 1;
        }

        result
    }

    const fn generate_attacks_king() -> [Board; BOARD_SIZE] {
        let mut result: [Board; BOARD_SIZE] = [0; BOARD_SIZE];

        let mut row: isize = 0;
        while row < BOARD_RANKS as isize {
            let mut col: isize = 0;
            while col < BOARD_FILES as isize {
                result[square_index(row as usize, col as usize)] =
                    single_cell_board(row, col + 1)     | 
                    single_cell_board(row, col - 1)     | 
                    single_cell_board(row - 1, col + 1) |
                    single_cell_board(row - 1, col)     |
                    single_cell_board(row - 1, col - 1) |
                    single_cell_board(row + 1, col + 1) |
                    single_cell_board(row + 1, col)     |
                    single_cell_board(row + 1, col - 1);
                col += 1;
            }
            row += 1;
        }

        result
    }

    const fn generate_attacks_pawn() -> ([Board; BOARD_SIZE], [Board; BOARD_SIZE]) {
        let mut white: [Board; BOARD_SIZE] = [0; BOARD_SIZE];
        let mut black: [Board; BOARD_SIZE] = [0; BOARD_SIZE];

        let mut row: isize = 0;
        while row < BOARD_RANKS as isize {
            let mut col: isize = 0;
            while col < BOARD_FILES as isize {
                let index = square_index(row as usize, col as usize);
                white[index] = single_cell_board(row + 1, col);
                black[index] = single_cell_board(row - 1, col);
                col += 1;                                                               
            }
            row += 1;
        }

        (white, black)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const RAYS: rays_masks::Rays = rays_masks::generate();
        const LINES: lines_masks::Lines = lines_masks::generate(&RAYS);
        const PIECES: Pieces = generate(&LINES);

        #[test]
        fn test_pieces_generation() {
            assert_eq!(PIECES[Piece::Knight as usize][square_index(3, 4)], 0x0000284400442800);
            assert_eq!(PIECES[Piece::Knight as usize][square_index(0, 0)], 0x0000000000020400);
            assert_eq!(PIECES[Piece::PawnWhite as usize][square_index(3, 4)], 0x0000001000000000);
            assert_eq!(PIECES[Piece::PawnBlack as usize][square_index(3, 4)], 0x0000000000100000);
            assert_eq!(PIECES[Piece::PawnWhite as usize][square_index(7, 3)], 0);
            assert_eq!(PIECES[Piece::PawnBlack as usize][square_index(7, 3)], single_cell_board(6, 3));
            assert_eq!(PIECES[Piece::PawnBlack as usize][square_index(0, 4)], 0);
            assert_eq!(PIECES[Piece::PawnWhite as usize][square_index(0, 4)], single_cell_board(1, 4));
            assert_eq!(PIECES[Piece::Rook as usize][square_index(3, 4)], 0x10101010EF101010);
            assert_eq!(PIECES[Piece::Bishop as usize][square_index(3, 4)], 0x0182442800284482);
            assert_eq!(PIECES[Piece::Queen as usize][square_index(3, 4)], 0x11925438EF385492);
            assert_eq!(PIECES[Piece::King as usize][square_index(3, 4)], 0x0000003828380000);
        }
    }
}