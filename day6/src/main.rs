use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 5934);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 26984457539);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> usize {
    let mut fish: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    for _ in 1..=80 {
        for i in 0..fish.len() {
            let f = fish[i];
            let mut num_new_fish = 0;
            let new_f = match f {
                0 => {
                    num_new_fish += 1;
                    6
                }
                _ => f - 1,
            };
            fish[i] = new_f;
            for _ in 0..num_new_fish {
                fish.push(8);
            }
        }
    }
    fish.len()
}

fn calc2(input: &str) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| {
            let val = match map.get(&x) {
                Some(n) => *n + 1,
                None => 1,
            };
            map.insert(x, val);
        });
    for _ in 1..=256 {
        let mut new_map: HashMap<usize, usize> = HashMap::new();
        for (key, val) in map {
            if key == 0 {
                let new_val = new_map.entry(6).or_insert(0);
                *new_val += val;
                let new_val = new_map.entry(8).or_insert(0);
                *new_val += val;
            } else {
                let new_val = new_map.entry(key - 1).or_insert(0);
                *new_val += val;
            }
        }
        map = new_map;
    }
    map.values().sum()
}
