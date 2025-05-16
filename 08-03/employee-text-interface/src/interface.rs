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
        let mut again = true;

        println!();
        println!("-----------------------------------------------------");
        let input = self.get_input();
        println!("-----------------------------------------------------");

        let mut command = input.split_whitespace();
        let input_trim = input.trim();
        let first = command.next().unwrap_or("");

        if input_trim == "list department" {
            self.list_department();
        } else if first == "list" {
            let name = command.collect::<Vec<_>>().join(" ");

            if name.is_empty() {
                self.help();
            } else {
                self.list_employee(name);
            }
        } else if first == "add" {
            let name = command.collect::<Vec<_>>().join(" ");
            self.add_department(name);
        } else if input_trim == "exit" {
            again = false;
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
        print!("list <department>\t\t");
        println!("List <departments>'s employees");
        print!("add <department>\t\t");
        println!("Add department");
        print!("exit\t\t\t\t");
        println!("Exit");
    }
}
