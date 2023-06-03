pub fn elevent() {
    const MAX_ADJACENT: usize = 4;
    let a = file_parsers::read_txt_into_separate_nums("Files/ten.txt");
    // let horizontal = (0usize..a.len())
    //     .filter_map(|i| {
    //         let row = &a[i];
    //         (0..(row.len() - MAX_ADJACENT + 1))
    //             .map(|j| {
    //                 if row[j] == 0 {
    //                     return 0;
    //                 }
    //                 (j..(MAX_ADJACENT + j)).fold(1, |a, k| a * row[k])
    //             })
    //             .max()
    //     })
    //     .max();
    let column_length = a[0].len();
    let vertical = (0usize..column_length)
        .filter_map(|i| {
            (0..(column_length - MAX_ADJACENT + 1))
                .map(|j| {
                    if a[j][i] == 0 {
                        return 0;
                    }
                    (j..(MAX_ADJACENT + j)).fold(1, |acc, k| acc * a[k][i])
                })
                .max()
        })
        .max();
    println!("{:?}", vertical)
}
pub fn read_txt_into_separate_nums<P: AsRef<Path>>(filename: P) -> Vec<Vec<u64>> {
    let read_lines = read_lines(filename);

    read_lines
        .map(|x| {
            x.expect("Failed")
                .split(" ")
                .map(|x| x.parse::<u64>().expect("Failed"))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}
