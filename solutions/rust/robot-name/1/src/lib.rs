use std::collections::HashSet;
use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::prelude::*;

pub struct Robot{
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self { name: Robot::build_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::build_name(); 
    }

    fn build_name() -> String {
        lazy_static! {
            static ref name_factory: Mutex<NameFactory> = Mutex::new(NameFactory::new());
        }        
        let factory = name_factory.lock();
        factory.unwrap().create_unique()
    }
}

struct NameFactory {
    names: HashSet<String>,
}

impl NameFactory {
    fn new() -> Self {
        Self { names: HashSet::new() }
    }

    fn create_unique(&mut self) -> String {
        for _ in 0..10 {
            let name = self.create();
            if !self.names.contains(&name) {
                self.names.insert(name.clone());
                return name;
            }
        }
        panic!("could generate unique name")
    }

    fn create(&mut self) -> String {
        let mut rng = thread_rng();
        let first_letter = rng.gen_range('A'..='Z');
        let second_letter = rng.gen_range('A'..='Z');
        let number = rng.gen_range(0..=999);
        format!("{first_letter}{second_letter}{number:03}")
    }    
}