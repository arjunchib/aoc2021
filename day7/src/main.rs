fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 37);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 168);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> usize {
    let mut crabs: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    crabs.sort();
    let median = match crabs.len() % 2 {
        0 => (crabs[crabs.len() / 2] + crabs[crabs.len() / 2 - 1]) / 2,
        _ => crabs[crabs.len() / 2],
    };
    let mut dev = 0;
    for crab in crabs {
        dev += (median as isize - crab as isize).abs()
    }
    dev as usize
}

fn dev2(a: isize, b: isize) -> usize {
    let n = (a - b).abs();
    (n * (n + 1) / 2) as usize
}

fn calc2(input: &str) -> usize {
    let crabs: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut best = usize::MAX;
    for center in 0..=*crabs.iter().max().unwrap() {
        let mut dev = 0;
        for crab in crabs.iter() {
            dev += dev2(center as isize, *crab as isize);
        }
        if dev < best {
            best = dev;
        }
    }
    best
}
