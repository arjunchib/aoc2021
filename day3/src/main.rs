use std::collections::HashSet;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 198);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 230);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn power(input: &str) -> (usize, usize) {
    let mut lines = input.lines().peekable();
    let length = lines.peek().unwrap().len();
    let mut counts = vec![0; length];
    let mut num_lines = 0;
    for line in lines {
        num_lines += 1;
        for (i, val) in line.char_indices() {
            if val == '1' {
                counts[i] += 1;
            }
        }
    }
    let half_lines = num_lines / 2;
    let binary_str = counts
        .iter()
        .map(|c| match c >= &half_lines {
            true => '1',
            false => '0',
        })
        .collect::<String>();
    let gamma = usize::from_str_radix(binary_str.as_str(), 2).unwrap();
    let shift = usize::BITS - length as u32;
    let epsilon = !gamma << shift >> shift;
    (gamma, epsilon)
}

fn calc1(input: &str) -> usize {
    let (gamma, epsilon) = power(input);
    gamma * epsilon
}

fn calc2(input: &str) -> usize {
    let (gamma, epsilon) = power(input);
    let mut lines = input.lines().peekable();
    let length = lines.peek().unwrap().len();
    let mut i = 0;
    let mut possible_co2: HashSet<&str> = HashSet::from_iter(lines.clone());
    let mut possible_o2: HashSet<&str> = HashSet::from_iter(lines.clone());
    let gamma_str = format!("{:b}", gamma);
    let eplison_str = format!("{:b}", epsilon);
    while i < length {
        if possible_co2.len() > 1 {
            possible_co2.clone().iter().for_each(|x| {
                if x.chars().nth(i).unwrap() != gamma_str.chars().nth(i).unwrap() {
                    possible_co2.remove(x);
                }
            })
        }
        if possible_o2.len() > 1 {
            possible_o2.clone().iter().for_each(|x| {
                if x.chars().nth(i).unwrap() != eplison_str.chars().nth(i).unwrap() {
                    possible_o2.remove(x);
                }
            })
        }
        println!("{:?}", gamma_str);
        i += 1;
    }
    let co2 = possible_co2.iter().next().unwrap();
    let co2 = usize::from_str_radix(co2, 2).unwrap();
    let o2 = possible_o2.iter().next().unwrap();
    let o2 = usize::from_str_radix(o2, 2).unwrap();
    co2 * o2
}
