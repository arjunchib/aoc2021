use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 4512);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 1924);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug)]
struct Board {
    spaces: HashMap<usize, (usize, usize)>,
    rows: [usize; 5],
    cols: [usize; 5],
    bingo: bool,
}

impl Board {
    // returns true if the mark created a bingo!
    fn mark(&mut self, num: usize) -> (bool, usize) {
        if !self.spaces.contains_key(&num) || self.bingo {
            return (false, 0);
        }
        let (c, r) = self.spaces.remove(&num).unwrap();
        self.rows[r] += 1;
        self.cols[c] += 1;
        self.bingo = self.rows[r] == 5 || self.cols[c] == 5;
        let score = self.spaces.keys().sum();
        (self.bingo, score)
    }
}

fn calc1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut nums = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap());
    let mut boards = vec![];
    let mut spaces = HashMap::new();
    let mut r = 0;
    while let Some(line) = lines.next() {
        if line == "" {
            continue;
        }
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .enumerate()
            .for_each(|(c, x)| {
                spaces.insert(x, (c, r));
            });
        r += 1;
        if r == 5 {
            boards.push(Board {
                spaces,
                rows: [0; 5],
                cols: [0; 5],
                bingo: false,
            });
            spaces = HashMap::new();
            r = 0;
        }
    }
    while let Some(num) = nums.next() {
        let result = boards
            .iter_mut()
            .map(|board| board.mark(num))
            .find(|(bingo, _)| *bingo == true);
        match result {
            Some((_, score)) => {
                return num * score;
            }
            None => {}
        }
    }
    0
}

fn calc2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut nums = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap());
    let mut boards = vec![];
    let mut spaces = HashMap::new();
    let mut r = 0;
    while let Some(line) = lines.next() {
        if line == "" {
            continue;
        }
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .enumerate()
            .for_each(|(c, x)| {
                spaces.insert(x, (c, r));
            });
        r += 1;
        if r == 5 {
            boards.push(Board {
                spaces,
                rows: [0; 5],
                cols: [0; 5],
                bingo: false,
            });
            spaces = HashMap::new();
            r = 0;
        }
    }
    let mut worst = 0;
    while let Some(num) = nums.next() {
        let score: isize =
            boards
                .iter_mut()
                .map(|board| board.mark(num))
                .fold(-1, |mut acc, (bingo, score)| {
                    if bingo {
                        acc = score as isize;
                    }
                    acc
                });
        if score >= 0 {
            worst = score as usize * num;
        }
    }
    worst
}
