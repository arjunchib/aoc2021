fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 26397);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 288957);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn first_error(input: &str) -> Option<char> {
    let mut stack = vec![];
    for x in input.chars() {
        if matches!(x, '(' | '[' | '{' | '<') {
            stack.push(x)
        } else {
            let is_valid = match stack.pop().unwrap() {
                '(' => x == ')',
                '[' => x == ']',
                '<' => x == '>',
                '{' => x == '}',
                _ => false,
            };
            if !is_valid {
                return Some(x);
            }
        }
    }
    None
}

fn calc1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|x| first_error(x))
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        })
        .sum()
}

fn complete(input: &str) -> usize {
    let mut stack = vec![];
    for x in input.chars() {
        if matches!(x, '(' | '[' | '{' | '<') {
            stack.push(x)
        } else {
            stack.pop();
        }
    }
    stack
        .into_iter()
        .rev()
        .map(|x| match x {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => ' ',
        })
        .fold(0, |acc, x| {
            let n = match x {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => 0,
            };
            acc * 5 + n
        })
}

fn calc2(input: &str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .filter(|x| first_error(x) == None)
        .map(|x| complete(x))
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}
