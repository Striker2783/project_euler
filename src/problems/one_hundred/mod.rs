pub mod fifties;
pub mod forties;
pub mod ones;
pub mod tens;
pub mod thirties;
pub mod twenties;

const SOLVERS: &[fn(usize)] = &[
    ones::run,
    tens::run,
    twenties::run,
    thirties::run,
    forties::run,
    fifties::run,
];

pub fn run(n: usize) {
    if (n / 10) as usize > SOLVERS.len() {
        return;
    }
    unsafe {
        SOLVERS[(n / 10) as usize](n % 10);
    }
}
pub fn run_all() {}
