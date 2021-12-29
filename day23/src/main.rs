mod amphipod;
use amphipod::Burrow;
use std::collections::HashMap;

fn main() {
    // Part 1
    // assert_eq!(calc1(include_str!("test1.in")), 12521);
    // println!("part 1: {}", calc1(include_str!("real1.in")));

    // Part 2
    // assert_eq!(calc2(include_str!("test2.in")), 44169);
    println!("part 2: {}", calc2(include_str!("real2.in")));
}

struct Solver {
    cache: HashMap<Burrow, usize>,
}

impl Solver {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn solve(&mut self, input: &str) -> usize {
        let b = Burrow::new(input);
        self.memo_step(&b)
    }

    fn memo_step(&mut self, burrow: &Burrow) -> usize {
        let mut burrow = burrow.clone();
        burrow.sort();
        if let Some(item) = self.cache.get(&burrow) {
            *item
        } else {
            let result = self.step(burrow.clone());
            self.cache.insert(burrow, result);
            result
        }
    }

    fn step(&mut self, burrow: Burrow) -> usize {
        if burrow.goal() {
            return 0;
        }
        burrow
            .next_burrows()
            .iter()
            // .inspect(|x| println!("{}", x.0))
            .map(|(b, e)| e.saturating_add(self.memo_step(b)))
            .min()
            .unwrap_or(usize::MAX)
    }
}

fn calc1(input: &str) -> usize {
    let mut s = Solver::new();
    s.solve(input)
}

fn calc2(input: &str) -> usize {
    calc1(input)
}
