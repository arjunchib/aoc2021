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
    counts: HashMap<char, usize>,
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
        let counts = template.chars().fold(HashMap::new(), |mut map, x| {
            let count = map.entry(x).or_insert(0);
            *count += 1;
            map
        });
        Polymer {
            template,
            rules,
            counts,
        }
    }

    fn range(&mut self, limit: usize) -> usize {
        let els: Vec<char> = self.template.chars().collect();
        els.windows(2)
            .for_each(|x| self.insert(x[0], x[1], 0, limit));
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        for count in self.counts.values() {
            if *count < min {
                min = *count
            }
            if *count > max {
                max = *count
            }
        }
        max - min
    }

    fn insert(&mut self, a: char, b: char, index: usize, limit: usize) {
        let el = *self.rules.get(&(a, b)).unwrap();
        let count = self.counts.entry(el).or_insert(0);
        *count += 1;
        if index + 1 == limit {
            return;
        }
        self.insert(a, el, index + 1, limit);
        self.insert(el, b, index + 1, limit);
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
