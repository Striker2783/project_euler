use std::env::args;

fn main() {
    let mut args = args();
    args.next();
    let n = match args.next() {
        Some(a) => match a.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Input a number");
                return;
            }
        },
        None => {
            eprintln!("Running all");
            run_all();
            return;
        }
    };
    run(n);
}
pub fn run(n: usize) {
    if n <= 100 {
        one_hundred::run(n)
    }
}

pub fn run_all() {
    one_hundred::run_all();
}