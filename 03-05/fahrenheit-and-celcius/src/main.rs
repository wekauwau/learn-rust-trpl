mod celcius;
mod fahrenheit;
mod kelvin;

use std::{io, num::ParseFloatError};

use celcius::Celcius;
use fahrenheit::Fahrenheit;
use kelvin::Kelvin;

// progress: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main() {
    let mut choice: u8;
    let mut num = 0.0;
    let mut result: f64;

    'menu: loop {
        println!();
        println!("-----------------------------------------------------");
        println!("1. Celcius");
        println!("2. Fahrenheit");
        println!("3. Kelvin");
        println!("0. Exit");
        println!("Choose temperature:");
        println!("-----------------------------------------------------");

        match get_input().trim().parse() {
            Ok(v) => choice = v,
            Err(_) => {
                wrong_menu();
                continue;
            }
        }

        if choice == 0 {
            return;
        } else if choice == 1 {
            loop {
                match get_temp() {
                    Ok(v) => {
                        Celcius::new(v).convert();
                        continue 'menu;
                    }
                    Err(_) => {
                        not_number();
                    }
                }
            }
        } else {
            wrong_menu();
        }
    }
}

fn wrong_menu() {
    println!("=====================================================");
    println!("Pls input number on menu!");
    println!("=====================================================");
}

fn not_number() {
    println!("=====================================================");
    println!("Pls input a number!");
    println!("=====================================================");
}

fn get_temp() -> Result<f64, ParseFloatError> {
    println!();
    println!("-----------------------------------------------------");
    println!("Masukan temperatur:");
    println!("-----------------------------------------------------");
    get_input().trim().parse()
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input
}
