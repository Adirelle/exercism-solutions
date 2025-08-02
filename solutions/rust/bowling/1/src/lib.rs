use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: u8,
    score: u16,
    pins_up: u16,
    second_roll: bool,
    spare: bool,
    strike1: u8,
    strike2: u8,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: 0,
            score: 0,
            pins_up: 10,
            second_roll: false,
            spare: false,
            strike1: 0,
            strike2: 0,
        }
    }

    fn is_complete(&self) -> bool {
        self.frames >= 10 && !self.spare && self.strike1 == 0 && self.strike2 == 0
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_complete() {
            Err(Error::GameComplete)
        } else if pins > self.pins_up {
            Err(Error::NotEnoughPinsLeft)
        } else {
            if self.spare {
                self.score += pins;
                self.spare = false;
            }
            if self.strike1 > 0 {
                self.strike1 -= 1;
                self.score += pins;
            }
            if self.strike2 > 0 {
                self.strike2 -= 1;
                self.score += pins;
            }
            if self.frames < 10 {
                self.score += pins;
            }

            self.pins_up -= pins;

            if self.pins_up == 0 && self.frames < 10 {
                if self.second_roll {
                    self.spare = true;
                } else if self.strike1 > 0 {
                    self.strike2 = 2;
                } else {
                    self.strike1 = 2;
                }
            }

            if self.pins_up == 0 || self.second_roll {
                self.pins_up = 10;
                self.second_roll = false;
                self.frames += 1;
            } else {
                self.second_roll = true;
            }

            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        self.is_complete().then_some(self.score)
    }
}
