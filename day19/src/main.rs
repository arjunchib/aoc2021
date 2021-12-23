use std::collections::HashSet;
fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 79);
    // println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    // assert_eq!(calc2(include_str!("test.in")), 3993);
    // println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Beacon>,
    location: Option<Beacon>,
}

#[derive(Debug, Clone)]
struct Beacon(isize, isize, isize);

impl Scanner {
    fn from(input: &[&str]) -> Self {
        let beacons: Vec<Beacon> = input
            .iter()
            .skip(1)
            .map(|line| {
                let loc: Vec<isize> = line.split(',').map(|x| x.parse().unwrap()).collect();
                Beacon(loc[0], loc[1], loc[2])
            })
            .collect();
        Self {
            beacons,
            location: None,
        }
    }

    fn change_basis(&mut self, other: Self) {}

    fn match_beacons(&self, other: Self) -> Option<Vec<(Beacon, Beacon)>> {
        for b1 in &self.beacons {
            for b2 in &other.beacons {}
        }
        None
    }
}

impl Beacon {
    fn dist(&self, other: &Self) -> f32 {
        let a = (self.0 - other.0).checked_pow(2).unwrap();
        let b = (self.1 - other.1).checked_pow(2).unwrap();
        let c = (self.2 - other.2).checked_pow(2).unwrap();
        ((a + b + c) as f32).sqrt()
    }
}

fn calc1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut s: Vec<Scanner> = lines
        .split(|x| x.is_empty())
        .map(|x| Scanner::from(x))
        .collect();
    s[0].location = Some(Beacon(0, 0, 0));
    println!("{:?}", s[0]);
    0
}

// fn calc2(input: &str) -> usize {
//     0
// }
