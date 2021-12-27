use std::fmt;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 58);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 3993);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug, PartialEq)]
enum Space {
    East,
    South,
    Empty,
}

struct Floor {
    spaces: Vec<Space>,
    width: usize,
    height: usize,
}

impl Floor {
    fn step(&mut self) -> bool {
        let moved_x = self.step_east();
        let moved_y = self.step_south();
        moved_x || moved_y
    }

    fn step_east(&mut self) -> bool {
        let mut swap_list = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let i = x + y * self.width;
                let j = ((x + 1) % self.width) + y * self.width;
                if self.spaces[i] == Space::East && self.spaces[j] == Space::Empty {
                    swap_list.push((i, j));
                }
            }
        }
        for (i, j) in &swap_list {
            self.spaces.swap(*i, *j);
        }
        !swap_list.is_empty()
    }

    fn step_south(&mut self) -> bool {
        let mut swap_list = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let i = x + y * self.width;
                let j = x + (y + 1) % self.height * self.width;
                if self.spaces[i] == Space::South && self.spaces[j] == Space::Empty {
                    swap_list.push((i, j));
                }
            }
        }
        for (i, j) in &swap_list {
            self.spaces.swap(*i, *j);
        }
        !swap_list.is_empty()
    }
}

impl fmt::Display for Floor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        let mut i = 0;
        for space in &self.spaces {
            let c = match space {
                Space::East => '>',
                Space::South => 'v',
                Space::Empty => '.',
            };
            output.push(c);
            if i == self.width - 1 {
                output += "\n";
                i = 0;
            } else {
                i += 1;
            }
        }
        write!(f, "{}", output)
    }
}

impl From<&str> for Floor {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let width = lines.peek().unwrap().len();
        let spaces: Vec<Space> = lines
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '>' => Space::East,
                    'v' => Space::South,
                    '.' => Space::Empty,
                    _ => panic!(),
                })
            })
            .collect();
        let height = spaces.len() / width;
        Self {
            spaces,
            width,
            height,
        }
    }
}

fn calc1(input: &str) -> usize {
    let mut f = Floor::from(input);
    let mut i = 1;
    while f.step() {
        i += 1;
    }
    println!("{}", f);
    i
}

fn calc2(input: &str) -> usize {
    0
}
