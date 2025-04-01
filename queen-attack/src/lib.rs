#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank <= 7 && file >= 0 && file <= 7 {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank1 = self.position.rank;
        let file1 = self.position.file;
        let rank2 = other.position.rank;
        let file2 = other.position.file;

        if rank1 == rank2 && file1 == file2 {
            return true;
        }

        if rank1 == rank2 {
            return true;
        }

        if file1 == file2 {
            return true;
        }

        let rank_diff = (rank1 - rank2).abs();
        let file_diff = (file1 - file2).abs();
        rank_diff == file_diff
    }
}
