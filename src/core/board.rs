use bitflags::bitflags;
pub type BitBoard = u64;

pub const BOARD_RANKS: usize = 8;
pub const BOARD_FILES: usize = 8;
pub const BOARD_SIZE: usize = BOARD_RANKS * BOARD_FILES;

pub type BySquare<T = BitBoard> = [T; BOARD_SIZE];

bitflags! {
    #[derive(PartialEq, Clone, Copy, Debug)]
    pub struct CastlingAvailability: usize {
        const None      = 0;
        const KingSide  = 1;
        const QueenSide = 2;
    }
}

pub const CASTLING_AVAILABILITY_SIZE: usize = 4;

pub const fn square_index(rank: usize, file: usize) -> usize {
    rank * BOARD_FILES + file
}

pub const fn rank_index(index: usize) -> usize {
    index / BOARD_FILES
}

pub const fn file_index(index: usize) -> usize {
    index % BOARD_FILES
}

// Generates a bit board where the square of 'rank' and 'file' is the only set square
pub const fn get_single_bit_board(rank: isize, file: isize) -> BitBoard {
    if rank < 0 || file < 0 || rank >= BOARD_RANKS as isize || file >= BOARD_FILES as isize {
        return 0;
    }

    (1 as BitBoard) << (rank as usize * BOARD_FILES + file as usize)
}

// Returns the index of the least significant bit and removes it from the BitBoard
#[inline(always)] pub const fn pop_lsb(bit_board: &mut BitBoard) -> usize { 
    let index = bit_board.trailing_zeros() as usize; 
    *bit_board &= *bit_board - 1; 
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_index() {
        assert_eq!(square_index(0, 0), 0);
        assert_eq!(square_index(7, 7), 63);
        assert_eq!(square_index(3, 4), 28);
        assert_eq!(square_index(4, 3), 35);
    }

    #[test]
    fn test_rank_and_file_index() {
        const RANK: usize = 3;
        const FILE: usize = 5;
        let index = square_index(RANK, FILE);
        assert_eq!(rank_index(index), RANK);
        assert_eq!(file_index(index), FILE);
    }

    #[test]
    fn test_single_cell_board() {
        const RANK: usize = 3;
        const FILE: usize = 5;
        let index = square_index(RANK, FILE);
        let board = get_single_bit_board(RANK as isize, FILE as isize);
        assert_eq!(board, (1 as BitBoard) << index);
    }

    #[test]
    fn test_single_cell_board_out_of_bounds() {
        assert_eq!(get_single_bit_board(-1, -1), 0);
        assert_eq!(get_single_bit_board(BOARD_RANKS as isize, 0), 0);
        assert_eq!(get_single_bit_board(0, BOARD_FILES as isize), 0);
    }
}
