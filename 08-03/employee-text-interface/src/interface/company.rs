use std::collections::HashMap;

use department::Department;

mod department;

pub struct Company {
    pub department: HashMap<String, Department>,
}

impl Company {
    pub fn new() -> Self {
        Company {
            department: HashMap::new(),
        }
    }

    pub fn list_department(&self) {
        let mut department_names: Vec<&String> = self.department.keys().collect();
        department_names.sort();

        println!("Departments:");
        for name in department_names {
            println!("{}", name);
        }
    }

    pub fn list_employee(&self, name: String) {
        match self.department.get(&name) {
            Some(department) => {
                println!("{} Department's employees:", name);
                department.list();
            }
            None => println!("There is no {} Department", &name),
        }
    }

    pub fn add_department(&mut self, name: String) {
        self.department.entry(name).or_insert(Department::new());
    }
}
