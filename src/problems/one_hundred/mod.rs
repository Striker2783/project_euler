pub mod fifties;
pub mod forties;
pub mod ones;
pub mod sixties;
pub mod tens;
pub mod thirties;
pub mod twenties;
pub mod seventies;
pub mod eighties;

const SOLVERS: &[fn(Option<usize>)] = &[
    ones::run,
    tens::run,
    twenties::run,
    thirties::run,
    forties::run,
    fifties::run,
    sixties::run,
    seventies::run,
    eighties::run,
];

pub fn run(n: usize) {
    if (n / 10) as usize > SOLVERS.len() {
        return;
    }
    unsafe {
        SOLVERS[(n / 10) as usize](Some(n % 10));
    }
}
pub fn run_all() {
    for s in SOLVERS {
        s(None);
    }
}
