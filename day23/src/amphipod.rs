use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
  Amber,
  Bronze,
  Copper,
  Desert,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Amphipod {
  color: Color,
  space: (isize, isize),
  moved: bool,
}

impl Amphipod {
  fn new(c: char, space: (isize, isize)) -> Self {
    let color = match c {
      'A' => Color::Amber,
      'B' => Color::Bronze,
      'C' => Color::Copper,
      'D' => Color::Desert,
      _ => panic!(),
    };
    Self {
      color,
      space,
      moved: false,
    }
  }

  fn dest_x(&self) -> isize {
    match self.color {
      Color::Amber => 3,
      Color::Bronze => 5,
      Color::Copper => 7,
      Color::Desert => 9,
    }
  }

  fn goal(&self) -> bool {
    self.dest_x() == self.space.0
  }

  fn dist(&self, space: (isize, isize)) -> usize {
    ((self.space.1 - space.1).abs() + (self.space.0 - space.0).abs()) as usize
  }

  fn energy(&self, space: (isize, isize)) -> usize {
    let level = match self.color {
      Color::Amber => 1,
      Color::Bronze => 10,
      Color::Copper => 100,
      Color::Desert => 1000,
    };
    self.dist(space) * level
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Burrow {
  amphipods: Vec<Amphipod>,
  height: isize,
}

impl Burrow {
  pub fn new(input: &str) -> Self {
    let mut amphipods = vec![];
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
      height += 1;
      for (x, c) in line.chars().enumerate() {
        if matches!(c, 'A' | 'B' | 'C' | 'D') {
          amphipods.push(Amphipod::new(c, (x as isize, y as isize)));
        }
      }
    }
    Self { amphipods, height }
  }

  pub fn next_burrows(&self) -> Vec<(Burrow, usize)> {
    let mut results = vec![];
    for (i, amphipod) in self.amphipods.iter().enumerate() {
      for (x, e) in self.next_amphipods(amphipod) {
        let mut amphipods = self.amphipods.clone();
        amphipods.push(x);
        amphipods.swap_remove(i);
        results.push((
          Burrow {
            amphipods,
            height: self.height,
          },
          e,
        ));
      }
    }
    results.sort_by_key(|x| x.1);
    results
  }

  fn next_amphipods(&self, amphipod: &Amphipod) -> Vec<(Amphipod, usize)> {
    match amphipod.space.1 {
      1 => self.next_amphipods_hallway(amphipod),
      _ => self.next_amphipods_room(amphipod),
    }
  }

  fn next_amphipods_hallway(&self, amphipod: &Amphipod) -> Vec<(Amphipod, usize)> {
    let mut results = vec![];
    let blocked: HashSet<(isize, isize)> =
      HashSet::from_iter(self.amphipods.iter().map(|x| x.space));
    let (x1, _) = amphipod.space;
    let x2 = amphipod.dest_x();
    let range = match x1 < x2 {
      true => (x1 + 1)..=x2,
      false => x2..=(x1 - 1),
    };
    let path = HashSet::from_iter(range.map(|x| (x, 1)));
    if blocked.intersection(&path).count() == 0 {
      if self
        .amphipods
        .iter()
        .filter(|x| x.space.0 == x2)
        .any(|x| x.color != amphipod.color)
      {
        return results;
      }
      let y2 = blocked
        .iter()
        .filter(|(x, _)| *x == x2)
        .map(|(_, y)| y)
        .min()
        .unwrap_or(&(self.height - 1))
        - 1;
      if y2 > 1 {
        let mut new_amphipod = amphipod.clone();
        new_amphipod.space = (x2, y2);
        results.push((new_amphipod, amphipod.energy((x2, y2))));
      }
    };
    results
  }

  fn next_amphipods_room(&self, amphipod: &Amphipod) -> Vec<(Amphipod, usize)> {
    let mut results = vec![];
    if amphipod.moved {
      return results;
    }
    let (x1, y1) = amphipod.space;
    let blocked: HashSet<(isize, isize)> =
      HashSet::from_iter(self.amphipods.iter().map(|x| x.space));
    for y in 2..y1 {
      if blocked.contains(&(x1, y)) {
        return results;
      }
    }
    for x in (x1 + 1)..=11 {
      if matches!(x, 3 | 5 | 7 | 9) {
        continue;
      }
      if !blocked.contains(&(x, 1)) {
        let mut new_amphipod = amphipod.clone();
        new_amphipod.space = (x, 1);
        new_amphipod.moved = true;
        results.push((new_amphipod, amphipod.energy((x, 1))))
      } else {
        break;
      }
    }
    for x in (1..=(x1 - 1)).rev() {
      if matches!(x, 3 | 5 | 7 | 9) {
        continue;
      }
      if !blocked.contains(&(x, 1)) {
        let mut new_amphipod = amphipod.clone();
        new_amphipod.space = (x, 1);
        new_amphipod.moved = true;
        results.push((new_amphipod, amphipod.energy((x, 1))))
      } else {
        break;
      }
    }
    results
  }

  pub fn goal(&self) -> bool {
    self.amphipods.iter().all(|x| x.goal())
  }

  pub fn sort(&mut self) {
    self.amphipods.sort();
  }
}

impl fmt::Display for Burrow {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut template: Vec<char> = include_str!("template.in").chars().collect();
    for a in &self.amphipods {
      let (x, y) = a.space;
      let c = match a.color {
        Color::Amber => 'A',
        Color::Bronze => 'B',
        Color::Copper => 'C',
        Color::Desert => 'D',
      };
      template[(x + y * 14) as usize] = c;
    }
    write!(f, "{}", template.iter().collect::<String>())
  }
}
