fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 7);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // // Part 2
    assert_eq!(calc2(include_str!("test.in")), 5);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> u32 {
    let mut depths = input.split("\n").filter_map(|x| x.parse::<u32>().ok());
    let mut count = 0;
    let mut prev = depths.next().unwrap();
    for curr in depths {
        if curr > prev {
            count += 1;
        }
        prev = curr;
    }
    count
}

fn calc2(input: &str) -> u32 {
    input
        .split("\n")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(0, |mut acc, x| {
            if x[1] > x[0] {
                acc += 1;
            }
            acc
        })
}
