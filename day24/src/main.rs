use std::collections::HashMap;

fn main() {
    // Part 1
    // println!("part 1: {}", calc1(include_str!("test1.in")));
    // println!("part 1: {}", calc1(include_str!("test2.in")));
    // println!("part 1: {}", calc1(include_str!("test3.in")));
    println!("part 1: {}", calc1(include_str!("real.in")));

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

    // fn run(&mut self, input: &str, program: &str) {
    //     self.reset();
    //     let program = program.lines();
    //     let input: Vec<&str> = input.split("").skip(1).collect();
    //     let mut input = input.iter();
    //     for line in program {
    //         let args: Vec<&str> = line.split_whitespace().collect();
    //         let op = args[0];
    //         let a = ALU::convert_operand(args[1]);
    //         let b = ALU::convert_operand(args.get(2).unwrap_or_else(|| input.next().unwrap()));
    //         match op {
    //             "inp" => self.inp(a, b),
    //             "add" => self.add(a, b),
    //             "mul" => self.mul(a, b),
    //             "div" => self.div(a, b),
    //             "mod" => self.modulo(a, b),
    //             "eql" => self.eql(a, b),
    //             _ => panic!(),
    //         }
    //     }
    // }

    fn step(&mut self, line: &str, input: Option<&str>) {
        let args: Vec<&str> = line.split_whitespace().collect();
        let op = args[0];
        let a = ALU::convert_operand(args[1]);
        let b = ALU::convert_operand(args.get(2).unwrap_or_else(|| input.as_ref().unwrap()));
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
    cache: HashMap<(isize, usize), Option<String>>,
}

impl Program {
    fn from(input: &str) -> Self {
        Self {
            instructions: input.lines().map(String::from).collect(),
            cache: HashMap::new(),
        }
    }

    fn analyze(&mut self) -> isize {
        let result = self.analyze_step(ALU::new());
        result.unwrap().parse().unwrap()
    }

    fn memo_analyze_step(&mut self, alu: ALU) -> Option<String> {
        if let Some(c) = self.cache.get(&(alu.z, alu.ip)) {
            c.clone()
        } else {
            let val = self.analyze_step(alu.clone());
            if val.is_some() {
                println!("{:?}", val);
            }
            self.cache.insert((alu.z, alu.ip), val.clone());
            val
        }
    }

    fn analyze_step(&mut self, alu: ALU) -> Option<String> {
        if alu.ip == self.instructions.len() {
            if alu.z < 1000 {
                println!("{:?}", alu)
            };
            return match alu.z {
                0 => Some(String::from("")),
                _ => None,
            };
        }
        let line = self.instructions[alu.ip].clone();
        for i in (1..=9).rev() {
            let mut new_alu = alu.clone();
            new_alu.step(&line, Some(&i.to_string()));
            for _ in 0..17 {
                let line = self.instructions[new_alu.ip].clone();
                new_alu.step(&line, None);
            }
            if let Some(n) = self.memo_analyze_step(new_alu) {
                return Some(i.to_string() + &n);
            }
        }
        None
    }

    fn run(&mut self, input: &str) -> isize {
        let mut alu = ALU::new();
        let mut digits = input.chars();
        for line in &self.instructions {
            if line.starts_with("inp") {
                alu.step(line, Some(&String::from(digits.next().unwrap())));
            } else {
                alu.step(line, None);
            }
        }
        alu.z
    }
}

fn calc1(input: &str) -> usize {
    let mut p = Program::from(input);
    println!("{}", p.analyze());
    // println!("{}", p.run("99997391969649"));
    0
}

// fn calc2(input: &str) -> usize {
//     0
// }
