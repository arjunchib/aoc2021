use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

fn main() {
    // Part 1
    // assert_eq!(calc1("AABBCCDD"), 0);
    // assert_eq!(calc1("BACDBCDA"), 12521);
    println!("part 1: {}", calc1("ACDDCBAB"));

    // Part 2
    // assert_eq!(calc2(include_str!("test.in")), 3993);
    // println!("part 2: {}", calc2(include_str!("real.in")));
}

/*
             1
   01234567890
  #############
0 #...........#
1 ###A#B#C#D###
2   #A#B#C#D#
    #########
*/

#[derive(Clone, PartialEq, Eq, Hash, Debug, PartialOrd)]
enum Color {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Clone, Eq, Debug)]
struct Amphipod {
    color: Color,
    space: (isize, isize),
    energy: usize,
}

impl Amphipod {
    fn move_to(&mut self, space: (isize, isize)) {
        let dist = (self.space.1 - space.1).abs() + (self.space.0 - space.0).abs();
        let level = match self.color {
            Color::Amber => 1,
            Color::Bronze => 10,
            Color::Copper => 100,
            Color::Desert => 1000,
        };
        self.energy += (dist * level) as usize;
        self.space = space;
    }

    fn h(&self) -> usize {
        let goal_x = match self.color {
            Color::Amber => 2,
            Color::Bronze => 4,
            Color::Copper => 6,
            Color::Desert => 8,
        };
        let goal = (goal_x, 2);
        let dist = (self.space.1 - goal.1).abs() + (self.space.0 - goal.0).abs();
        let level = match self.color {
            Color::Amber => 1,
            Color::Bronze => 10,
            Color::Copper => 100,
            Color::Desert => 1000,
        };
        (dist * level) as usize
    }
}

impl Hash for Amphipod {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.space.hash(state);
        (self.energy == 0).hash(state);
    }
}

impl PartialEq for Amphipod {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.space == other.space
            && (self.energy == 0) == (other.energy == 0)
    }
}

struct Solver {
    cache: HashMap<Vec<Amphipod>, usize>,
    best: usize,
}

