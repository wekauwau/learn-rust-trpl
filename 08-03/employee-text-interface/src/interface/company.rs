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

    fn get_department_names_sorted(&self) -> Vec<&String> {
        let mut department_names: Vec<&String> = self.department.keys().collect();
        department_names.sort();
        department_names
    }

    pub fn list_all(&self) {
        for department_name in self.get_department_names_sorted() {
            println!("{} Department", department_name);
            let department = self.department.get(department_name).unwrap();
            department.list();
            println!();
        }
    }

    pub fn list_department(&self) {
        println!("Departments:");

        for name in self.get_department_names_sorted() {
            println!("{}", name);
        }
    }

    pub fn list_employee(&self, name: &String) {
        match self.department.get(name) {
            Some(department) => {
                println!("{} Department's employees:", name);
                department.list();
            }
            None => println!("There is no {} Department", &name),
        }
    }

    pub fn add_department(&mut self, name: String) {
        // INFO: Do nothing when there is already department with same name
        self.department.entry(name).or_insert(Department::new());
    }

    pub fn add_employee(&mut self, employee: String, department_name: &String) {
        match self.department.get_mut(department_name) {
            Some(department) => {
                department.add(employee);
                println!(" to {}", department_name);
            }
            None => println!("There is no {} Department", department_name),
        }
    }
}
