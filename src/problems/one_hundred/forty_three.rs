pub fn run() {
    let mut permutations = Permutations::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    for p in permutations {
        println!("{p:?}");
    }
}

struct Permutations<T: Default + Copy, const N: usize> {
    permutation: [T; N],
    active: [bool; N],
    indices: Option<[usize; N]>,
}

impl<T: Default + Copy, const N: usize> Iterator for Permutations<T, N> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.indices.is_none() {
            let mut indices = [0; N];
            for i in 1..N {
                indices[i] = i;
            }
            self.indices = Some(indices);
            return Some(unsafe { self.indices_to_arr() });
        }
        let mut indices = &mut self.indices.unwrap();
        let mut i = N - 1;
        println!("{indices:?}");
        todo!()
    }
}

impl<T: Default + Copy, const N: usize> Permutations<T, N> {
    unsafe fn indices_to_arr(&self) -> [T; N] {
        let mut values = [T::default(); N];
        for i in 0..N {
            values[i] = self.permutation[self.indices.unwrap()[i]];
        }
        values
    }
    pub fn new(permutation: [T; N]) -> Self {
        Self {
            permutation,
            active: [false; N],
            indices: None,
        }
    }
}
