use itertools::Itertools;
use std::io::BufRead;

struct Computer {
    state: ComputerState,

    instructions: Vec<u64>,
    output: Vec<u8>,
}

struct ComputerState {
    pc: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

impl Computer {
    pub fn run_program(&mut self) {
        while self.state.pc < self.instructions.len() {
            let instruction = self.instructions[self.state.pc];
            let operand = self.instructions[self.state.pc + 1];

            if let Some(output) = self.state.run_single_step(instruction, operand) {
                self.output.push(output);
            }
        }
    }
}

impl ComputerState {
    pub fn run_single_step(&mut self, instruction: u64, operand: u64) -> Option<u8> {
        match instruction {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => {
                let res = self.out(operand);
                return Some(res);
            }
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => unreachable!(),
        };

        None
    }
    fn adv(&mut self, operand: u64) {
        self.reg_a /= 2u64.pow(self.combo(operand) as u32);
        self.increment();
    }

    fn bxl(&mut self, operand: u64) {
        self.reg_b ^= operand;
        self.increment();
    }

    fn bst(&mut self, operand: u64) {
        self.reg_b = self.combo(operand) % 8;
        self.increment();
    }

    fn jnz(&mut self, operand: u64) {
        if self.reg_a != 0 {
            self.pc = operand as usize;
        } else {
            self.increment();
        }
    }

    fn bxc(&mut self, _operand: u64) {
        self.reg_b ^= self.reg_c;
        self.increment();
    }

    fn out(&mut self, operand: u64) -> u8 {
        self.increment();
        (self.combo(operand) % 8) as u8
    }

    fn bdv(&mut self, operand: u64) {
        self.reg_b = self.reg_a / 2u64.pow(self.combo(operand) as u32);
        self.increment();
    }

    fn cdv(&mut self, operand: u64) {
        self.reg_c = self.reg_a / 2u64.pow(self.combo(operand) as u32);
        self.increment();
    }

    fn increment(&mut self) {
        self.pc += 2;
    }

    fn combo(&self, val: u64) -> u64 {
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
pub fn run<R>(reader: R) -> (String, u64)
where
    R: BufRead,
{
    let mut lines = reader.lines();
    let reg_a = parse_reg(&mut lines.next().unwrap().unwrap());
    let reg_b = parse_reg(&mut lines.next().unwrap().unwrap());
    let reg_c = parse_reg(&mut lines.next().unwrap().unwrap());
    lines.next();
    let instructions = parse_opcodes(&mut lines.next().unwrap().unwrap());

    let a_value = 0u64;
    let res = loop_de_loop(&instructions, a_value, instructions.len());
    let res_p2 = res.unwrap();

    let mut computer = Computer {
        state: ComputerState {
            pc: 0,
            reg_a,
            reg_b,
            reg_c,
        },
        instructions,
        output: Vec::new(),
    };

    computer.run_program();

    let res = computer
        .output
        .iter()
        .map(|val| ((val + b'0') as char).to_string())
        .join(",");

    (res, res_p2)
}

fn loop_de_loop(instructions: &[u64], a_value: u64, curr_check_idx: usize) -> Option<u64> {
    if curr_check_idx == 0 {
        return Some(a_value);
    }
    let a_value = a_value << 3;
    let instr = instructions[curr_check_idx - 1];
    for test in 0..8 {
        let test_reg_a = a_value + test;
        let mut state = ComputerState {
            pc: 0,
            reg_a: test_reg_a,
            reg_b: 0,
            reg_c: 0,
        };

        let output = loop {
            let instr = instructions[state.pc];
            let op = instructions[state.pc + 1];
            if let Some(output) = state.run_single_step(instr, op) {
                break output;
            }
        };

        if output as u64 == instr {
            if let Some(a_val) = loop_de_loop(&instructions, test_reg_a, curr_check_idx - 1) {
                return Some(a_val);
            }
        }
    }
    None
}

fn parse_reg(register_line: &str) -> u64 {
    register_line
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_opcodes(opcode_line: &str) -> Vec<u64> {
    opcode_line
        .split_whitespace()
        .last()
        .unwrap()
        .split(",")
        .map(|val| val.parse::<u64>().unwrap())
        .collect()
}
