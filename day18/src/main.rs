use std::fmt;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 4140);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 2188189693529);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug)]
enum Action {
    Explode,
    Split,
}

#[derive(Debug, Clone)]
enum Element {
    Pair(SnailNumber),
    Number(usize),
}

#[derive(Debug, Clone)]
struct SnailNumber {
    x: Box<Element>,
    y: Box<Element>,
    parent: Option<Box<SnailNumber>>,
}

fn pivot(s: &str) -> Option<usize> {
    let mut stack = vec![];
    for (i, c) in s.chars().enumerate() {
        match c {
            '[' => {
                stack.push('[');
            }
            ']' => {
                stack.pop();
            }
            ',' if stack.is_empty() => {
                return Some(i);
            }
            _ => {}
        }
    }
    None
}

impl SnailNumber {
    fn from(s: &str) -> Self {
        let s = &s[1..s.len() - 1];
        let (s1, s2) = s.split_at(pivot(s).unwrap());
        let s2 = &s2[1..];
        let x = Box::new(match s1.parse::<usize>() {
            Ok(n) => Element::Number(n),
            _ => Element::Pair(SnailNumber::from(s1)),
        });
        let y = Box::new(match s2.parse::<usize>() {
            Ok(n) => Element::Number(n),
            _ => Element::Pair(SnailNumber::from(s2)),
        });
        let parent = Box::new()
        SnailNumber { x, y, parent }
    }

    fn add(&self, other: &Self) -> Self {
        let mut n = SnailNumber {
            x: Box::new(Element::Pair(self.clone())),
            y: Box::new(Element::Pair(other.clone())),
        };
        n.reduce();
        n
    }

    fn reduce(&mut self) {
        loop {
            if let Some(n) = self.nest4() {
                self.explode(n);
            }
        }
    }

    fn nest4(&self) -> Option<&Self> {
        let mut stack = vec![self];
        while !stack.is_empty() {
            if stack.len() == 5 {
                return Some(stack.pop().unwrap());
            }
            if let Element::Pair(n) = &*self.x {
                stack.push(n);
                continue;
            }
            if let Element::Pair(n) = &*self.y {
                stack.push(n);
                continue;
            }
            stack.pop();
        }
        None
    }

    fn greater10(&self) -> Option<&Self> {
        let mut stack = vec![self];
        while !stack.is_empty() {
            if stack.len() == 5 {
                return Some(stack.pop().unwrap());
            }
            if let Element::Pair(n) = &*self.x {
                stack.push(n);
                continue;
            }
            if let Element::Pair(n) = &*self.y {
                stack.push(n);
                continue;
            }
            stack.pop();
        }
        None
    }

    fn explode(&mut self, pair: &SnailNumber) {}
}

impl fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s1 = match &*self.x {
            Element::Number(n) => n.to_string(),
            Element::Pair(n) => n.to_string(),
        };
        let s2 = match &*self.y {
            Element::Number(n) => n.to_string(),
            Element::Pair(n) => n.to_string(),
        };
        write!(f, "[{},{}]", s1, s2)
    }
}

fn calc1(input: &str) -> usize {
    let nums: Vec<SnailNumber> = input.lines().map(|x| SnailNumber::from(x)).collect();
    let sum = nums[0].add(&nums[1]);
    println!("{}", sum);
    0
}

fn calc2(input: &str) -> usize {
    0
}
