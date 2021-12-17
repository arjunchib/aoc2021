use std::cmp::Ordering;
use std::ops::RangeInclusive;

fn main() {
    // Part 1
    assert_eq!(calc1((20..=30, -10..=-5)), 45);
    println!("part 1: {}", calc1((179..=201, -109..=-63)));

    // Part 2
    assert_eq!(calc2((20..=30, -10..=-5)), 112);
    println!("part 2: {}", calc2((179..=201, -109..=-63)));
}

struct Probe {
    x: isize,
    y: isize,
    vel_x: isize,
    vel_y: isize,
    target_x: RangeInclusive<isize>,
    target_y: RangeInclusive<isize>,
}

impl Probe {
    fn new(vel: (isize, isize), target: (RangeInclusive<isize>, RangeInclusive<isize>)) -> Self {
        Probe {
            x: 0,
            y: 0,
            vel_x: vel.0,
            vel_y: vel.1,
            target_x: target.0,
            target_y: target.1,
        }
    }

    fn step_x(&mut self) -> bool {
        self.x += self.vel_x;
        self.vel_x += match self.vel_x.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };
        self.target_x.contains(&self.x)
    }

    fn step_y(&mut self) -> bool {
        self.y += self.vel_y;
        self.vel_y -= 1;
        self.target_y.contains(&self.y)
    }

    fn launch_x(&mut self) -> bool {
        let mut hit = false;
        while !hit && self.vel_x != 0 && self.x < *self.target_x.end() {
            hit = self.step_x();
        }
        hit
    }

    fn launch_y(&mut self) -> bool {
        let mut hit = false;
        while !hit && self.y > *self.target_y.start() {
            hit = self.step_y();
        }
        hit
    }

    fn launch(&mut self) -> bool {
        let mut hit = false;
        while !hit && self.x < *self.target_x.end() && self.y > *self.target_y.start() {
            let hit_x = self.step_x();
            let hit_y = self.step_y();
            hit = hit_x && hit_y;
        }
        hit
    }

    fn max_y(&mut self) -> (bool, isize) {
        let mut h = 0;
        let mut hit = false;
        while !hit && self.y > *self.target_y.end() {
            hit = self.step_y();
            h = h.max(self.y);
        }
        (hit, h)
    }
}

fn calc1(target: (RangeInclusive<isize>, RangeInclusive<isize>)) -> isize {
    let mut y = 0;
    let mut max_h = 0;
    loop {
        let mut p = Probe::new((0, y), target.clone());
        let (hit, h) = p.max_y();
        if hit {
            max_h = max_h.max(h);
        }
        if y > 1000 {
            break max_h;
        }
        y += 1;
    }
}

fn calc2(target: (RangeInclusive<isize>, RangeInclusive<isize>)) -> isize {
    let mut x_vels = vec![];
    let mut y_vels = vec![];
    for i in -1000..1000 {
        let mut p = Probe::new((i, i), target.clone());
        if i > 0 && p.launch_x() {
            x_vels.push(i);
        };
        if p.launch_y() {
            y_vels.push(i);
        };
    }
    let mut count = 0;
    for x in &x_vels {
        for y in &y_vels {
            let mut p = Probe::new((*x, *y), target.clone());
            if p.launch() {
                count += 1
            }
        }
    }
    count
}
