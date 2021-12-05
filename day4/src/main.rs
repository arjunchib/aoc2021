use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 4512);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 1924);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(PartialEq, Eq)]
struct Board {
    spaces: HashMap<usize, (usize, usize)>,
    rows: [usize; 5],
    cols: [usize; 5],
    bingo: bool,
}

impl Board {
    fn from(input: String) -> Board {
        let spaces = HashMap::from_iter(
            input
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .enumerate()
                .map(|(i, x)| (x, (i / 5, i % 5))),
        );
        Board {
            spaces,
            rows: [0; 5],
            cols: [0; 5],
            bingo: false,
        }
    }

    fn mark(&mut self, num: usize) {
        if !self.spaces.contains_key(&num) || self.bingo {
            return;
        }
        let (c, r) = self.spaces.remove(&num).unwrap();
        self.rows[r] += 1;
        self.cols[c] += 1;
        self.bingo = self.rows[r] == 5 || self.cols[c] == 5;
    }

    fn score(&self) -> usize {
        self.spaces.keys().sum()
    }
}

struct Game {
    boards: Vec<Board>,
    nums: Vec<usize>,
}

impl Game {
    fn from(input: &str) -> Game {
        let mut lines = input.lines();
        let nums = lines
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let boards = lines
            .skip(1)
            .collect::<Vec<&str>>()
            .split(|x| *x == "")
            .map(|x| x.join(" "))
            .map(|x| Board::from(x))
            .collect();
        Game { nums, boards }
    }

    fn find_best(&mut self) -> Option<usize> {
        for n in self.nums.iter() {
            self.boards.iter_mut().for_each(|b| b.mark(*n));
            match self.boards.iter().find(|b| b.bingo) {
                Some(b) => return Some(b.score() * n),
                None => {}
            }
        }
        None
    }

    fn find_worst(&mut self) -> Option<usize> {
        let mut worst: Option<usize> = None;
        for n in self.nums.iter() {
            self.boards.retain(|b| !b.bingo);
            self.boards.iter_mut().for_each(|b| b.mark(*n));
            match self.boards.iter().find(|b| b.bingo) {
                Some(b) => worst = Some(b.score() * n),
                None => {}
            }
        }
        worst
    }
}

fn calc1(input: &str) -> usize {
    let mut game = Game::from(input);
    game.find_best().unwrap()
}

fn calc2(input: &str) -> usize {
    let mut game = Game::from(input);
    game.find_worst().unwrap()
}
