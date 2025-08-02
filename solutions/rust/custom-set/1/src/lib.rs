use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CustomSet<T: Eq + Hash> {
    items: HashMap<T, ()>
}

impl<T: Eq + Hash + Clone> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self { items: HashMap::from_iter(input.iter().map(|i| (i.clone() ,()))) }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.items.contains_key(element)
    }

    pub fn add(&mut self, element: T) {
        self.items.insert(element, ());
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.items.keys().all(|item| other.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.items.keys().all(|item| !other.contains(item))
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        Self { 
            items: HashMap::from_iter(
                self.items
                    .keys()
                    .filter_map(|item| 
                        other.contains(item)
                            .then_some((item.clone(), ()))
                    ) 
            )
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        Self { 
            items: HashMap::from_iter(
                self.items
                    .keys()
                    .filter_map(|item| 
                        (!other.contains(item))
                            .then_some((item.clone(), ()))
                    ) 
            )
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut new = self.clone();
        new.items.extend(
            other.items
                .keys()
                .filter(|item| !self.items.contains_key(item))
                .map(|item| (item.clone(), ()))
        );
        new
    }
}
