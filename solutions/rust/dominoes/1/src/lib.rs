use std::ops::RemAssign;

type Dots = u8;

#[derive(Clone,Debug,PartialEq,Eq,Copy)]
struct Stone {
    left: Dots,
    right: Dots
}

impl Stone {
    fn reverse(&self) -> Self {
        Stone {
            left: self.right,
            right: self.left,
        }
    }

    fn connect(&self, other: Stone) -> bool {
        return self.right == other.left
    }
}

impl From<&(Dots, Dots)> for Stone {
    fn from(value: &(Dots, Dots)) -> Self {
        Stone {
            left: value.0,
            right: value.1,
        }
    }
}

impl From<Stone> for (Dots, Dots) {
    fn from(value: Stone) -> Self {
        (value.left, value.right)
    }
}

#[derive(Clone,Debug,PartialEq,Eq)]
struct Chain(Vec<Stone>);

impl Chain {
    fn valid(&self) -> bool {
        if self.0.is_empty() {
            return false;
        }
        if self.0.last().unwrap().right != self.0.first().unwrap().left {
            return false;
        }
        true
    }

    fn add(&self, stone: Stone) -> Chain {
        let mut new = self.0.clone();
        new.push(stone);
        Chain(new)
    }

    fn try_add(&self, stone: Stone) -> Option<Chain> {
        if self.0.is_empty() {
            return Some(Chain(vec![stone]));
        }
        let last = self.0.last().unwrap();
        if last.connect(stone) {
            Some(self.add(stone))
        } else if let reverse = stone.reverse() && last.connect(reverse) {
            Some(self.add(reverse))
        } else {
            None
        }
    }

    fn make_chain(&self, set: &Vec<Stone>) -> Option<Chain> {
        if set.is_empty() {
            return if self.valid() {
                Some(self.clone())
            } else {
                None
            };
        }
        for (i, stone) in set.iter().enumerate() {
            if let Some(result) = self.try_add(*stone) {
                let mut remaining = set.clone();
                _ = remaining.remove(i);
                if let Some(chain) = result.make_chain(&remaining) {
                    return Some(chain);
                }
            }
        }
        None
    }

    fn new() -> Chain {
        Chain(vec![])
    }
}

pub fn chain(input: &[(Dots, Dots)]) -> Option<Vec<(Dots, Dots)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    let set = input.iter().map(|s| s.into()).collect::<Vec<_>>();
    let c = Chain::new();
    Some(
        c.make_chain(&set)?.0.into_iter().map(|x| x.into()).collect::<Vec<_>>()
    )
}
