use interface::company::Company;

mod interface;

fn main() {
    let mut app = Company::new();
    while app.command() {}
}
