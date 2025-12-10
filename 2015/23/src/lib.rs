use common::Answer;

type IntType = u32;

#[derive(Debug, Clone)]
struct Computer {
    instructions: Vec<Instruction>,
    current: usize,
    a: IntType,
    b: IntType,
}

impl Computer {
    pub fn new(reg_a: IntType, instructions: Vec<Instruction>) -> Self {
        Self {
            a: reg_a,
            b: 0,
            instructions,
            current: 0,
        }
    }

    pub fn run(&mut self) {
        while self.current < self.instructions.len() {
            match self.instructions[self.current] {
                Instruction::Half(reg) => match reg {
                    Register::A => self.a /= 2,
                    Register::B => self.b /= 2,
                },
                Instruction::Triple(reg) => match reg {
                    Register::A => self.a *= 3,
                    Register::B => self.b *= 3,
                },
                Instruction::Increment(reg) => match reg {
                    Register::A => self.a += 1,
                    Register::B => self.b += 1,
                },
                Instruction::Jump(offset) => {
                    self.current = (self.current as isize + offset as isize) as usize;
                    continue;
                }
                Instruction::JumpIfEven(reg, offset) => {
                    let value = match reg {
                        Register::A => self.a,
                        Register::B => self.b,
                    };

                    if value.is_multiple_of(2) {
                        self.current = (self.current as isize + offset as isize) as usize;
                        continue;
                    }
                }
                Instruction::JumpIfOne(reg, offset) => {
                    let value = match reg {
                        Register::A => self.a,
                        Register::B => self.b,
                    };
                    if value == 1 {
                        self.current = (self.current as isize + offset as isize) as usize;
                        continue;
                    }
                }
            }
            self.current += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Register {
    A,
    B,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i8),
    JumpIfEven(Register, i8),
    JumpIfOne(Register, i8),
}

fn parse(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|l| match &l[0..3] {
            "hlf" => {
                if &l[4..5] == "a" {
                    Instruction::Half(Register::A)
                } else {
                    Instruction::Half(Register::B)
                }
            }
            "tpl" => {
                if &l[4..5] == "a" {
                    Instruction::Triple(Register::A)
                } else {
                    Instruction::Triple(Register::B)
                }
            }
            "inc" => {
                if &l[4..5] == "a" {
                    Instruction::Increment(Register::A)
                } else {
                    Instruction::Increment(Register::B)
                }
            }
            "jmp" => {
                let offset = l[4..].parse().unwrap();
                Instruction::Jump(offset)
            }
            "jie" => {
                let offset = l[7..].parse().unwrap();
                if &l[4..5] == "a" {
                    Instruction::JumpIfEven(Register::A, offset)
                } else {
                    Instruction::JumpIfEven(Register::B, offset)
                }
            }
            "jio" => {
                let offset = l[7..].parse().unwrap();
                if &l[4..5] == "a" {
                    Instruction::JumpIfOne(Register::A, offset)
                } else {
                    Instruction::JumpIfOne(Register::B, offset)
                }
            }
            _ => unreachable!(),
        })
        .collect()
}

fn run_and_get_reg(reg_a: IntType, s: &str, reg: Register) -> Answer {
    let instructions = parse(s);
    let mut cpu = Computer::new(reg_a, instructions);

    cpu.run();

    let val = match reg {
        Register::A => cpu.a,
        Register::B => cpu.b,
    };
    Answer::Unsigned(val as u64)
}

pub fn step1(s: &str) -> Answer {
    run_and_get_reg(0, s, Register::B)
}

pub fn step2(s: &str) -> Answer {
    run_and_get_reg(1, s, Register::B)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"inc a
jio a, +2
tpl a
inc a"#;

    #[test]
    fn parse_extracts_correct_number_of_lines() {
        assert_eq!(parse(INPUT).len(), 4);
    }

    #[test]
    fn step1_works_on_example_input() {
        let answer = run_and_get_reg(0, INPUT, Register::A);
        assert_eq!(answer, Answer::Unsigned(2));
    }
}
