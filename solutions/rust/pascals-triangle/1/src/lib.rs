pub struct PascalsTriangle {
    count: u32
}

impl PascalsTriangle {
    pub fn new(count: u32) -> Self {     
        Self { count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.count == 0 {
            return Vec::new();
        }
        let mut rows = Vec::new();
        rows.push(vec![1]);
        for i in 1..self.count {
            let mut row = Vec::new();
            let mut prev = 0_u32;
            for x in rows.last().unwrap().iter() {
                row.push(x + prev);
                prev = *x;
            }
            row.push(1);
            rows.push(row);
        }
        rows
    }
}
