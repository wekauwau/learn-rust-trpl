pub struct Department {
    pub employees: Vec<String>,
}

impl Department {
    pub fn new() -> Self {
        Department {
            employees: Vec::new(),
        }
    }
}
