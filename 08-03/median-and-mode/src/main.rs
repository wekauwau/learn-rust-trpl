use rand::Rng;
use std::{collections::HashMap, io};

// progress: https://doc.rust-lang.org/book/ch08-03-hash-maps.html

fn main() {
    let len = get_input("Vec<u8> length.");
    let range_max = get_input("u8 MAX range.");
    let mut rng = rand::thread_rng();
    let mut nums = Vec::with_capacity(len.into());
    for _ in 0..len {
        nums.push(rng.gen_range(1..=range_max));
    }

    println!("Vec<u8> = {:?}\n", nums);
    println!("median\t: {}", get_median(&mut nums));
    println!("mode\t: {}", get_mode(&nums));
}

fn get_input(prompt: &str) -> u8 {
    let warn = "Enter a number between 1 - 99";

    loop {
        println!("{prompt}");
        println!("{warn}:");

        let mut input_container = String::new();
        io::stdin()
            .read_line(&mut input_container)
            .expect("Error: Failed to read line!");
        match input_container.trim().parse() {
            Ok(v) => {
                if v > 0 && v < 100 {
                    println!();
                    return v;
                } else {
                    println!("Error: {warn}!");
                }
            }
            Err(_) => println!("Error: Not a valid u8!"),
        };
        println!();
    }
}

fn get_median(nums: &mut Vec<u8>) -> f64 {
    nums.sort_unstable();
    let len = nums.len();

    // avg two middle numbers if even
    if len % 2 == 0 {
        (nums[len / 2 - 1] as f64 + nums[len / 2] as f64) / 2.0
    } else {
        nums[len / 2] as f64
    }
}

fn get_mode(nums: &Vec<u8>) -> u8 {
    let mut occurrences = HashMap::new();

    // Count occurrences of each number
    for &num in nums {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    // Find the number with the highest occurrence
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .unwrap() // Safe because the vector is non-empty
}
