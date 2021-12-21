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

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
}

#[derive(Clone)]
struct Beacon {
    x: isize,
    y: isize,
    z: isize,
    neighbors: Vec<f32>,
}

impl Scanner {
    fn from(input: &[String]) -> Self {
        let mut lines = input.iter();
        let first = lines.next().unwrap();
        let id: usize = first[12..first.len() - 4].parse().unwrap();
        let mut beacons: Vec<Beacon> = lines
            .map(|line| {
                let loc: Vec<isize> = line.split(',').map(|x| x.parse().unwrap()).collect();
                Beacon {
                    x: loc[0],
                    y: loc[1],
                    z: loc[2],
                    neighbors: vec![],
                }
            })
            .collect();
        let original_beacons = beacons.clone();
        for b1 in &mut beacons {
            for b2 in &original_beacons {
                b1.add_neighbor(b1.dist(b2));
            }
        }
        Scanner { id, beacons }
    }

    fn common_beacons(&self, other: &Self) -> Vec<(Beacon, Beacon)> {
        let mut beacons = vec![];
        for b1 in &self.beacons {
            for b2 in &other.beacons {
                if b1.common_beacons(b2) >= 12 {
                    beacons.push((b1.clone(), b2.clone()));
                }
            }
        }
        beacons
    }
}

impl Beacon {
    fn dist(&self, other: &Self) -> f32 {
        let x = (self.x - other.x).checked_pow(2).unwrap();
        let y = (self.y - other.y).checked_pow(2).unwrap();
        let z = (self.z - other.z).checked_pow(2).unwrap();
        ((x + y + z) as f32).sqrt()
    }

    fn add_neighbor(&mut self, dist: f32) {
        self.neighbors.push(dist)
    }

    fn common_beacons(&self, other: &Self) -> usize {
        let mut count = 0;
        let error_margin = f32::EPSILON;
        for dist1 in &self.neighbors {
            for dist2 in &other.neighbors {
                if (*dist1 - *dist2).abs() < error_margin {
                    count += 1
                }
            }
        }
        count
    }

    fn all_orientations(&self) -> Vec<(isize, isize, isize)> {
        let mut orientations = vec![];
        for dir in [-1, 1] {
            for dim in ["x", "y", "z"] {
                let rot = match dim {
                    "x" => ["y", "z"],
                    "y" => ["x", "z"],
                    "z" => ["x", "y"],
                    _ => panic!(),
                };
                let a = self[rot[0]];
                let b = self[rot[1]];
                orientations.push((dir * self[dim], a, -b));
                orientations.push((dir * self[dim], -b, -a));
                orientations.push((dir * self[dim], -a, b));
                orientations.push((dir * self[dim], b, a));
            }
        }
        orientations
    }
}
impl fmt::Debug for Beacon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl std::ops::Index<&'_ str> for Beacon {
    type Output = isize;
    fn index(&self, s: &str) -> &isize {
        match s {
            "x" => &self.x,
            "y" => &self.y,
            "z" => &self.z,
            _ => panic!("unknown field: {}", s),
        }
    }
}

fn possible_locations(beacons: &Vec<(Beacon, Beacon)>) -> Vec<(isize, isize, isize)> {
    beacons
        .iter()
        .map(|(b1, b2)| {
            HashSet::from_iter(
                b2.all_orientations()
                    .iter()
                    .map(|b| (b1.x + b.0, b1.y + b.1, b1.z + b.2)),
            )
        })
        .reduce(|a, b| {
            a.intersection(&b)
                .copied()
                .collect::<HashSet<(isize, isize, isize)>>()
        })
        .unwrap()
        .drain()
        .collect()
}

fn calc1(input: &str) -> usize {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let scanners: Vec<Scanner> = lines
        .split(|x| x.is_empty())
        .map(|x| Scanner::from(x))
        .collect();
    let cbs = scanners[0].common_beacons(&scanners[1]);
    println!("{:#?}", possible_locations(&cbs));
    0
}

// fn calc2(input: &str) -> usize {
//     0
// }
