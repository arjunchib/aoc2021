fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 1656);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 195);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

struct Grid {
    octopi: Vec<u32>,
    rows: usize,
    cols: usize,
    flashes: usize,
}

impl Grid {
    fn step(&mut self) {
        for x in 0..self.cols {
            for y in 0..self.rows {
                self.flashes += self.flash(x as isize, y as isize);
            }
        }
        self.reset_flash();
    }

    fn reset_flash(&mut self) {
        for x in 0..self.cols {
            for y in 0..self.rows {
                if self.octopi[x + y * self.cols] > 9 {
                    self.octopi[x + y * self.cols] = 0;
                }
            }
        }
    }

    fn is_sync(&self) -> bool {
        self.octopi.iter().all(|x| *x == 0)
    }

    fn energy(&mut self, x: usize, y: usize) -> Option<&mut u32> {
        self.octopi.get_mut(x as usize + y as usize * self.cols)
    }

    fn flash(&mut self, x: isize, y: isize) -> usize {
        if x < 0 || x >= self.cols as isize || y < 0 || y >= self.rows as isize {
            return 0;
        }
        let energy = self.energy(x as usize, y as usize);
        if energy == None {
            return 0;
        }
        let lvl = *energy.unwrap();
        self.octopi[x as usize + y as usize * self.cols] = lvl + 1;
        let mut flashes = 0;
        // println!("{}", lvl);
        if lvl == 9 {
            flashes += 1;
            for x_off in -1..=1 {
                for y_off in -1..=1 {
                    flashes += self.flash(x + x_off, y + y_off);
                }
            }
        }
        flashes
    }
}

fn calc1(input: &str) -> usize {
    let octopi: Vec<u32> = input
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let mut grid = Grid {
        octopi,
        rows: 10,
        cols: 10,
        flashes: 0,
    };
    for _ in 0..100 {
        grid.step();
    }
    grid.flashes
}

fn calc2(input: &str) -> usize {
    let octopi: Vec<u32> = input
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let mut grid = Grid {
        octopi,
        rows: 10,
        cols: 10,
        flashes: 0,
    };
    let mut i = 0;
    while !grid.is_sync() {
        i += 1;
        grid.step();
    }
    i
}
