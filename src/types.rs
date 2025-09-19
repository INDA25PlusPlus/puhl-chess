use crate::core::board::*;
use crate::core::board::{ BOARD_SIZE, BOARD_FILES, BOARD_RANKS };

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct BoundedUsize<const MAX: usize>(usize);

// None inclusive bound
impl<const MAX: usize> BoundedUsize<MAX> {
    pub fn new(value: usize) -> Option<BoundedUsize<MAX>> {
        if value < MAX {
            Some(BoundedUsize(value))
        } else {
            None
        }
    }

    pub fn get(self) -> usize {
        self.0
    }
}

pub type Rank = BoundedUsize<{BOARD_RANKS}>;
pub type File = BoundedUsize<{BOARD_FILES}>;
pub type Index = BoundedUsize<{BOARD_SIZE}>;

impl Index {
    pub fn as_bb(&self) -> BitBoard {
        assert!(self.get() < 64);
        (1 as BitBoard) << self.get()
    }

    pub fn get_rank(&self) -> Rank {
        Rank::new(rank_index(self.get())).unwrap()
    }

    pub fn get_file(&self) -> File {
        File::new(file_index(self.get())).unwrap()
    }
}