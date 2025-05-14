use std::io::stdin;

pub fn get_input() -> String {
    println!("Enter command:");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: Failed to read line!");

    input
}

fn command(command: &str) {
    let mut words = get_input().split_whitespace();
    let first = words.next().unwrap().to_ascii_lowercase();

    if first == "add" {
        // add(name, department);
    } else if first == "list" {
        match words.next() {
            Some(department) => list_department(department),
            None => list(),
        }
    }
}
