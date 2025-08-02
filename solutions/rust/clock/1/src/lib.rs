use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        let mut t = (h * 60 + m) % 1440;
        if t < 0 {
            t += 1440;
        }

        Clock { hours: t / 60, minutes: t % 60 }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}