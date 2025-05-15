use interface::company::Company;

mod interface;

fn main() {
    let app = Company::new();
    while app.command() {}
}
