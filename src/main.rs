use std::env::args;

use project_euler::{self, problems::one_hundred::run_all, run};

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
