#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition(rank, file))
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (self_rank, self_file, other_rank, other_file) = (self.0.0, self.0.1, other.0.0, other.0.1);
        let same_rank = (self_rank - other_rank).abs();
        let same_file = (self_file - other_file).abs();
        same_rank == 0 || same_file == 0 || same_rank == same_file
    }
}
