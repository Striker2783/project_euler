use std::env::args;

use project_euler::{self, run};

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
            eprintln!("Insert a number");
            return;
        }
    };
    run(n);
}
