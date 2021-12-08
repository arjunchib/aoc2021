use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // let test = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
    // println!("{:?}", decode(test));

    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 26);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 61229);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let (_, output) = line.split_once(" | ").unwrap();
        total += output
            .split(' ')
            .filter(|x| match x.len() {
                2 | 4 | 7 | 3 => true,
                _ => false,
            })
            .count();
    }
    total
}

fn decode(input: &str) -> HashMap<usize, HashSet<char>> {
    let mut knowns = HashMap::new();
    let mut unknowns = vec![];
    let signals = input.split(' ');
    for signal in signals {
        let set: HashSet<char> = signal.chars().collect();
        match set.len() {
            2 => {
                knowns.insert(1, set);
            }
            4 => {
                knowns.insert(4, set);
            }
            7 => {
                knowns.insert(8, set);
            }
            3 => {
                knowns.insert(7, set);
            }
            _ => unknowns.push(set),
        };
    }
    // Fives
    let fives = unknowns.iter().filter(|x| x.len() == 5);

    // 2
    let two = fives
        .clone()
        .find(|x| x.intersection(&knowns[&4]).count() == 2);
    knowns.insert(2, two.unwrap().clone());
    // 3
    let three = fives
        .clone()
        .find(|x| x.intersection(&knowns[&1]).count() == 2);
    knowns.insert(3, three.unwrap().clone());
    // 5
    let five = fives.clone().find(|x| {
        x.intersection(&knowns[&2]).count() != 5 && x.intersection(&knowns[&3]).count() != 5
    });
    knowns.insert(5, five.unwrap().clone());

    // Sixes
    let sixes = unknowns.iter().filter(|x| x.len() == 6);

    // 6
    let six = sixes
        .clone()
        .find(|x| x.intersection(&knowns[&1]).count() == 1);
    knowns.insert(6, six.unwrap().clone());

    // 9
    let nine = sixes
        .clone()
        .find(|x| x.intersection(&knowns[&4]).count() == 4);
    knowns.insert(9, nine.unwrap().clone());

    // 0
    let zero = sixes.clone().find(|x| {
        x.intersection(&knowns[&6]).count() != 6 && x.intersection(&knowns[&9]).count() != 6
    });
    knowns.insert(0, zero.unwrap().clone());

    knowns
}

fn calc2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let (signals, output) = line.split_once(" | ").unwrap();
        let patterns = decode(signals);
        let out_num_str: String = output
            .split(' ')
            .map(|x| {
                let set: HashSet<char> = x.chars().collect();
                let num = patterns
                    .iter()
                    .find(|(_, pattern)| set.symmetric_difference(pattern).count() == 0)
                    .unwrap()
                    .0;
                num.to_string()
            })
            .collect();
        total += usize::from_str_radix(out_num_str.as_str(), 10).unwrap();
    }
    total
}
