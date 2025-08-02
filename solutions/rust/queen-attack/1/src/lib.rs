use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(Self {
                rank: rank as u8,
                file: file as u8,
            })
        } else {
            None
        }
    }
}

impl Sub<ChessPosition> for ChessPosition {
    type Output = (i16, i16);

    fn sub(self, rhs: ChessPosition) -> Self::Output {
        (
            self.rank as i16 - rhs.rank as i16,
            self.file as i16 - rhs.file as i16,
        )
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (dr, df) = other.position - self.position;
        dr == 0 || df == 0 || dr.abs() == df.abs()
    }
}
