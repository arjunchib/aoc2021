fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 150);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 900);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> isize {
    let lines = input.lines();
    let mut depth = 0;
    let mut horizontal = 0;
    for line in lines {
        let (cmd, val) = line.split_once(" ").unwrap();
        let val: isize = val.parse().unwrap();
        match cmd {
            "forward" => horizontal += val,
            "down" => depth += val,
            "up" => depth -= val,
            _ => {}
        }
    }
    depth * horizontal
}

fn calc2(input: &str) -> isize {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in input.lines() {
        let (cmd, x) = line.split_once(" ").unwrap();
        let x: isize = x.parse().unwrap();
        match cmd {
            "forward" => {
                horizontal += x;
                depth += aim * x;
            }
            "down" => aim += x,
            "up" => aim -= x,
            _ => {}
        }
    }
    depth * horizontal
}
