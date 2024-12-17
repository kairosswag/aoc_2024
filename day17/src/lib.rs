use itertools::Itertools;
use std::io::BufRead;

struct Computer {
    pc: usize,
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,

    instructions: Vec<u32>,
    output: Vec<u8>,
}

impl Computer {
    pub fn run_program(&mut self) {
        while self.pc < self.instructions.len() {
            let instruction = self.instructions[self.pc];
            let operand = self.instructions[self.pc + 1];

            match instruction {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => unreachable!(),
            }
        }
    }

    fn adv(&mut self, operand: u32) {
        self.reg_a /= 2u32.pow(self.combo(operand));
        self.increment();
    }

    fn bxl(&mut self, operand: u32) {
        self.reg_b ^= operand;
        self.increment();
    }

    fn bst(&mut self, operand: u32) {
        self.reg_b = self.combo(operand) % 8;
        self.increment();
    }

    fn jnz(&mut self, operand: u32) {
        if self.reg_a != 0 {
            self.pc = operand as usize;
        } else {
            self.increment();
        }
    }

    fn bxc(&mut self, _operand: u32) {
        self.reg_b ^= self.reg_c;
        self.increment();
    }

    fn out(&mut self, operand: u32) {
        self.output.push((self.combo(operand) % 8) as u8);
        self.increment();
    }

    fn bdv(&mut self, operand: u32) {
        self.reg_b = self.reg_a / 2u32.pow(self.combo(operand));
        self.increment();
    }

    fn cdv(&mut self, operand: u32) {
        self.reg_c = self.reg_a / 2u32.pow(self.combo(operand));
        self.increment();
    }

    fn increment(&mut self) {
        self.pc += 2;
    }

    fn combo(&self, val: u32) -> u32 {
        match val {
            0..=3 => val,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => unreachable!("Invalid combo! {}", val),
            _ => unreachable!("Invalid opcode! {}", val),
        }
    }
}
pub fn run<R>(reader: R) -> (String, usize)
where
    R: BufRead,
{
    let mut lines = reader.lines();
    let reg_a = parse_reg(&mut lines.next().unwrap().unwrap());
    let reg_b = parse_reg(&mut lines.next().unwrap().unwrap());
    let reg_c = parse_reg(&mut lines.next().unwrap().unwrap());
    lines.next();
    let instructions = parse_opcodes(&mut lines.next().unwrap().unwrap());

    let mut computer = Computer {
        pc: 0,
        reg_a,
        reg_b,
        reg_c,
        instructions,
        output: Vec::new(),
    };

    computer.run_program();

    let res = computer
        .output
        .iter()
        .map(|val| ((val + b'0') as char).to_string())
        .join(",");

    (res, 5)
}

fn parse_reg(register_line: &str) -> u32 {
    register_line
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_opcodes(opcode_line: &str) -> Vec<u32> {
    opcode_line
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|val| val.parse::<u32>().unwrap())
        .collect()
}
