use ahash::RandomState;
use std::collections::HashMap;
use std::time::Instant;

fn main() {
    // Part 1
    // println!("part 1: {}", calc1(include_str!("test1.in")));
    // println!("part 1: {}", calc1(include_str!("test2.in")));
    // println!("part 1: {}", calc1(include_str!("test3.in")));
    let now = Instant::now();
    println!("part 1: {}", calc1(include_str!("real.in")));
    println!("{}", now.elapsed().as_micros());

    // Part 2
    // assert_eq!(calc2(include_str!("test.in")), 3993);
    // println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug)]
enum Operand {
    Variable(char),
    Number(isize),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct ALU {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
    ip: usize,
}

impl ALU {
    fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            ip: 0,
        }
    }

    fn convert_operand(a: &str) -> Operand {
        match a {
            "w" | "x" | "y" | "z" => Operand::Variable(a.chars().last().unwrap()),
            _ => Operand::Number(a.parse().unwrap()),
        }
    }

    fn reset(&mut self) {
        self.w = 0;
        self.x = 0;
        self.y = 0;
        self.z = 0;
        self.ip = 0;
    }

    fn step(&mut self, line: &str, input: Option<isize>) {
        let args: Vec<&str> = line.split_whitespace().collect();
        let op = args[0];
        let a = ALU::convert_operand(args[1]);
        let b = match input {
            Some(input) => Operand::Number(input),
            None => ALU::convert_operand(args.get(2).unwrap()),
        };
        match op {
            "inp" => self.inp(a, b),
            "add" => self.add(a, b),
            "mul" => self.mul(a, b),
            "div" => self.div(a, b),
            "mod" => self.modulo(a, b),
            "eql" => self.eql(a, b),
            _ => panic!(),
        }
        self.ip += 1;
    }

    fn write(&mut self, register: char, value: isize) {
        match register {
            'w' => self.w = value,
            'x' => self.x = value,
            'y' => self.y = value,
            'z' => self.z = value,
            _ => panic!(),
        }
    }

    fn get(&self, register: char) -> isize {
        match register {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => panic!(),
        }
    }

    fn operand_value(&self, b: Operand) -> isize {
        match b {
            Operand::Variable(b) => self.get(b),
            Operand::Number(b) => b,
        }
    }

    fn inp(&mut self, a: Operand, b: Operand) {
        if let Operand::Variable(a) = a {
            if let Operand::Number(b) = b {
                self.write(a, b);
            }
        }
    }

    fn add(&mut self, a: Operand, b: Operand) {
        if let Operand::Variable(a) = a {
            let b = self.operand_value(b);
            self.write(a, self.get(a) + b);
        }
    }

    fn mul(&mut self, a: Operand, b: Operand) {
        if let Operand::Variable(a) = a {
            let b = self.operand_value(b);
            self.write(a, self.get(a) * b);
        }
    }

    fn div(&mut self, a: Operand, b: Operand) {
        if let Operand::Variable(a) = a {
            let b = self.operand_value(b);
            self.write(a, self.get(a) / b);
        }
    }

    fn modulo(&mut self, a: Operand, b: Operand) {
        if let Operand::Variable(a) = a {
            let b = self.operand_value(b);
            self.write(a, self.get(a) % b);
        }
    }

    fn eql(&mut self, a: Operand, b: Operand) {
        if let Operand::Variable(a) = a {
            let b = self.operand_value(b);
            let value = match self.get(a) == b {
                true => 1,
                false => 0,
            };
            self.write(a, value);
        }
    }
}

struct Program {
    instructions: Vec<String>,
    cache: HashMap<(isize, isize, isize), isize, RandomState>,
    hits: usize,
    total: usize,
}

impl Program {
    fn from(input: &str) -> Self {
        Self {
            instructions: input.lines().map(String::from).collect(),
            cache: HashMap::default(),
            hits: 0,
            total: 0,
        }
    }

    fn run(&self, a: isize) -> isize {
        let a = a.to_string();
        let mut ws = a.chars().map(|x| x.to_digit(10).unwrap() as isize);
        let mut alu = ALU::new();
        for ins in &self.instructions {
            let w = match ins.starts_with("inp") {
                true => ws.next(),
                false => None,
            };
            alu.step(ins, w);
        }
        alu.z
    }

    fn fast_run(&mut self, a: isize) -> isize {
        let a = a.to_string();
        let mut ws = a.chars().map(|x| x.to_digit(10).unwrap() as isize);
        let mut z = 0;
        for i in 0..14 {
            z = self.run_sub(i, z, ws.next().unwrap());
        }
        z
    }

    fn run_sub(&mut self, i: isize, z: isize, w: isize) -> isize {
        self.total += 1;
        if let Some(n) = self.cache.get(&(i, z, w)) {
            self.hits += 1;
            return *n;
        }
        let mut alu = ALU::new();
        alu.z = z;
        let start = i * 18;
        for j in start..start + 18 {
            let ins = &self.instructions[j as usize];
            let w = match ins.starts_with("inp") {
                true => Some(w),
                false => None,
            };
            alu.step(ins, w);
        }
        self.cache.insert((i, z, w), alu.z);
        alu.z
    }

    fn secant(&self) -> isize {
        let mut p0: i128 = 99999999999999;
        let mut p1: i128 = 99999999999998;
        let mut q0 = self.run(p0 as isize) as i128;
        let mut q1 = self.run(p1 as isize) as i128;
        loop {
            println!("{} {} {} {}", p0, p1, q0, q1);
            let p = p1 - q1 * (p1 - p0) / (q1 - q0);
            if (p - p1).abs() == 0 {
                break p as isize;
            }
            println!("{}", p);
            p0 = p1;
            q0 = q1;
            p1 = p;
            q1 = self.run(p as isize) as i128;
        }
    }
}

fn calc1(input: &str) -> isize {
    let mut p = Program::from(input);
    for i in (11111111111111..99999999999999).rev().take(1000) {
        // assert_eq!(p.fast_run(i), p.run(i));
        // println!("{} {}", i, p.fast_run(i));
        p.fast_run(i);
    }
    println!("{}", p.hits as f32 / p.total as f32);
    0
    // p.secant() as usize
}

// fn calc2(input: &str) -> usize {
//     0
// }
