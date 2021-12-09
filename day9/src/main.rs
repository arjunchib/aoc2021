fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 15);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 1134);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> u32 {
    let mut num_col = 0;
    let map: Vec<u32> = input
        .lines()
        .inspect(|x| num_col = x.len())
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let num_row = map.len() / num_col;

    let mut total = 0;
    for x in 0..num_col {
        for y in 0..num_row {
            let i = map[x + y * num_col];
            let mut top = true;
            let mut bottom = true;
            let mut left = true;
            let mut right = true;
            if x > 0 {
                left = map[(x - 1) + y * num_col] > i
            }
            if x < num_col - 1 {
                right = map[(x + 1) + y * num_col] > i
            }
            if y > 0 {
                top = map[x + (y - 1) * num_col] > i
            }
            if y < num_row - 1 {
                bottom = map[x + (y + 1) * num_col] > i
            }
            if top && bottom && left && right {
                total += i + 1
            }
        }
    }
    total
}

struct HeightMap {
    cols: usize,
    rows: usize,
    map: Vec<u32>,
}

impl HeightMap {
    fn basin_size(&mut self, x: usize, y: usize) -> usize {
        let mut size = 1;
        self.map[x + y * self.cols] = 9;
        // Left
        if x > 0 && self.map[(x - 1) + y * self.cols] != 9 {
            self.map[(x - 1) + y * self.cols] = 9;
            size += self.basin_size(x - 1, y);
        }
        // Right
        if x < self.cols - 1 && self.map[(x + 1) + y * self.cols] != 9 {
            self.map[(x + 1) + y * self.cols] = 9;
            size += self.basin_size(x + 1, y);
        }
        // Top
        if y > 0 && self.map[x + (y - 1) * self.cols] != 9 {
            self.map[x + (y - 1) * self.cols] = 9;
            size += self.basin_size(x, y - 1);
        }
        // Bottom
        if y < self.rows - 1 && self.map[x + (y + 1) * self.cols] != 9 {
            self.map[x + (y + 1) * self.cols] = 9;
            size += self.basin_size(x, y + 1);
        }
        size
    }
}

fn calc2(input: &str) -> usize {
    let mut num_col = 0;
    let map: Vec<u32> = input
        .lines()
        .inspect(|x| num_col = x.len())
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap()))
        .collect();
    let num_row = map.len() / num_col;

    let mut hmap = HeightMap {
        cols: num_col,
        rows: num_row,
        map,
    };

    let mut lows: Vec<(usize, usize)> = vec![];
    for x in 0..num_col {
        for y in 0..num_row {
            let i = hmap.map[x + y * num_col];
            let mut top = true;
            let mut bottom = true;
            let mut left = true;
            let mut right = true;
            if x > 0 {
                left = hmap.map[(x - 1) + y * num_col] > i
            }
            if x < num_col - 1 {
                right = hmap.map[(x + 1) + y * num_col] > i
            }
            if y > 0 {
                top = hmap.map[x + (y - 1) * num_col] > i
            }
            if y < num_row - 1 {
                bottom = hmap.map[x + (y + 1) * num_col] > i
            }
            if top && bottom && left && right {
                lows.push((x, y));
            }
        }
    }
    let mut basins: Vec<usize> = lows.iter().map(|(x, y)| hmap.basin_size(*x, *y)).collect();
    basins.sort_unstable();
    basins.into_iter().rev().take(3).product::<usize>()
}
