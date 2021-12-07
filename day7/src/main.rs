fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 37);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 168);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn parse(input: &str) -> Vec<isize> {
    input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect()
}

fn median(a: &Vec<isize>) -> isize {
    let mut a = a.clone();
    a.sort();
    let len = a.len();
    match a.len() % 2 {
        0 => (a[len / 2 - 1] + a[len / 2]) / 2,
        _ => a[len / 2],
    }
}

fn calc1(input: &str) -> isize {
    let crabs = parse(input);
    let median = median(&crabs);
    crabs
        .into_iter()
        .fold(0, |acc, crab| acc + (median - crab).abs())
}

fn total_moves(crabs: &Vec<isize>, center: isize) -> isize {
    crabs.iter().fold(0, |acc, crab| {
        let n = (crab - center).abs();
        acc + n * (n + 1) / 2
    })
}

fn calc2(input: &str) -> isize {
    let crabs = parse(input);
    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    (min..=max)
        .map(|center| total_moves(&crabs, center))
        .min()
        .unwrap()
}
