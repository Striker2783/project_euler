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

#[macro_export]
macro_rules! hundreds {
    ($f: expr) => {
        const F: &[fn(Option<usize>)] = $f;
        pub fn run(n: usize) {
            if n / 10 > F.len() {
                return;
            }
            F[n / 10](Some(n % 10));
        }
        pub fn run_all() {
            for s in F {
                s(None);
            }
        }
    };
}
