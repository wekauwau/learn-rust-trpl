// progress: https://doc.rust-lang.org/book/ch06-00-enums.html

fn main() {
    let n = 43;

    println!("fib({n}) = {}", get_fib(n));
    println!("fib({n}) = {}", get_fib_slow(n));
}

fn get_fib(n: u64) -> u64 {
    if n > 1 {
        let mut temp_n_2: u64 = 0;
        let mut n_2: u64 = 0;
        let mut n_1: u64 = 1;
        let mut counter = 0;

        while counter < n - 2 {
            n_2 = n_1;
            n_1 += temp_n_2;
            temp_n_2 = n_2;

            counter += 1;
        }

        n_1 + n_2
    } else {
        n
    }
}

fn get_fib_slow(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => get_fib_slow(n - 1) + get_fib_slow(n - 2),
    }
}
