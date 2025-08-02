const digits: [char;9] = [' ', '1', '2', '3', '4', '5', '6', '7', '8'];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mf = Minefield(minefield);
    mf.0.iter().enumerate().map(|(y, row)| {
        row.chars().enumerate().map(|(x, pos)| {
            if pos == '*' {
                '*'
            } else {
                let x = x as isize;
                let y = y as isize;
                let num = mf.count_mine(x-1, y-1)
                    + mf.count_mine(x, y-1)
                    + mf.count_mine(x+1, y-1)
                    + mf.count_mine(x-1, y)
                    + mf.count_mine(x+1, y)
                    + mf.count_mine(x-1, y+1)
                    + mf.count_mine(x, y+1)
                    + mf.count_mine(x+1, y+1);
                digits[num as usize]
            }
        }).collect::<String>()
    }).collect::<Vec<String>>()
}

struct Minefield<'a>(&'a [&'a str]);

impl<'a> Minefield<'a> {
    fn count_mine(&self, x: isize, y: isize) -> u8 {
        if y >= 0 && y < self.0.len() as isize {
            let row = self.0[y as usize];
            if x >= 0 && x < row.len() as isize && row.chars().nth(x as usize).unwrap() == '*' {
                1
            } else {
                0
            }
        } else {   
            0
        }
    }
}