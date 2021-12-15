fn main() {
    // Part 1
    assert_eq!(calc1(include_str!("test.in")), 40);
    println!("part 1: {}", calc1(include_str!("real.in")));

    // Part 2
    assert_eq!(calc2(include_str!("test.in")), 315);
    println!("part 2: {}", calc2(include_str!("real.in")));
}

fn calc1(input: &str) -> u32 {
    let mut lines = input.lines().peekable();
    let w = lines.peek().unwrap().chars().count();
    let v: Vec<u32> = lines
        .flat_map(|x| x.chars())
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let h = v.len() / w;
    let mut dists: Vec<u32> = vec![0; v.len()];
    for n in 1..(w + h) {
        for i in 0..=n {
            let x = n - i;
            let y = i;
            if x >= w || y >= h {
                continue;
            }
            let mut a = u32::MAX;
            if x != 0 {
                a = dists[(x - 1) + y * w]
            }
            let mut b = u32::MAX;
            if y != 0 {
                b = dists[x + (y - 1) * w]
            }
            dists[x + y * w] = a.min(b) + v[x + y * w];
        }
    }
    *dists.last().unwrap()
}

fn calc2(input: &str) -> u32 {
    let mut lines = input.lines().peekable();
    let w_temp = lines.peek().unwrap().chars().count();
    let w = w_temp * 5;
    let template: Vec<u32> = lines
        .flat_map(|x| x.chars())
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let h_temp = template.len() / w_temp;
    let h = h_temp * 5;
    let mut v: Vec<u32> = vec![0; w * h];
    for y in 0..5 {
        for x in 0..5 {
            let n = x + y;
            let chunk: Vec<u32> = template
                .iter()
                .map(|i| {
                    let mut new = i + n as u32;
                    if new > 9 {
                        new -= 9
                    }
                    new
                })
                .collect();
            for j in 0..h_temp {
                for i in 0..w_temp {
                    v[(w_temp * x + i) + (y * h_temp * w) + (w * j)] = chunk[i + j * w_temp];
                }
            }
        }
    }
    let mut dists: Vec<u32> = vec![0; v.len()];
    for n in 1..(w + h) {
        for i in 0..=n {
            let x = n - i;
            let y = i;
            if x >= w || y >= h {
                continue;
            }
            let mut a = u32::MAX;
            if x != 0 {
                a = dists[(x - 1) + y * w]
            }
            let mut b = u32::MAX;
            if y != 0 {
                b = dists[x + (y - 1) * w]
            }
            dists[x + y * w] = a.min(b) + v[x + y * w];
        }
    }
    for _ in 0..500 {
        for n in 1..(w + h) {
            for i in 0..=n {
                let x = n - i;
                let y = i;
                if x >= w || y >= h {
                    continue;
                }
                let mut a = u32::MAX;
                if x != 0 {
                    a = dists[(x - 1) + y * w]
                }
                let mut b = u32::MAX;
                if y != 0 {
                    b = dists[x + (y - 1) * w]
                }
                let mut c = u32::MAX;
                if x != w - 1 {
                    c = dists[(x + 1) + y * w]
                }
                let mut d = u32::MAX;
                if y != h - 1 {
                    d = dists[x + (y + 1) * w]
                }
                dists[x + y * w] = a.min(b).min(c).min(d) + v[x + y * w];
            }
        }
    }
    // v.chunks(w).for_each(|x| println!("{:?}", x));
    *dists.last().unwrap()
}
