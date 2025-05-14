pub struct Department {
    pub employees: Vec<String>,
}

impl Department {
    pub fn add(&mut self, employee: String) {
        self.employees.push(employee);
        self.employees.sort();
    }

    pub fn list(&self) {
        for i in &self.employees {
            println!("{i}");
        }
    }
}
