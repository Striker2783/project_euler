macro_rules! tens {
    ($f: expr) => {
        const F: &[fn()] = $f;
        pub fn run(n: usize) {
            if n >= F.len() {
                return;
            }
            F[n]()
        }
    };
}

pub(crate) use tens;
