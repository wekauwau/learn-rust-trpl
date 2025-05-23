use std::io;

mod celcius;
mod fahrenheit;
mod kelvin;

// progress: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    let mut choice;
    let mut num = 0.0;
    let mut result: f64;

    loop {
        choice = get_choice();

        if choice == 0 {
            return;
        } else if choice == 1 {
            println!("F to C");
            println!();
            println!("Enter F:");

            if get_float(&mut num) {
                result = (num - 32.0) * (5.0 / 9.0);
                println!("{num}F = {result:.2}C");
            } else {
                println!("Pls input a number!");
            }
        } else {
            println!("C to F");
            println!();
            println!("Enter C:");

            if get_float(&mut num) {
                result = (num * (9.0 / 5.0)) + 32.0;
                println!("{num}C = {result:.2}F");
            } else {
                println!("Pls input a number!");
            }
        }
    }
}

/// returns i64 between 0 - 2
fn get_choice() -> i64 {
    let mut choice: i64 = 0;

    loop {
        println!();
        println!("1. Celcius");
        println!("2. Fahrenheit");
        println!("3. Fahrenheit");
        println!("0. Exit");
        println!("Choose temperature:");

        if get_int(&mut choice) && (0..=2).contains(&choice) {
            return choice;
        }

        println!();
        println!("Pls input a number between 0 - 2!");
    }
}

fn get_int(n: &mut i64) -> bool {
    *n = match get_input().trim().parse() {
        Ok(r) => r,
        Err(_) => return false,
    };

    true
}

fn get_float(n: &mut f64) -> bool {
    *n = match get_input().trim().parse() {
        Ok(r) => r,
        Err(_) => return false,
    };

    true
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input
}
