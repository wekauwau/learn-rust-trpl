pub mod department;

use std::collections::{hash_map::Entry, HashMap};

use department::Department;

pub struct Company {
    pub department: HashMap<String, Department>,
}

impl Company {
    pub fn add(&mut self, department: String) {
        match self.department.entry(department) {
            Entry::Vacant(v) => {
                v.insert(Department {
                    employees: Vec::new(),
                });
            }
            Entry::Occupied(o) => {
                println!("{} department already exists", o.key());
            }
        }
    }
}
