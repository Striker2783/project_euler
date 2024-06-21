pub mod fifty;
pub mod fifty_one;
pub mod fifty_two;

const F: &[fn()] = &[fifty::run, fifty_one::run, fifty_two::run];

pub unsafe fn run(n: usize) {
    F[n]()
}
