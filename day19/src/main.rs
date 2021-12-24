use std::collections::HashSet;
use std::fmt;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 79);
    // println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    // assert_eq!(calc2(include_str!("test.in")), 3993);
    // println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
    location: Option<Beacon>,
}

#[derive(Clone, PartialEq)]
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

    // Changes basis to the basis of other
    fn normalize(&mut self, other: &Self) -> bool {
        let matches = self.match_beacons(other);
        if matches.len() < 12 {
            return false;
        }
        for basis in Basis::all() {
            let (b1, b2) = &matches[0];
            let loc = b2.clone() - b1.change_basis(&basis);
            if matches
                .iter()
                .map(|(b1, b2)| b2.clone() - b1.change_basis(&basis))
                .all(|x| x == loc)
            {
                self.location = Some(loc.clone());
                self.beacons = self
                    .beacons
                    .iter()
                    .map(|x| x.change_basis(&basis) + loc.clone())
                    .collect();
                break;
            }
        }
        true
    }

    fn match_beacons(&self, other: &Self) -> Vec<(Beacon, Beacon)> {
        let mut matches = vec![];
        let other_pairs = other.signature_pairs();
        for (b1, s1) in self.signature_pairs() {
            for (b2, s2) in &other_pairs {
                if Beacon::signature_match(&s1, s2) {
                    matches.push((b1.clone(), b2.clone()));
                }
            }
        }
        matches
    }

    fn signature_pairs(&self) -> Vec<(Beacon, Vec<f32>)> {
        self.beacons
            .iter()
            .map(|b| (b.clone(), b.signature(&self.beacons)))
            .collect()
    }
}

impl Beacon {
    fn dist(&self, other: &Self) -> f32 {
        let a = (self.0 - other.0).checked_pow(2).unwrap();
        let b = (self.1 - other.1).checked_pow(2).unwrap();
        let c = (self.2 - other.2).checked_pow(2).unwrap();
        ((a + b + c) as f32).sqrt()
    }

    fn signature(&self, neighbors: &Vec<Beacon>) -> Vec<f32> {
        neighbors.iter().map(|other| self.dist(other)).collect()
    }

    fn signature_match(a: &Vec<f32>, b: &Vec<f32>) -> bool {
        let mut count = 0;
        for s1 in a {
            for s2 in b {
                if (s1 - s2).abs() < f32::EPSILON {
                    count += 1
                }
            }
        }
        count >= 12
    }

    // Transforms a beacon to the standard basis
    fn change_basis(&self, basis: &Basis) -> Self {
        let a0 = self.0 * (basis.0).0 + self.1 * (basis.0).1 + self.2 * (basis.0).2;
        let a1 = self.0 * (basis.1).0 + self.1 * (basis.1).1 + self.2 * (basis.1).2;
        let a2 = self.0 * (basis.2).0 + self.1 * (basis.2).1 + self.2 * (basis.2).2;
        Self(a0, a1, a2)
    }
}

impl fmt::Debug for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl std::ops::Sub for Beacon {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Add for Beacon {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Debug)]
struct Basis(
    (isize, isize, isize),
    (isize, isize, isize),
    (isize, isize, isize),
);

impl Basis {
    fn all() -> Vec<Basis> {
        let x = (1, 0, 0);
        let y = (0, 1, 0);
        let z = (0, 0, 1);
        let nx = (-1, 0, 0);
        let ny = (0, -1, 0);
        let nz = (0, 0, -1);
        vec![
            Basis(x, y, z),
            Basis(x, ny, nz),
            Basis(x, z, ny),
            Basis(x, nz, y),
            Basis(nx, y, nz),
            Basis(nx, ny, z),
            Basis(nx, z, y),
            Basis(nx, nz, ny),
            Basis(y, x, nz),
            Basis(y, nx, z),
            Basis(y, z, x),
            Basis(y, nz, nx),
            Basis(ny, x, z),
            Basis(ny, nx, nz),
            Basis(ny, z, nx),
            Basis(ny, nz, x),
            Basis(z, x, y),
            Basis(z, nx, ny),
            Basis(z, y, nx),
            Basis(z, ny, x),
            Basis(nz, x, ny),
            Basis(nz, nx, y),
            Basis(nz, y, x),
            Basis(nz, ny, nx),
        ]
    }
}

fn calc1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut s: Vec<Scanner> = lines
        .split(|x| x.is_empty())
        .map(|x| Scanner::from(x))
        .collect();
    s[0].location = Some(Beacon(0, 0, 0));
    // let root = s[0].clone();
    // s[1].normalize(&root);
    // println!("{:#?}", s[1]);
    // let s1 = s[1].clone();
    // s[4].normalize(&s1);
    // println!("{:#?}", s[4]);
    // let s4 = s[4].clone();
    // s[2].normalize(&s4);
    // println!("{:#?}", s[2]);
    // let s2 = s[2].clone();
    // s[3].normalize(&s1);
    // println!("{:#?}", s[3]);
    0
}

// fn calc2(input: &str) -> usize {
//     0
// }
