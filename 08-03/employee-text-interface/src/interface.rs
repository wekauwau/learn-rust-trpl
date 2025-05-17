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
        println!();
        println!("-----------------------------------------------------");
        let input = self.get_input();
        println!("-----------------------------------------------------");

        let input_trim = input.trim();

        match input_trim {
            "list all" => self.list_all(),
            "list department" => self.list_department(),
            "exit" => return false,
            _ => {
                let command: Vec<&str> = input.split_whitespace().collect();
                match command[0] {
                    "list" => {
                        let name = command[1..].join(" ");

                        match name.is_empty() {
                            false => self.list_employee(&name),
                            true => self.help(),
                        }
                    }
                    "add" => match command.iter().position(|&w| w == "to") {
                        Some(index) => {
                            let employee = command[1..index].join(" ");
                            let department = command[index + 1..].join(" ");

                            match employee.is_empty() || department.is_empty() {
                                false => self.add_employee(employee, department),
                                true => self.help(),
                            }
                        }
                        None => {
                            let name = command[1..].join(" ");

                            match name.is_empty() {
                                false => self.add_department(name),
                                true => self.help(),
                            }
                        }
                    },
                    _ => self.help(),
                }
            }
        }

        // let first = command.next().unwrap_or("");

        // if input_trim == "list department" {
        //     self.list_department();
        // } else if first == "list" {
        //     let name = command.collect::<Vec<_>>().join(" ");
        //
        //     if name.is_empty() {
        //         self.help();
        //     } else {
        //         self.list_employee(name);
        //     }
        // } else if first == "add" {
        //     let name = command.collect::<Vec<_>>().join(" ");
        //     self.add_department(name);
        // } else if input_trim == "exit" {
        //     again = false;
        // } else {
        //     self.help();
        // }

        println!("-----------------------------------------------------");
        true
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
