use ndarray::{Array, Array2};

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 12521);
    // println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    // assert_eq!(calc2(include_str!("test.in")), 3993);
    // println!("part 2: {}", calc2(include_str!("real.in")));
}

struct Solver {
    burrow: Array2<char>,
}

impl Solver {
    fn from(input: &str) -> Self {
        let burrow = Array::from_iter(input.lines().flat_map(|x| x.chars()))
            .into_shape((5, 13))
            .unwrap();
        Self { burrow }
    }

    fn moves(burrow: &Array2<char>) -> Vec<Array2<char>> {
        let mut boards = vec![];
        for ((x, y), frog) in burrow
            .indexed_iter()
            .filter(|(_, frog)| !matches!(frog, '.' | '#' | ' '))
        {
            let moves = vec![(x, y + 1), (x, y - 1), (x - 1, y), (x + 1, y)];
            for m in moves {
                match burrow.get(m) {
                    // Some(n) if n == '.' => 
                }
            }
        }
        boards
    }

    fn step(&self, burrow: &Array2<char>) -> usize {
        Solver::moves(burrow);
        0
    }

    fn solve(&self) -> usize {
        self.step(&self.burrow)
    }

    fn print(&self) {
        for row in self.burrow.rows() {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }
}

fn calc1(input: &str) -> usize {
    let s = Solver::from(input);
    s.solve();
    0
}

// fn calc2(input: &str) -> usize {
//     0
// }
