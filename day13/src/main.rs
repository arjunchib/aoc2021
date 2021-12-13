fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 17);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    calc2(include_str!("real.in"));
}

struct Paper {
    points: Vec<(usize, usize)>,
}

impl Paper {
    fn fold_y(&mut self, y_axis: usize) {
        self.points = self
            .points
            .iter()
            .map(|(x, y)| {
                let new_y = match *y > y_axis {
                    true => y_axis - (*y - y_axis),
                    false => *y,
                };
                (*x, new_y)
            })
            .collect();
    }

    fn fold_x(&mut self, x_axis: usize) {
        self.points = self
            .points
            .iter()
            .map(|(x, y)| {
                let new_x = match *x > x_axis {
                    true => x_axis - (*x - x_axis),
                    false => *x,
                };
                (new_x, *y)
            })
            .collect();
    }

    fn total_visible_points(&self) -> usize {
        let mut pts = self.points.clone();
        pts.sort_unstable();
        pts.dedup();
        pts.len()
    }
}

fn calc1(input: &str) -> usize {
    let mut lines = input.lines();
    let points: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|x| !x.trim().is_empty())
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let (axis, val) = lines.next().unwrap()[11..].split_once('=').unwrap();
    let val = val.parse().unwrap();
    let mut p = Paper { points };
    match axis {
        "x" => p.fold_x(val),
        "y" => p.fold_y(val),
        _ => {}
    }
    p.total_visible_points()
}

fn calc2(input: &str) {
    let mut lines = input.lines();
    let points: Vec<(usize, usize)> = lines
        .by_ref()
        .take_while(|x| !x.trim().is_empty())
        .map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let mut p = Paper { points };
    for line in lines {
        let (axis, val) = line[11..].split_once('=').unwrap();
        let val = val.parse().unwrap();
        match axis {
            "x" => p.fold_x(val),
            "y" => p.fold_y(val),
            _ => {}
        }
    }
    println!("{:?}", p.points);
}
