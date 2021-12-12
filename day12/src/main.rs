use std::collections::HashMap;

fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 10);
    assert_eq!(calc1(include_str!("test2.in")), 19);
    assert_eq!(calc1(include_str!("test3.in")), 226);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 36);
    assert_eq!(calc2(include_str!("test2.in")), 103);
    assert_eq!(calc2(include_str!("test3.in")), 3509);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn is_uppercase(s: &str) -> bool {
    s.to_uppercase() == s
}

struct Graph {
    edges: HashMap<&'static str, Vec<&'static str>>,
}

impl Graph {
    fn total_paths(&self) -> usize {
        self.total_path_rec("start", &vec![])
    }

    fn total_path_rec(&self, node: &'static str, visited: &Vec<&str>) -> usize {
        let mut updated_visited = visited.clone();
        if !is_uppercase(node) {
            updated_visited.push(node);
        }
        let mut total = 0;
        let nodes = self.edges.get(node);
        if nodes == None {
            return 0;
        }
        for n in nodes.unwrap() {
            if *n == "end" {
                total += 1
            } else if !visited.contains(n) {
                total += self.total_path_rec(n, &updated_visited);
            }
        }
        total
    }

    fn total_paths2(&self) -> usize {
        let mut v = self.total_path_rec2("start", &vec![], true, &vec![]);
        v.sort();
        v.dedup();
        v.len()
    }

    fn total_path_rec2(
        &self,
        node: &'static str,
        visited: &Vec<&str>,
        has_mulligan: bool,
        path: &Vec<&str>,
    ) -> Vec<String> {
        let mut updated_path = path.clone();
        updated_path.push(node);
        let mut updated_visited = visited.clone();
        if !is_uppercase(node) {
            updated_visited.push(node);
        }
        let mut all = vec![];
        for n in self.edges.get(node).unwrap() {
            if *n == "end" {
                updated_path.push("end");
                all.push(updated_path.join(","));
            } else if !visited.contains(n) {
                all.append(&mut self.total_path_rec2(
                    n,
                    &updated_visited,
                    has_mulligan,
                    &updated_path,
                ));
                if has_mulligan && node != "start" && !is_uppercase(node) {
                    all.append(&mut self.total_path_rec2(n, visited, false, &updated_path));
                }
            }
        }
        all
    }
}

fn calc1(input: &'static str) -> usize {
    let edges: HashMap<&str, Vec<&str>> =
        input
            .lines()
            .map(|x| x.split_once('-').unwrap())
            .fold(HashMap::new(), |mut map, x| {
                let v = map.entry(x.0).or_insert(vec![]);
                v.push(x.1);
                let v = map.entry(x.1).or_insert(vec![]);
                v.push(x.0);
                map
            });

    let g = Graph { edges };
    g.total_paths()
}

fn calc2(input: &'static str) -> usize {
    let edges: HashMap<&str, Vec<&str>> =
        input
            .lines()
            .map(|x| x.split_once('-').unwrap())
            .fold(HashMap::new(), |mut map, x| {
                let v = map.entry(x.0).or_insert(vec![]);
                v.push(x.1);
                let v = map.entry(x.1).or_insert(vec![]);
                v.push(x.0);
                map
            });
    let g = Graph { edges };
    g.total_paths2()
}
