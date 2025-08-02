// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        }
    }
    fn left(&self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
    fn right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            d: self.d.right(),
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            d: self.d.left(),
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (dx, dy) = self.d.delta();
        Self {
            x: self.x + dx,
            y: self.y + dy,
            ..self
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            match c {
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                'A' => robot = robot.advance(),
                _ => panic!("unknwon instruction: {:?}", c),
            }
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
