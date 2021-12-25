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

#[derive(Debug)]
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
    }

    fn run(&mut self, input: &str, program: &str) {
        self.reset();
        let program = program.lines();
        let input: Vec<&str> = input.split("").skip(1).collect();
        let mut input = input.iter();
        for line in program {
            let args: Vec<&str> = line.split_whitespace().collect();
            let op = args[0];
            let a = ALU::convert_operand(args[1]);
            let b = ALU::convert_operand(args.get(2).unwrap_or_else(|| input.next().unwrap()));
            match op {
                "inp" => self.inp(a, b),
                "add" => self.add(a, b),
                "mul" => self.mul(a, b),
                "div" => self.div(a, b),
                "mod" => self.modulo(a, b),
                "eql" => self.eql(a, b),
                _ => panic!(),
            }
        }
    }

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
    }

    fn write(&mut self, register: char, value: isize) {
        // println!("{:?}", self);
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
    instructions: String,
    cache: HashMap<ALU, isize>,
}

impl Program {
    fn from(input: &str) -> Self {
        Self {
            instructions: String::from(input),
            cache: HashMap::new(),
        }
    }

    fn analyze(&self) {}

    fn analyze_step(&self, alu: ALU, ip: usize) {}

    fn run(&mut self, alu: &mut ALU, input: usize) -> isize {
        alu.run(&input.to_string(), &self.instructions);
        alu.z
    }
}

fn calc1(input: &str) -> usize {
    let mut p = Program::from(input);
    0
}

// fn calc2(input: &str) -> usize {
//     0
// }
