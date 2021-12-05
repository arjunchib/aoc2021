use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 5);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 12);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn parse_coord(input: &str) -> (usize, usize) {
    let (x, y) = input.split_once(",").unwrap();
    let x: usize = x.parse().unwrap();
    let y: usize = y.parse().unwrap();
    (x, y)
}

fn calc1(input: &str) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (c1, c2) = line.split_once(" -> ").unwrap();
            (parse_coord(c1), parse_coord(c2))
        })
        .for_each(|(c1, c2)| {
            let (x1, y1) = c1;
            let (x2, y2) = c2;
            if x1 == x2 {
                let rng = match y1 < y2 {
                    true => y1..=y2,
                    false => y2..=y1,
                };
                for y in rng {
                    let count = match map.get(&(x1, y)) {
                        Some(n) => *n + 1,
                        None => 1,
                    };
                    map.insert((x1, y), count);
                }
            }
            if y1 == y2 {
                let rng = match x1 < x2 {
                    true => x1..=x2,
                    false => x2..=x1,
                };
                for x in rng {
                    let count = match map.get(&(x, y1)) {
                        Some(n) => *n + 1,
                        None => 1,
                    };
                    map.insert((x, y1), count);
                }
            }
        });
    map.into_values().filter(|x| *x >= 2).count()
}

fn calc2(input: &str) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (c1, c2) = line.split_once(" -> ").unwrap();
            (parse_coord(c1), parse_coord(c2))
        })
        .for_each(|(c1, c2)| {
            let (x1, y1) = c1;
            let (x2, y2) = c2;
            if x1 == x2 {
                let rng = match y1 < y2 {
                    true => y1..=y2,
                    false => y2..=y1,
                };
                for y in rng {
                    let count = match map.get(&(x1, y)) {
                        Some(n) => *n + 1,
                        None => 1,
                    };
                    map.insert((x1, y), count);
                }
            } else if y1 == y2 {
                let rng = match x1 < x2 {
                    true => x1..=x2,
                    false => x2..=x1,
                };
                for x in rng {
                    let count = match map.get(&(x, y1)) {
                        Some(n) => *n + 1,
                        None => 1,
                    };
                    map.insert((x, y1), count);
                }
            } else {
                let mut curr_x = x1;
                let mut curr_y = y1;
                while curr_x != x2 && curr_y != y2 {
                    let count = match map.get(&(curr_x, curr_y)) {
                        Some(n) => *n + 1,
                        None => 1,
                    };
                    map.insert((curr_x, curr_y), count);
                    match x1 < x2 {
                        true => curr_x += 1,
                        false => curr_x -= 1,
                    };
                    match y1 < y2 {
                        true => curr_y += 1,
                        false => curr_y -= 1,
                    };
                }
                let count = match map.get(&(curr_x, curr_y)) {
                    Some(n) => *n + 1,
                    None => 1,
                };
                map.insert((curr_x, curr_y), count);
            }
        });
    map.into_values().filter(|x| *x >= 2).count()
}
