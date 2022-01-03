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
    println!("{}", now.elapsed().as_secs());

    // Part 2
    // assert_eq!(calc2(include_str!("test.in")), 3993);
    // println!("part 2: {}", calc2(include_str!("real.in")));
}

#[derive(Debug, Clone)]
enum Operand {
    Variable(char),
    Number(isize),
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Debug, Clone)]
struct Instruction(Operation, char, Operand);

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let op = parts.next().unwrap();
        let a = parts.next().unwrap();
        let b = parts.next().unwrap_or("0");
        let op = match op {
            "inp" => Operation::Inp,
            "add" => Operation::Add,
            "mul" => Operation::Mul,
            "div" => Operation::Div,
            "mod" => Operation::Mod,
            "eql" => Operation::Eql,
            _ => panic!(),
        };
        let a = a.chars().last().unwrap();
        let b = match b.parse() {
            Ok(n) => Operand::Number(n),
            Err(_) => Operand::Variable(b.chars().last().unwrap()),
        };
        Instruction(op, a, b)
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct ALU {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl ALU {
    fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn run(&mut self, instruction: &Instruction) {
        let a = self.get(instruction.1);
        let b = match instruction.2 {
            Operand::Variable(b) => self.get(b),
            Operand::Number(b) => b,
        };
        let result = match &instruction.0 {
            Operation::Inp => b,
            Operation::Add => a + b,
            Operation::Mul => a * b,
            Operation::Div => a / b,
            Operation::Mod => a % b,
            Operation::Eql => match a == b {
                true => 1,
                false => 0,
            },
        };
        self.write(instruction.1, result);
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
}

struct Program {
    instructions: Vec<Instruction>,
    cache: HashMap<(isize, isize, isize), isize, RandomState>,
    hits: usize,
    total: usize,
}

impl Program {
    fn from(input: &str) -> Self {
        Self {
            instructions: input.lines().map(Instruction::from).collect(),
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
            let mut ins = ins.clone();
            if ins.0 == Operation::Inp {
                ins.2 = Operand::Number(ws.next().unwrap());
            }
            alu.run(&ins);
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
        // self.total += 1;
        // if let Some(n) = self.cache.get(&(i, z, w)) {
        //     self.hits += 1;
        //     return *n;
        // }
        let mut alu = ALU::new();
        alu.z = z;
        let start = i * 18;
        let mut ins = self.instructions[start as usize].clone();
        ins.2 = Operand::Number(w);
        alu.run(&ins);
        for j in start + 1..start + 18 {
            alu.run(&self.instructions[j as usize]);
        }
        // self.cache.insert((i, z, w), alu.z);
        alu.z
    }

    fn highest_root(&mut self) -> isize {
        let mut states: HashMap<isize, isize, RandomState> = HashMap::default();
        states.insert(0, 0);
        for i in 0..14 {
            let mut new_states: HashMap<isize, isize, RandomState> = HashMap::default();
            for (z, num) in states.drain() {
                for j in (1..=9).rev() {
                    let z = self.run_sub(i, z, j);
                    let num = num + j * isize::pow(10, 13 - i as u32);
                    new_states.entry(z).or_insert(num);
                }
            }
            states = new_states;
            println!("{:2} {}", i, states.len(),);
        }
        let mut nums: Vec<isize> = states.values().copied().collect();
        nums.sort_unstable();
        for num in nums.iter().rev() {
            if self.fast_run(*num) == 0 {
                return *num;
            };
        }
        0
    }
}

fn calc1(input: &str) -> isize {
    let mut p = Program::from(input);
    // p.run(16964171414113)
    p.highest_root()
}

// fn calc2(input: &str) -> usize {
//     0
// }
