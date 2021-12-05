use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 5);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 12);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point(usize, usize);

impl Point {
    fn from(input: &str) -> Point {
        let (x, y) = input.split_once(",").unwrap();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        Point(x, y)
    }
}

struct Vents {
    hitmap: HashMap<Point, usize>,
}

impl Vents {
    fn hit(&mut self, p: Point) {
        let count = match self.hitmap.get(&p) {
            Some(n) => *n + 1,
            None => 1,
        };
        self.hitmap.insert(p, count);
    }

    fn hit_range_perp(&mut self, p1: Point, p2: Point) {
        let Point(x1, y1) = p1;
        let Point(x2, y2) = p2;
        if x1 == x2 {
            let rng = match y1 < y2 {
                true => y1..=y2,
                false => y2..=y1,
            };
            for y in rng {
                self.hit(Point(x1, y));
            }
        } else if y1 == y2 {
            let rng = match x1 < x2 {
                true => x1..=x2,
                false => x2..=x1,
            };
            for x in rng {
                self.hit(Point(x, y1));
            }
        }
    }

    fn hit_range(&mut self, p1: Point, p2: Point) {
        let Point(x1, y1) = p1;
        let Point(x2, y2) = p2;
        if x1 == x2 || y1 == y2 {
            return self.hit_range_perp(p1, p2);
        }
        let mut curr_x = x1;
        let mut curr_y = y1;
        while curr_x != x2 && curr_y != y2 {
            self.hit(Point(curr_x, curr_y));
            match x1 < x2 {
                true => curr_x += 1,
                false => curr_x -= 1,
            };
            match y1 < y2 {
                true => curr_y += 1,
                false => curr_y -= 1,
            };
        }
        self.hit(Point(curr_x, curr_y));
    }

    fn count_danger(&self) -> usize {
        self.hitmap.values().filter(|c| **c >= 2).count()
    }

    fn new() -> Vents {
        Vents {
            hitmap: HashMap::new(),
        }
    }
}

fn input_iter(input: &'static str) -> impl Iterator<Item = (Point, Point)> {
    input.lines().map(|line| {
        let (p1, p2) = line.split_once(" -> ").unwrap();
        (Point::from(p1), Point::from(p2))
    })
}

fn calc1(input: &'static str) -> usize {
    let mut v = Vents::new();
    input_iter(input).for_each(|(p1, p2)| v.hit_range_perp(p1, p2));
    v.count_danger()
}

fn calc2(input: &'static str) -> usize {
    let mut v = Vents::new();
    input_iter(input).for_each(|(p1, p2)| v.hit_range(p1, p2));
    v.count_danger()
}
