pub struct Allergies(u32);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        self.0 & a.score() == a.score()
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        for a in Allergen::all().iter() {
            if self.is_allergic_to(a) {
                allergies.push(*a);
            }
        }
        allergies
    }
}

impl Allergen {
    pub fn score(&self) -> u32 {
        use Allergen::*;
        match self {
            Eggs => 1,
            Peanuts => 2,
            Shellfish => 4,
            Strawberries => 8,
            Tomatoes => 16,
            Chocolate => 32,
            Pollen => 64,
            Cats => 128,
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ][..]
    }
}
