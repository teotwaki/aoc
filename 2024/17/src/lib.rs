use common::Answer;

type Register = u64;
type Operand = u8;

#[derive(Debug, Clone, Copy)]
struct LiteralOperand(Operand);

impl LiteralOperand {
    fn value(&self) -> Register {
        self.0 as Register
    }
}

#[derive(Debug, Clone, Copy)]
struct ComboOperand(Operand);

impl ComboOperand {
    fn value(&self, computer: &Computer) -> Register {
        match self.0 {
            4 => computer.a,
            5 => computer.b,
            6 => computer.c,
            7 => unreachable!(),
            n => n as Register,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl From<(u8, u8)> for Instruction {
    fn from(value: (u8, u8)) -> Self {
        use Instruction::*;

        match value.0 {
            0 => Adv(ComboOperand(value.1)),
            1 => Bxl(LiteralOperand(value.1)),
            2 => Bst(ComboOperand(value.1)),
            3 => Jnz(LiteralOperand(value.1)),
            4 => Bxc,
            5 => Out(ComboOperand(value.1)),
            6 => Bdv(ComboOperand(value.1)),
            7 => Cdv(ComboOperand(value.1)),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Computer<'a> {
    a: Register,
    b: Register,
    c: Register,

    instructions: &'a [Instruction],
    instr_ptr: usize,

    output: Vec<u8>,
}

impl<'a> Computer<'a> {
    #[must_use]
    fn new(a: Register, b: Register, c: Register, instructions: &'a [Instruction]) -> Self {
        Self {
            a,
            b,
            c,

            instructions,
            instr_ptr: 0,

            output: vec![],
        }
    }

    fn advance(&mut self) {
        self.instr_ptr += 1;
    }

    fn adv(&mut self, operand: Register) {
        self.a /= 2u64.pow(operand as u32);
        self.advance();
    }

    fn bdv(&mut self, operand: Register) {
        self.b = self.a / 2u64.pow(operand as u32);
        self.advance();
    }

    fn cdv(&mut self, operand: Register) {
        self.c = self.a / 2u64.pow(operand as u32);
        self.advance();
    }

    fn bxl(&mut self, operand: Register) {
        self.b ^= operand;
        self.advance();
    }

    fn bst(&mut self, operand: Register) {
        self.b = operand & 7;
        self.advance();
    }

    fn jnz(&mut self, operand: Register) {
        if self.a != 0 {
            self.instr_ptr = operand as usize;
        } else {
            self.advance();
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c;
        self.advance();
    }

    fn out(&mut self, operand: Register) {
        self.output.push((operand & 7) as u8);
        self.advance();
    }

    fn execute(&mut self) {
        use Instruction::*;

        while self.instr_ptr < self.instructions.len() {
            match self.instructions[self.instr_ptr] {
                Adv(n) => self.adv(n.value(self)),
                Bxl(n) => self.bxl(n.value()),
                Bst(n) => self.bst(n.value(self)),
                Jnz(n) => self.jnz(n.value()),
                Bxc => self.bxc(),
                Out(n) => self.out(n.value(self)),
                Bdv(n) => self.bdv(n.value(self)),
                Cdv(n) => self.cdv(n.value(self)),
            }
        }
    }

    #[must_use]
    fn print_output(&self) -> String {
        let mut output = String::new();

        for i in self.output.iter() {
            output.push(',');
            output.push_str(&i.to_string());
        }

        output[1..].to_string()
    }
}

fn parse_register(s: &str) -> Register {
    s.split_whitespace()
        .nth(2)
        .and_then(|s| s.parse::<Register>().ok())
        .unwrap()
}

fn parse_program(s: &str) -> Vec<Instruction> {
    s.split(',')
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|ops| {
            let opcode = ops[0].parse::<u8>().unwrap();
            let operand = ops[1].parse::<u8>().unwrap();

            (opcode, operand).into()
        })
        .collect::<Vec<_>>()
}

fn parse(s: &str) -> (Register, Register, Register, Vec<Instruction>) {
    let mut lines = s.lines();

    let a = parse_register(lines.next().unwrap());
    let b = parse_register(lines.next().unwrap());
    let c = parse_register(lines.next().unwrap());

    let instructions = parse_program(
        lines
            .nth(1)
            .and_then(|s| s.split_whitespace().nth(1))
            .unwrap(),
    );

    (a, b, c, instructions)
}

pub fn step1(s: &str) -> Answer {
    let (a, b, c, instructions) = parse(s);
    let mut computer = Computer::new(a, b, c, &instructions);
    computer.execute();

    computer.print_output().into()
}

pub fn step2(s: &str) -> Answer {
    let program = s
        .split("\n\n")
        .nth(1)
        .and_then(|s| s.split_whitespace().nth(1))
        .unwrap();
    let instructions = parse_program(program);
    let output_digits = program
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut register = 0;

    output_digits.iter().for_each(|_| {
        register *= 8;
        let mut i = 0;
        loop {
            let mut computer = Computer::new(register + i, 0, 0, &instructions);
            computer.execute();

            if computer.output == output_digits[output_digits.len() - computer.output.len()..] {
                register += i;
                break;
            }

            i += 1;
        }
    });

    register.into()
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;

    const INPUT: &str = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

    const EMPTY_INSTR: [Instruction; 0] = [];

    const COMPUTER: Computer = Computer {
        a: 10,
        b: 20,
        c: 30,
        instructions: &EMPTY_INSTR,
        instr_ptr: 0,
        output: vec![],
    };

    #[test]
    fn step1_finds_correct_output_value() {
        assert_eq!(
            step1(INPUT),
            Answer::Text("4,6,3,5,6,3,5,2,1,0".to_string())
        );
    }

    #[parameterized(
        input = { 0, 1, 2, 3, 4, 5, 6 },
        result = { 0, 1, 2, 3, 10, 20, 30 }
    )]
    fn combooperands_provide_correct_values(input: u8, result: Register) {
        assert_eq!(ComboOperand(input).value(&COMPUTER), result);
    }

    #[test]
    fn computer_handles_bst() {
        let mut c = Computer {
            a: 0,
            b: 0,
            c: 9,
            instructions: &EMPTY_INSTR,
            instr_ptr: 0,
            output: vec![],
        };

        c.bst(9);

        assert_eq!(c.b, 1);
    }

    #[test]
    fn computer_handles_bxl() {
        let mut c = Computer {
            a: 0,
            b: 29,
            c: 0,
            instructions: &EMPTY_INSTR,
            instr_ptr: 0,
            output: vec![],
        };

        c.bxl(7);

        assert_eq!(c.b, 26);
    }

    #[test]
    fn computer_handles_bxc() {
        let mut c = Computer {
            a: 0,
            b: 2024,
            c: 43690,
            instructions: &EMPTY_INSTR,
            instr_ptr: 0,
            output: vec![],
        };

        c.bxc();

        assert_eq!(c.b, 44354);
    }

    #[test]
    fn step1_finds_correct_second_example() {
        let input = r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#;

        assert_eq!(step1(input), Answer::Text("0,1,2".to_string()))
    }

    #[test]
    fn step1_finds_correct_third_example() {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;

        assert_eq!(
            step1(input),
            Answer::Text("4,2,5,6,7,7,7,7,3,1,0".to_string())
        )
    }

    #[test]
    fn verify_step2() {
        let input = r#"Register A: 216584205979245
Register B: 0
Register C: 0

Program: 2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0"#;

        assert_eq!(
            step1(input),
            Answer::Text("2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0".to_string())
        )
    }
}
