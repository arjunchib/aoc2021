use std::fmt;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 4140);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 3993);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug, Clone)]
enum SnailElement {
    Number(usize),
    LevelUp,
    LevelDown,
}

#[derive(Debug, Clone)]
struct SnailNumber {
    elements: Vec<SnailElement>,
}

impl SnailNumber {
    fn from(s: &str) -> Self {
        let mut elements = vec![];
        for c in s.chars().filter(|c| *c != ',') {
            let el = match c {
                '[' => SnailElement::LevelUp,
                ']' => SnailElement::LevelDown,
                _ => SnailElement::Number(c.to_digit(10).unwrap() as usize),
            };
            elements.push(el);
        }
        SnailNumber { elements }
    }

    fn add(&mut self, other: &mut Self) {
        self.elements.append(&mut other.elements);
        self.elements.insert(0, SnailElement::LevelUp);
        self.elements.push(SnailElement::LevelDown);
        self.reduce()
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            };
            if self.split() {
                continue;
            };
            break;
        }
    }

    fn magnitude(&self) -> usize {
        let mut reduction = self.elements.clone();
        while reduction.len() > 1 {
            let mut new_reduction = vec![];
            let mut skip = 0;
            for (i, el) in reduction.iter().enumerate() {
                if skip > 0 {
                    skip -= 1;
                    continue;
                }
                if let SnailElement::Number(a) = el {
                    if let SnailElement::Number(b) = reduction[i + 1] {
                        new_reduction.pop();
                        new_reduction.push(SnailElement::Number(a * 3 + b * 2));
                        skip = 2;
                        continue;
                    }
                }
                new_reduction.push(el.clone());
            }
            // println!(
            //     "{}",
            //     SnailNumber {
            //         elements: new_reduction.clone()
            //     }
            // );
            reduction = new_reduction;
        }
        if let SnailElement::Number(n) = reduction[0] {
            return n;
        }
        panic!("Failed!");
    }

    fn explode(&mut self) -> bool {
        let mut level = 0;
        let mut index = None;
        for (i, el) in self.elements.iter().enumerate() {
            match el {
                SnailElement::LevelUp => level += 1,
                SnailElement::LevelDown => level -= 1,
                SnailElement::Number(_) => {
                    if level >= 5 {
                        index = Some(i);
                        break;
                    }
                }
            }
        }
        if index.is_none() {
            return false;
        }
        let index = index.unwrap();
        if let SnailElement::Number(left) = self.elements[index] {
            if let SnailElement::Number(right) = self.elements[index + 1] {
                for el in self.elements[0..index].iter_mut().rev() {
                    if let SnailElement::Number(n) = el {
                        *el = SnailElement::Number(*n + left);
                        break;
                    }
                }
                for el in self.elements[index + 2..].iter_mut() {
                    if let SnailElement::Number(n) = el {
                        *el = SnailElement::Number(*n + right);
                        break;
                    }
                }
                self.elements[index] = SnailElement::Number(0);
                self.elements.remove(index + 2); //]
                self.elements.remove(index + 1); //right
                self.elements.remove(index - 1); //[
                return true;
            }
        }
        false
    }

    fn split(&mut self) -> bool {
        let mut index = None;
        for (i, el) in self.elements.iter().enumerate() {
            if let SnailElement::Number(n) = el {
                if *n >= 10 {
                    index = Some(i);
                    break;
                }
            }
        }
        if index.is_none() {
            return false;
        }
        let index = index.unwrap();
        if let SnailElement::Number(num) = self.elements[index] {
            let left = num / 2;
            let right = match num % 2 {
                0 => num / 2,
                _ => num / 2 + 1,
            };
            self.elements.remove(index);
            self.elements.insert(index, SnailElement::LevelDown);
            self.elements.insert(index, SnailElement::Number(right));
            self.elements.insert(index, SnailElement::Number(left));
            self.elements.insert(index, SnailElement::LevelUp);
            return true;
        }
        false
    }
}

impl fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self
            .elements
            .iter()
            .map(|el| match el {
                SnailElement::LevelUp => String::from('['),
                SnailElement::LevelDown => String::from(']'),
                SnailElement::Number(n) => n.to_string(),
            })
            .collect();
        write!(f, "{}", s)
    }
}

fn calc1(input: &str) -> usize {
    let mut nums: Vec<SnailNumber> = input.lines().map(|x| SnailNumber::from(x)).collect();
    let sum = nums
        .drain(..)
        .reduce(|mut a, mut b| {
            a.add(&mut b);
            a
        })
        .unwrap();
    sum.magnitude()
}

fn calc2(input: &str) -> usize {
    let nums: Vec<SnailNumber> = input.lines().map(|x| SnailNumber::from(x)).collect();
    nums.iter()
        .flat_map(|x| nums.iter().map(move |y| (x, y)))
        .map(|(a, b)| {
            let mut new = a.clone();
            new.add(&mut b.clone());
            new.magnitude()
        })
        .max()
        .unwrap()
}
