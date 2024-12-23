use std::collections::HashMap;
const CC: [u32; 2] = [0, 10];
const CC_TOTAL: u32 = 16;
const C: [u32; 10] = [0, 10, 11, 24, 39, 5, 40, 40, 41, 42];
const C_TOTAL: u32 = 16;
// TODO
pub fn run() {
    eprintln!("84 to do");
    println!("{}", solve(4));
}
fn solve(rolls: u32) -> String {
    let results = simulate(rolls);
    let mut v = vec![0f64; 40];
    for ((k, _), c) in simulate(6) {
        v[k as usize] += c;
    }
    let mut v: Vec<_> = v.iter().enumerate().collect();
    v.sort_by(|a,b| b.1.partial_cmp(a.1).unwrap());
    println!("{v:?}");
    let mut str = String::new();
    for i in 0usize..3 {
        str.push_str(format!("{:02}", v[i].0).as_str());
    }
    return str;
}
fn simulate(rolls: u32) -> HashMap<(u32, i32), f64> {
    let mut map = HashMap::new();
    map.insert((0, 0), 1f64);
    for _ in 0..1_000 {
        let mut new_map = HashMap::new();
        for ((mut place, mut doubles), chance) in map {
            for r1 in 1..=rolls {
                for r2 in 1..=rolls {
                    let place = (place + r1 + r2) % 40;
                    let doubles = if r1 == r2 { doubles + 1 } else { 0 };
                    let add = chance / (rolls * rolls) as f64;
                    if doubles == 3 {
                        new_map
                            .entry((10, 0))
                            .and_modify(|v| *v += add)
                            .or_insert(add);
                        continue;
                    }
                    match place {
                        7 | 22 | 36 => {
                            for c in C {
                                let mut place = match c {
                                    40 => match place {
                                        7 => 15,
                                        22 => 25,
                                        36 => 5,
                                        _ => unreachable!(),
                                    },
                                    41 => match place {
                                        7 => 12,
                                        22 => 28,
                                        36 => 12,
                                        _ => unreachable!(),
                                    },
                                    42 => place - 3,
                                    _ => c,
                                };
                                new_map
                                    .entry((place, doubles))
                                    .and_modify(|v| *v += add / C_TOTAL as f64)
                                    .or_insert(add / C_TOTAL as f64);
                            }
                            new_map
                                .entry((place, doubles))
                                .and_modify(|v| {
                                    *v += add * (C_TOTAL - C.len() as u32) as f64 / C_TOTAL as f64
                                })
                                .or_insert(
                                    add * (C_TOTAL - C.len() as u32) as f64 / C_TOTAL as f64,
                                );
                        }
                        2 | 17 | 33 => {
                            for c in CC {
                                new_map
                                    .entry((c, doubles))
                                    .and_modify(|v| *v += add / CC_TOTAL as f64)
                                    .or_insert(add / CC_TOTAL as f64);
                            }
                            new_map
                                .entry((place, doubles))
                                .and_modify(|v| {
                                    *v +=
                                        add * (CC_TOTAL - CC.len() as u32) as f64 / CC_TOTAL as f64
                                })
                                .or_insert(
                                    add * (CC_TOTAL - CC.len() as u32) as f64 / CC_TOTAL as f64,
                                );
                        }
                        30 => {
                            new_map
                                .entry((10, doubles))
                                .and_modify(|v| *v += add)
                                .or_insert(add);
                        }
                        _ => {
                            new_map
                                .entry((place, doubles))
                                .and_modify(|v| *v += add)
                                .or_insert(add);
                        }
                    };
                }
            }
        }
        map = new_map;
    }
    map
}
