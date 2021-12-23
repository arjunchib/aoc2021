use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};

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
    pos: (isize, isize, isize),
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
        let pos = (0, 0, 0);
        Scanner { id, beacons, pos }
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

    fn relative_location(&self, other: &Self) -> Option<(isize, isize, isize)> {
        let locs: Option<HashSet<(isize, isize, isize)>> = self
            .common_beacons(other)
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
            });
        if let Some(mut n) = locs {
            if let Some(loc) = n.drain().last() {
                return Some((self.pos.0 + loc.0, self.pos.1 + loc.1, self.pos.2 + loc.2));
            }
        }
        None
    }

    fn change_basis(&mut self, loc: (usize, usize, usize)) {}
}

impl Hash for Scanner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Scanner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Scanner {}

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
        // let x = self.x;
        // let y = self.y;
        // let z = self.z;
        // vec![
        //     (x, y, z),
        //     (x, z, -y),
        //     (x, -y, -z),
        //     (x, -z, y),
        //     (-x, z, -y),
        //     (-x, -y, -z),
        //     (-x, -z, y),
        //     (-x, y, z),
        //     (y, z, -x),
        //     (y, -x, -z),
        //     (y, -z, x),
        //     (y, x, z),
        //     (-y, -z, x),
        //     (-y, x, z),
        //     (-y, z, -x),
        //     (-y, -x, -z),
        //     (z, -x, -y),
        //     (z, -y, x),
        //     (z, x, y),
        //     (z, y, -x),
        //     (-z, x, y),
        //     (-z, y, -x),
        //     (-z, -x, -y),
        //     (-z, -y, x),
        // ]
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

fn locate_scanners(scanners: &mut Vec<&mut Scanner>) {
    let mut unvisited: HashSet<usize> = (1..scanners.len()).collect();
    let mut stack = vec![0];
    let mut map = HashMap::new();
    while !unvisited.is_empty() {
        let i = stack.pop().unwrap();
        for j in unvisited.clone() {
            let s1 = &scanners[i];
            let s2 = &scanners[j];
            match s1.relative_location(s2) {
                None => {}
                Some(pos) => {
                    scanners[j].pos = pos;
                    unvisited.remove(&j);
                    stack.push(j);
                    map.insert(j, i);
                }
            };
        }
    }
    while map.values().any(|x| *x != 0) {
        for (i, j) in map.clone().iter().filter(|(_, v)| **v != 0) {
            if map[j] == 0 {
                let s1 = &scanners[*i];
                let s2 = &scanners[*j];
                scanners[*i].pos = (
                    s1.pos.0 + s2.pos.0,
                    s1.pos.1 + s2.pos.1,
                    s1.pos.2 + s2.pos.2,
                );
                map.insert(*i, 0);
            }
        }
    }
}

fn calc1(input: &str) -> usize {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut s: Vec<Scanner> = lines
        .split(|x| x.is_empty())
        .map(|x| Scanner::from(x))
        .collect();
    let rel_loc = s[0].relative_location(&s[1]);
    println!("{:#?}", rel_loc);
    // s[1].change_basis(rel_loc);
    println!("{:#?}", s[1].relative_location(&s[4]));
    // locate_scanners(&mut s.iter_mut().collect());
    // println!("{:#?}", s);
    0
}

// fn calc2(input: &str) -> usize {
//     0
// }
