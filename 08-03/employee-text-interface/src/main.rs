use std::collections::HashMap;

use company::Company;

mod company;
mod interface;

fn main() {
    let mut company = Company {
        department: HashMap::new(),
    };

    println!("Usage:");
    println!("add <name> to <department>");
    println!("list");
    println!("list <department>");
    println!();
}
