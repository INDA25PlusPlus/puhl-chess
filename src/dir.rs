use crate::board::*;

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Dir {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub const DIR_COUNT: usize = 8;

impl Dir {
    pub const FROM_SQUARES_PAIRS: [[Option<Dir>; BOARD_SIZE]; BOARD_SIZE] = Dir::from_all_square_pairs();

    // Calculates Dir::from_slots for every pair of squares on the board
    const fn from_all_square_pairs() -> [[Option<Dir>; BOARD_SIZE]; BOARD_SIZE] {
        let mut result: [[Option<Dir>; BOARD_SIZE]; BOARD_SIZE] = [[None; BOARD_SIZE]; BOARD_SIZE];

        let mut index_src: usize = 0;
        while index_src < BOARD_SIZE {
            let mut index_dst: usize = 0;
            while index_dst < BOARD_SIZE {
                result[index_src][index_dst] = Dir::from_square_pair(index_src, index_dst);
                index_dst += 1;
            }
            index_src += 1;
        }
        result
    }

    // Returns the Dir direction between a source square and a destination square
    // If the pair are not on a straight cardinal direction (North, NorthWest, etc.) then the method returns None
    const fn from_square_pair(index_src: usize, index_dst: usize) -> Option<Dir> {
        let rank_src = rank_index(index_src);
        let file_src = file_index(index_src);
        let rank_dst = rank_index(index_dst);
        let file_dst = file_index(index_dst);

        let rank_diff = rank_dst as isize - rank_src as isize;
        let file_diff = file_dst as isize - file_src as isize;
        if rank_diff == 0 && file_diff == 0 {
            return None
        }

        match (rank_diff, file_diff) {
            (0, f) if f < 0 => Some(Dir::East),
            (0, f) if f > 0 => Some(Dir::West),
            (r, 0) if r > 0 => Some(Dir::North),
            (r, 0) if r < 0 => Some(Dir::South),
            (r, f) if r == f && r > 0 => Some(Dir::NorthWest),
            (r, f) if r == f && r < 0 => Some(Dir::SouthEast),
            (r, f) if r == -f && r > 0 => Some(Dir::NorthEast),
            (r, f) if r == -f && r < 0 => Some(Dir::SouthWest),
            _ => None
        }
    }

    pub fn opposite(dir: Dir) -> Dir {
        match dir {
            Dir::North => Dir::South,
            Dir::NorthEast => Dir::SouthWest,
            Dir::East => Dir::West,
            Dir::SouthEast => Dir::NorthWest,
            Dir::South => Dir::North,
            Dir::SouthWest => Dir::NorthEast,
            Dir::West => Dir::East,
            Dir::NorthWest => Dir::SouthEast,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_square_pair() {
        // TODO: Maybe use array + loop instead; maybe also create a helper module for testing
        assert_eq!(Dir::from_square_pair(square_index(0, 0), square_index(0, 1)), Some(Dir::West));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(5, 7)), Some(Dir::West));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(5, 1)), Some(Dir::East));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(7, 5)), Some(Dir::North));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(0, 5)), Some(Dir::South));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(7, 7)), Some(Dir::NorthWest));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(2, 2)), Some(Dir::SouthEast));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(7, 3)), Some(Dir::NorthEast));
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(4, 6)), Some(Dir::SouthWest));
    }

    #[test]
    fn test_from_square_pair_none() {
        // TODO: Maybe use array + loop instead; maybe also create a helper module for testing
        assert_eq!(Dir::from_square_pair(square_index(0, 0), square_index(0, 0)), None);
        assert_eq!(Dir::from_square_pair(square_index(5, 5), square_index(2, 3)), None);
        assert_eq!(Dir::from_square_pair(square_index(4, 5), square_index(7, 3)), None);
        assert_eq!(Dir::from_square_pair(square_index(5, 7), square_index(2, 6)), None);
    }

    #[test]
    fn test_from_all_square_pairs() {
        let board = Dir::from_all_square_pairs();
        assert_eq!(board[square_index(0, 0)][square_index(0, 0)], None);
        assert_eq!(board[square_index(0, 0)][square_index(0, 1)], Some(Dir::West));
        assert_eq!(board[square_index(5, 5)][square_index(5, 7)], Some(Dir::West));
        assert_eq!(board[square_index(5, 5)][square_index(5, 1)], Some(Dir::East));
        assert_eq!(board[square_index(5, 5)][square_index(7, 5)], Some(Dir::North));
        assert_eq!(board[square_index(5, 5)][square_index(0, 5)], Some(Dir::South));
        assert_eq!(board[square_index(5, 5)][square_index(7, 7)], Some(Dir::NorthWest));
        assert_eq!(board[square_index(5, 5)][square_index(2, 2)], Some(Dir::SouthEast));
        assert_eq!(board[square_index(5, 5)][square_index(7, 3)], Some(Dir::NorthEast));
        assert_eq!(board[square_index(5, 5)][square_index(4, 6)], Some(Dir::SouthWest));
    }
}
