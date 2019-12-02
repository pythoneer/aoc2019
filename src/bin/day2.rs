use std::error::Error;
use std::num::ParseIntError;


struct Cpu {
    memory: Vec<u32>,
    ip: usize,
}

impl Cpu {
    pub fn init(input: &str) -> Result<Cpu, ParseIntError> {
        Ok(Cpu {
            memory: parse_input(input)?,
            ip: 0,
        })
    }

    pub fn run(&mut self) -> u32 {
        while self.eval() {}
        self.memory[0]
    }

    pub fn patch(&mut self, idx: usize, value: u32) {
        self.memory[idx] = value;
    }

    pub fn eval(&mut self) -> bool {
        let opcode = self.memory[self.ip];
        match opcode {
            99 => false,
            1 | 2 => {
                let output_address = self.memory[self.ip + 3] as usize;
                let param1 = self.memory[self.memory[self.ip + 1] as usize];
                let param2 = self.memory[self.memory[self.ip + 2] as usize];

                self.memory[output_address] = if opcode == 1 { param1 + param2 } else { param1 * param2 };
                self.ip += 4;
                true
            }
            _ => { panic!("invalid opcode {}", opcode) }
        }
    }
}

pub fn parse_input(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split(',').map(str::parse).collect()
}

pub fn part1(input: &str, use_patch: bool) -> String {
    let mut cpu = Cpu::init(input).unwrap();
    if use_patch {
        cpu.patch(1, 12);
        cpu.patch(2, 2);
    }
    cpu.run().to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../../input/day2.txt");

    println!("part1: {}", part1(input, true));

    Ok(())
}

#[test]
fn cpu_test_part1() {
    assert_eq!("3500", part1("1,9,10,3,2,3,11,0,99,30,40,50", false));
    assert_eq!("2", part1("1,0,0,0,99", false));
    assert_eq!("2", part1("2,3,0,3,99", false));
    assert_eq!("2", part1("2,4,4,5,99,0", false));
    assert_eq!("30", part1("1,1,1,4,99,5,6,0,99", false));
}