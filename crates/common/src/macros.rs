#[macro_export]
macro_rules! tens {
    ($f: expr) => {
        const F: &[fn()] = $f;
        pub fn run(n: Option<usize>) {
            if let Some(n) = n {
                if n >= F.len() {
                    return;
                }
                F[n]();
            } else {
                for f in F {
                    f();
                }
            }
        }
    };
}