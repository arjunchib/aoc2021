use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 1588);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 2188189693529);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

struct Polymer {
    template: String,
    rules: HashMap<(char, char), char>,
    memo: HashMap<(char, char, usize), HashMap<char, usize>>,
}

impl Polymer {
    fn from(input: &str) -> Self {
        let mut lines = input.lines();
        let template = String::from(lines.next().unwrap());
        let rules = lines.skip(1).fold(HashMap::new(), |mut map, x| {
            let (pair, el) = x.split_once(" -> ").unwrap();
            let v: Vec<char> = pair.chars().collect();
            let w: Vec<char> = el.chars().collect();
            map.insert((v[0], v[1]), w[0]);
            map
        });
        Polymer {
            template,
            rules,
            memo: HashMap::new(),
        }
    }

    fn range(&mut self, limit: usize) -> usize {
        let els: Vec<char> = self.template.chars().collect();
        let counts = els.iter().fold(HashMap::new(), |mut map, x| {
            let count = map.entry(*x).or_insert(0);
            *count += 1;
            map
        });
        let counts = els.windows(2).fold(counts, |mut map, x| {
            let counts = self.insert(x[0], x[1], 0, limit);
            for (c, new_count) in counts {
                let count = map.entry(c).or_insert(0);
                *count += new_count;
            }
            map
        });
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for count in counts.values() {
            if *count < min {
                min = *count
            }
            if *count > max {
                max = *count
            }
        }
        max - min
    }

    fn memo_insert(
        &mut self,
        a: char,
        b: char,
        index: usize,
        limit: usize,
    ) -> HashMap<char, usize> {
        let cache = self.memo.get(&(a, b, index));
        if cache == None {
            let map = self.insert(a, b, index, limit);
            self.memo.insert((a, b, index), map.clone());
            map
        } else {
            cache.unwrap().clone()
        }
    }

    fn insert(&mut self, a: char, b: char, index: usize, limit: usize) -> HashMap<char, usize> {
        if index == limit {
            return HashMap::new();
        }
        let el = *self.rules.get(&(a, b)).unwrap();
        let left = self.memo_insert(a, el, index + 1, limit);
        let right = self.memo_insert(el, b, index + 1, limit);
        let mut map = left.clone();
        for (c, r_count) in right {
            let count = map.entry(c).or_insert(0);
            *count += r_count;
        }
        let count = map.entry(el).or_insert(0);
        *count += 1;
        map
    }
}

fn calc1(input: &str) -> usize {
    let mut p = Polymer::from(input);
    p.range(10)
}

fn calc2(input: &str) -> usize {
    let mut p = Polymer::from(input);
    p.range(40)
}
