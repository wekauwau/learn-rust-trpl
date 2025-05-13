use std::{char, io::stdin};

const VOWEL: [char; 5] = ['a', 'i', 'u', 'e', 'o'];
const Y: char = 'y';

fn main() {
    let text = get_input();
    for word in text.split_whitespace() {
        print!("{} ", to_pig_latin(word));
    }
    println!();
}

fn get_input() -> String {
    println!("Enter text:");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: Failed to read line!");

    input
}

fn is_vowel(c: &char) -> bool {
    VOWEL.contains(c)
}

fn to_pig_latin(word: &str) -> String {
    // rules: https://www.wikihow.com/Speak-Pig-Latin
    let mut chars = word.chars();
    let first = chars.next().unwrap().to_ascii_lowercase();

    if is_vowel(&first) {
        // rule 2
        format!("{word}-yay")
    } else {
        let second = chars.next().unwrap().to_ascii_lowercase();
        let rest: String = chars.collect();

        if is_vowel(&second) || Y == second {
            // rule 1.0, 3.0, 3.1
            format!("{second}{rest}-{first}ay")
        } else {
            // rule 1.1, 3.2
            format!("{rest}-{first}{second}ay")
        }
    }
}
