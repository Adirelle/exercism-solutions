pub struct Triangle(u64, u64, u64);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        let [a, b, c] = sides;
        if a > 0 && b > 0 && c > 0 && a + b >= c && b + c >= a && a + c >= b {
            Some(Triangle(a, b, c))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2 && self.2 != self.0
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.2 == self.0
    }
}
