use std::collections::HashSet;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 198);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 230);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> usize {
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
    gamma * epsilon
}

fn find_rating(input: &str, allow_fn: &dyn Fn(usize, usize) -> char) -> usize {
    let mut nums: HashSet<&str> = HashSet::from_iter(input.lines());
    let mut i = 0;
    while nums.len() > 1 {
        let mut num_1 = 0;
        let length = nums.len();
        for num in nums.clone().iter() {
            if num.chars().nth(i).unwrap() == '1' {
                num_1 += 1
            }
        }
        let num_0 = length - num_1;
        let allow_char = allow_fn(num_0, num_1);
        for num in nums.clone().iter() {
            if num.chars().nth(i).unwrap() != allow_char {
                nums.remove(num);
            }
        }
        i += 1;
    }
    let result = nums.iter().next().unwrap();
    usize::from_str_radix(result, 2).unwrap()
}

fn mcv(num_0: usize, num_1: usize) -> char {
    match num_1 >= num_0 {
        true => '1',
        false => '0',
    }
}

fn lcv(num_0: usize, num_1: usize) -> char {
    match num_0 <= num_1 {
        true => '0',
        false => '1',
    }
}

fn calc2(input: &str) -> usize {
    let oxy = find_rating(input, &mcv);
    let co2 = find_rating(input, &lcv);
    oxy * co2
}
