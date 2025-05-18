pub struct Department {
    pub employees: Vec<String>,
}

impl Department {
    pub fn new() -> Self {
        Department {
            employees: Vec::new(),
        }
    }

    pub fn list(&self) {
        for name in &self.employees {
            println!("{}", name);
        }
    }

    pub fn add(&mut self, name: String) {
        self.employees.push(name);
        print!("{} added", self.employees.last().unwrap());
        self.employees.sort();
    }
}