impl Solver {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            best: usize::MAX,
        }
    }

    fn solve(&mut self, input: &str) -> usize {
        let mut amphipods = vec![];
        let spaces = [
            (2, 1),
            (2, 2),
            (4, 1),
            (4, 2),
            (6, 1),
            (6, 2),
            (8, 1),
            (8, 2),
        ];
        let colors: Vec<Color> = input
            .chars()
            .map(|c| match c {
                'A' => Color::Amber,
                'B' => Color::Bronze,
                'C' => Color::Copper,
                'D' => Color::Desert,
                _ => panic!(),
            })
            .collect();
        for i in 0..8 {
            amphipods.push(Amphipod {
                space: spaces[i],
                color: colors[i].clone(),
                energy: 0,
            })
        }
        self.memo_step(amphipods)
    }

    fn check(amphipods: &Vec<Amphipod>, space: (isize, isize), color: Color) -> bool {
        if let Some(a) = amphipods.iter().find(|a| a.space == space) {
            return a.color == color;
        }
        false
    }

    fn goal(amphipods: &Vec<Amphipod>) -> bool {
        Solver::check(amphipods, (2, 1), Color::Amber)
            && Solver::check(amphipods, (2, 2), Color::Amber)
            && Solver::check(amphipods, (4, 1), Color::Bronze)
            && Solver::check(amphipods, (4, 2), Color::Bronze)
            && Solver::check(amphipods, (6, 1), Color::Copper)
            && Solver::check(amphipods, (6, 2), Color::Copper)
            && Solver::check(amphipods, (8, 1), Color::Desert)
            && Solver::check(amphipods, (8, 2), Color::Desert)
    }

    fn total_energy(amphipods: &Vec<Amphipod>) -> usize {
        amphipods.iter().map(|a| a.energy).sum()
    }

    fn memo_step(&mut self, amphipods: Vec<Amphipod>) -> usize {
        let mut sorted = amphipods.clone();
        sorted.sort_by_key(|x| x.space);
        if let Some(item) = self.cache.get(&sorted) {
            *item
        } else {
            let result = self.step(sorted.clone());
            self.cache.insert(sorted, result);
            result
        }
    }

    fn step(&mut self, amphipods: Vec<Amphipod>) -> usize {
        let total = Solver::total_energy(&amphipods);
        // if total >= self.best {
        //     return usize::MAX;
        // }
        if Solver::goal(&amphipods) {
            if total < self.best {
                self.best = total
            }
            println!("{}", total);
            return 0;
        }
        // println!("{:?}", amphipods);
        amphipods
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                let mut base = amphipods.clone();
                base.remove(i);
                Solver::possible_moves(a, &base)
                    .iter()
                    .map(|b| {
                        let mut v = base.clone();
                        v.push(b.clone());
                        let e = b.energy - a.energy;
                        e.saturating_add(self.memo_step(v))
                    })
                    .collect::<Vec<usize>>()
            })
            .min()
            .unwrap_or(usize::MAX)
    }

    fn possible_moves(amphipod: &Amphipod, amphipods: &Vec<Amphipod>) -> Vec<Amphipod> {
        let mut blocked = HashSet::new();
        for a in amphipods {
            blocked.insert(a.space);
        }
        let mut moves = vec![];
        let (x, y) = amphipod.space;
        match y {
            0 => {
                let d_x = Solver::color_x(&amphipod.color);
                let range = match x < d_x {
                    true => (x + 1)..=d_x,
                    false => d_x..=(x - 1),
                };
                let mut path = HashSet::new();
                for p_x in range {
                    path.insert((p_x, 0));
                }
                if blocked.intersection(&path).count() == 0 {
                    if !blocked.contains(&(d_x, 2)) {
                        let mut a = amphipod.clone();
                        a.move_to((d_x, 2));
                        moves.push(a);
                    } else if !blocked.contains(&(d_x, 1)) {
                        let mut a = amphipod.clone();
                        a.move_to((d_x, 1));
                        moves.push(a);
                    }
                }
            }
            1 => {
                if amphipod.energy == 0 {
                    for p_x in (x + 1)..=10 {
                        if matches!(p_x, 2 | 4 | 6 | 8) {
                            continue;
                        }
                        if !blocked.contains(&(p_x, 0)) {
                            let mut a = amphipod.clone();
                            a.move_to((p_x, 0));
                            moves.push(a);
                        } else {
                            break;
                        }
                    }
                    for p_x in (0..=(x - 1)).rev() {
                        if matches!(p_x, 2 | 4 | 6 | 8) {
                            continue;
                        }
                        if !blocked.contains(&(p_x, 0)) {
                            let mut a = amphipod.clone();
                            a.move_to((p_x, 0));
                            moves.push(a);
                        } else {
                            break;
                        }
                    }
                }
            }
            2 => {
                if amphipod.energy == 0 && !blocked.contains(&(x, 1)) {
                    for p_x in (x + 1)..=10 {
                        if matches!(p_x, 2 | 4 | 6 | 8) {
                            continue;
                        }
                        if !blocked.contains(&(p_x, 0)) {
                            let mut a = amphipod.clone();
                            a.move_to((p_x, 0));
                            moves.push(a);
                        } else {
                            break;
                        }
                    }
                    for p_x in (0..=(x - 1)).rev() {
                        if matches!(p_x, 2 | 4 | 6 | 8) {
                            continue;
                        }
                        if !blocked.contains(&(p_x, 0)) {
                            let mut a = amphipod.clone();
                            a.move_to((p_x, 0));
                            moves.push(a);
                        } else {
                            break;
                        }
                    }
                }
            }
            _ => panic!(),
        }
        moves.sort_unstable_by_key(|a| a.h());
        moves
    }

    fn hallway() -> HashSet<(isize, isize)> {
        let mut hallway = HashSet::new();
        for x in [0, 1, 3, 5, 7, 9, 10] {
            hallway.insert((x, 0));
        }
        hallway
    }

    fn color_x(c: &Color) -> isize {
        match c {
            Color::Amber => 2,
            Color::Bronze => 4,
            Color::Copper => 6,
            Color::Desert => 8,
        }
    }
}

fn calc1(input: &str) -> usize {
    let mut s = Solver::new();
    s.solve(input)
}

// fn calc2(input: &str) -> usize {
//     0
// }
