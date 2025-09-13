pub type Board = u64;

pub const BOARD_RANKS: usize = 8;
pub const BOARD_FILES: usize = 8;
pub const BOARD_SIZE: usize = BOARD_RANKS * BOARD_FILES;

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
pub const fn single_square_board(rank: isize, file: isize) -> Board {
    if rank < 0 || file < 0 || rank >= BOARD_RANKS as isize || file >= BOARD_FILES as isize {
        return 0;
    }

    (1 as Board) << (rank as usize * BOARD_FILES + file as usize)
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
        let board = single_square_board(RANK as isize, FILE as isize);
        assert_eq!(board, (1 as Board) << index);
    }

    #[test]
    fn test_single_cell_board_out_of_bounds() {
        assert_eq!(single_square_board(-1, -1), 0);
        assert_eq!(single_square_board(BOARD_RANKS as isize, 0), 0);
        assert_eq!(single_square_board(0, BOARD_FILES as isize), 0);
    }
}
