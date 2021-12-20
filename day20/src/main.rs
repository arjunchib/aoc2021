fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 35);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 3351);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

struct Picture {
    enhancer: Vec<u8>,
    pixels: Vec<u8>,
    width: isize,
    height: isize,
    background: u8,
}

impl Picture {
    fn from(input: &str) -> Self {
        let mut lines = input.lines().peekable();
        let enhancer: Vec<u8> = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!(),
            })
            .collect();
        lines.next();
        let width = lines.peek().unwrap().len() as isize;
        let pixels: Vec<u8> = lines
            .into_iter()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!(),
                })
            })
            .collect();
        let height = pixels.len() as isize / width;
        Picture {
            enhancer,
            pixels,
            width,
            height,
            background: 0,
        }
    }

    fn enhance(&mut self) {
        let mut pixels = vec![];
        for y in -1..=self.height {
            for x in -1..=self.width {
                pixels.push(self.enhancement(x, y));
            }
        }
        self.pixels = pixels;
        self.width += 2;
        self.height += 2;
        if self.enhancer[0] == 1 {
            self.background = match self.background {
                0 => 1,
                1 => 0,
                _ => panic!(),
            }
        }
    }

    fn enhancement(&self, x: isize, y: isize) -> u8 {
        let index = self.pixel(x - 1, y - 1) << 8
            | self.pixel(x, y - 1) << 7
            | self.pixel(x + 1, y - 1) << 6
            | self.pixel(x - 1, y) << 5
            | self.pixel(x, y) << 4
            | self.pixel(x + 1, y) << 3
            | self.pixel(x - 1, y + 1) << 2
            | self.pixel(x, y + 1) << 1
            | self.pixel(x + 1, y + 1);
        self.enhancer[index]
    }

    fn pixel(&self, x: isize, y: isize) -> usize {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return self.background as usize;
        }
        let index = (x + y * self.width) as usize;
        self.pixels[index] as usize
    }

    fn total_lit(&self) -> usize {
        self.pixels.iter().filter(|p| **p > 0).count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!();
        for y in 0..self.height {
            let mut row = String::new();
            for x in 0..self.width {
                let c = match self.pixel(x, y) {
                    0 => '.',
                    1 => '#',
                    _ => panic!(),
                };
                row.push(c);
            }
            println!("{}", row)
        }
        println!();
    }
}

fn calc1(input: &str) -> usize {
    let mut p = Picture::from(input);
    p.enhance();
    p.enhance();
    p.total_lit()
}

fn calc2(input: &str) -> usize {
    let mut p = Picture::from(input);
    for _ in 0..50 {
        p.enhance();
    }
    p.total_lit()
}
