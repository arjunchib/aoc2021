use std::collections::HashSet;

fn main() {
    let baby_test = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    // Part 1
    assert_eq!(calc1("on x=10..12,y=10..12,z=10..12"), 27);
    assert_eq!(calc1(include_str!("test1.in")), 590784);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(baby_test), 39);
    assert_eq!(calc2(include_str!("test2.in")), 2758514936282235);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

struct Cube1 {
    grid: HashSet<(isize, isize, isize)>,
}

impl Cube1 {
    fn new() -> Self {
        Self {
            grid: HashSet::new(),
        }
    }

    fn step(&mut self, input: &str) {
        // on x=10..12,y=10..12,z=10..12
        let (pos, input) = input.split_once(' ').unwrap();
        let dims: Vec<(isize, isize)> = input
            .split(',')
            .map(|s| {
                let s = &s[2..];
                let (start, end) = s.split_once("..").unwrap();
                let start: isize = start.parse().unwrap();
                let end: isize = end.parse().unwrap();
                (start.min(end), end.max(start))
            })
            .collect();
        for bound in dims.iter().flat_map(|x| [x.0, x.1]) {
            if bound > 50 || bound < -50 {
                return;
            }
        }
        for x in dims[0].0..=dims[0].1 {
            for y in dims[1].0..=dims[1].1 {
                for z in dims[2].0..=dims[2].1 {
                    match pos {
                        "on" => {
                            self.grid.insert((x, y, z));
                        }
                        "off" => {
                            self.grid.remove(&(x, y, z));
                        }
                        _ => panic!(),
                    }
                }
            }
        }
    }

    fn total_on(&self) -> usize {
        self.grid.len()
    }
}

fn calc1(input: &str) -> usize {
    let mut c = Cube1::new();
    for line in input.lines() {
        c.step(line);
    }
    c.total_on()
}

struct Reactor {
    blocks: Vec<Block>,
}

#[derive(Debug)]
struct Block {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Reactor {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn parse_step(input: &str) -> (bool, Block) {
        let (pos, input) = input.split_once(' ').unwrap();
        let dims: Vec<(isize, isize)> = input
            .split(',')
            .map(|s| {
                let s = &s[2..];
                let (start, end) = s.split_once("..").unwrap();
                let start: isize = start.parse().unwrap();
                let end: isize = end.parse().unwrap();
                (start.min(end), end.max(start))
            })
            .collect();
        let pos = match pos {
            "on" => true,
            "off" => false,
            _ => panic!(),
        };
        let block = Block {
            x: (dims[0].0, dims[0].1),
            y: (dims[1].0, dims[1].1),
            z: (dims[2].0, dims[2].1),
        };
        (pos, block)
    }

    fn step(&mut self, input: &str) {
        let (pos, block) = Reactor::parse_step(input);
        let mut new_blocks = vec![];

        // The following method is the same as drain_filter, which is only
        // available in nightly rust
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
        let mut i = 0;
        while i < self.blocks.len() {
            if block.intersect(&self.blocks[i]) {
                let other = self.blocks.remove(i);
                new_blocks.append(&mut other.split(&block));
            } else {
                i += 1;
            }
        }

        self.blocks.append(&mut new_blocks);
        if pos {
            self.blocks.push(block)
        }

        // println!(
        //     "{:?}",
        //     self.blocks.iter().map(|b| b.volume()).sum::<usize>()
        // );
        // println!("{:?}", self.blocks)
    }

    fn total_on(&self) -> usize {
        self.blocks.iter().map(|b| b.volume()).sum()
    }
}

impl Block {
    fn intersect_dim(start1: isize, end1: isize, start2: isize, end2: isize) -> bool {
        if start1 < start2 {
            end1 >= start2
        } else if start2 < start1 {
            end2 >= start1
        } else {
            true
        }
    }

    fn intersect(&self, other: &Self) -> bool {
        Self::intersect_dim(self.x.0, self.x.1, other.x.0, other.x.1)
            && Self::intersect_dim(self.y.0, self.y.1, other.y.0, other.y.1)
            && Self::intersect_dim(self.z.0, self.z.1, other.z.0, other.z.1)
    }

    fn volume(&self) -> usize {
        let x = isize::abs(self.x.1 - self.x.0) as usize + 1;
        let y = isize::abs(self.y.1 - self.y.0) as usize + 1;
        let z = isize::abs(self.z.1 - self.z.0) as usize + 1;
        x * y * z
    }

    fn split(&self, other: &Self) -> Vec<Self> {
        let mut blocks = vec![];
        let left_x = (self.x.0, other.x.0 - 1);
        let right_x = (other.x.1 + 1, self.x.1);
        let mid_x = (self.x.0.max(other.x.0), self.x.1.min(other.x.1));
        if left_x.0 <= left_x.1 {
            blocks.push(Block {
                x: left_x,
                y: self.y,
                z: self.z,
            })
        }
        if right_x.0 <= right_x.1 {
            blocks.push(Block {
                x: right_x,
                y: self.y,
                z: self.z,
            });
        }
        let left_y = (self.y.0, other.y.0 - 1);
        let right_y = (other.y.1 + 1, self.y.1);
        let mid_y = (self.y.0.max(other.y.0), self.y.1.min(other.y.1));
        if left_y.0 <= left_y.1 {
            blocks.push(Block {
                x: mid_x,
                y: left_y,
                z: self.z,
            })
        }
        if right_y.0 <= right_y.1 {
            blocks.push(Block {
                x: mid_x,
                y: right_y,
                z: self.z,
            });
        }
        let left_z = (self.z.0, other.z.0 - 1);
        let right_z = (other.z.1 + 1, self.z.1);
        if left_z.0 <= left_z.1 {
            blocks.push(Block {
                x: mid_x,
                y: mid_y,
                z: left_z,
            })
        }
        if right_z.0 <= right_z.1 {
            blocks.push(Block {
                x: mid_x,
                y: mid_y,
                z: right_z,
            });
        }
        // println!(
        //     "
        // {:?} {:?}
        // {:?}",
        //     self, other, blocks
        // );
        blocks
    }
}

fn calc2(input: &str) -> usize {
    let mut r = Reactor::new();
    for line in input.lines() {
        r.step(line);
    }
    r.total_on()
}
