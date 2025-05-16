use std::io::stdin;

use company::Company;

pub mod company;

impl Company {
    fn get_input(&self) -> String {
        println!("Enter command:");

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Error: Failed to read line!");

        input
    }

    pub fn command(&mut self) -> bool {
        let again = true;

        println!();
        println!("-----------------------------------------------------");
        let input = self.get_input();
        println!("-----------------------------------------------------");

        let mut command = input.split_whitespace();
        let first = command.next().unwrap_or("");

        if input.trim() == "list department" {
            self.list_department();
        } else if first == "add" {
            let rest = command.collect::<Vec<_>>().join(" ");
            self.add_department(rest);
        } else {
            self.help();
        }

        println!("-----------------------------------------------------");
        again
    }

    fn help(&self) {
        println!("Usage:");
        print!("list department\t\t\t");
        println!("List all departments");
        print!("add <department>\t\t");
        println!("Add department");
    }
}
