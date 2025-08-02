use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
enum Cell {
    Empty,
    Vertical,
    Horizontal,
    Crossing,
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            ' ' => Self::Empty,
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            '+' => Self::Crossing,
            _ => return Err(format!("invalid cell: {}", c)),
        })
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    fn from_lines(lines: &[&str]) -> Self {
        let height = lines.len();
        let width = if height > 0 { lines[0].len() } else { 0 };
        let mut cells: Vec<Cell> = Vec::with_capacity(width * height);
        for line in lines {
            assert!(line.len() == width);
            for cell in line.chars() {
                cells.push(Cell::try_from(cell).expect("invalid grid"));
            }
        }
        Self {
            width,
            height,
            cells,
        }
    }

    fn try_get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[x + y * self.width])
        } else {
            None
        }
    }

    fn is_horizontal(&self, x: usize, y: usize) -> bool {
        matches!(
            self.try_get(x, y),
            Some(Cell::Horizontal) | Some(Cell::Crossing)
        )
    }

    fn is_vertical(&self, x: usize, y: usize) -> bool {
        matches!(
            self.try_get(x, y),
            Some(Cell::Vertical) | Some(Cell::Crossing)
        )
    }

    fn is_crossing(&self, x: usize, y: usize) -> bool {
        matches!(self.try_get(x, y), Some(Cell::Crossing))
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Rectangle {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

pub fn count(lines: &[&str]) -> u32 {
    let grid = Grid::from_lines(lines);
    if grid.width < 1 || grid.height < 1 {
        return 0;
    }
    let mut rects: BTreeSet<Rectangle> = BTreeSet::new();
    for x in 0..grid.width - 1 {
        for y in 0..grid.height - 1 {
            if !grid.is_crossing(x, y) {
                continue;
            }
            for i in x + 1..grid.width {
                if !grid.is_horizontal(i, y) {
                    break;
                } else if !grid.is_vertical(i, y) {
                    continue;
                }
                for j in y + 1..grid.height {
                    if !grid.is_vertical(x, j) || !grid.is_vertical(i, j) {
                        break;
                    } else if !grid.is_horizontal(x, j) {
                        continue;
                    }
                    let mut k = x;
                    while k <= i && grid.is_horizontal(k, j) {
                        k += 1;
                    }
                    if k <= i {
                        continue;
                    }
                    rects.insert(Rectangle {
                        x,
                        y,
                        w: i - x,
                        h: j - y,
                    });
                }
            }
        }
    }
    rects.len() as u32
}
