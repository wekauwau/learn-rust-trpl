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
}
