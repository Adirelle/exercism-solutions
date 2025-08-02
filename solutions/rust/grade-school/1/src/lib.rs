use std::collections::BTreeMap;

pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> Self {
        Self {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.entry(grade).or_default().push(student.into());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.grades.get(&grade) {
            Some(students) => {
                let mut v = students.to_owned();
                v.sort();
                v
            }
            None => vec![],
        }
    }
}

impl Default for School {
    fn default() -> Self {
        Self::new()
    }
}
