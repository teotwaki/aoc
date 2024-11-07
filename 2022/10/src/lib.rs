use common::Answer;
use std::{collections::VecDeque, fmt::Display, str::FromStr};

enum Instruction {
    Addx(i16),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        Ok(match parts[0] {
            "addx" => Instruction::Addx(parts[1].parse().unwrap()),
            "noop" => Instruction::Noop,
            _ => unreachable!(),
        })
    }
}

struct Crt {
    lines: Vec<Vec<bool>>,
    pos: (usize, usize),
}

impl Crt {
    fn sync(&mut self, x: i16) {
        if let Ok(x) = x.try_into() {
            self.lines[self.pos.0].push(self.pos.1 >= x && self.pos.1 <= x + 2);
        } else {
            self.lines[self.pos.0].push(false);
        }

        self.pos = (self.pos.0, self.pos.1 + 1);

        if self.pos.1 == 40 {
            self.lines.push(vec![]);
            self.pos.0 += 1;
            self.pos.1 = 0;
        }
    }

    fn new() -> Self {
        Self {
            lines: vec![vec![]],
            pos: (0, 0),
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            f.write_str(
                &line
                    .iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>(),
            )
            .unwrap();
            f.write_str("\n").unwrap();
        }

        Ok(())
    }
}

struct Cpu {
    instructions: VecDeque<Instruction>,
    x: i16,
    cycle: i16,
    breakpoints: [i16; 6],
    signal_strengths: Vec<i16>,
    crt: Crt,
}

impl Cpu {
    fn tick(&mut self) {
        self.cycle += 1;

        self.crt.sync(self.x - 1);

        if self.breakpoints.contains(&self.cycle) {
            self.signal_strengths.push(self.cycle * self.x);
        }
    }

    fn run(&mut self) {
        loop {
            match self.instructions.pop_front() {
                Some(Instruction::Addx(i)) => {
                    self.tick();
                    self.tick();
                    self.x += i;
                }
                Some(Instruction::Noop) => self.tick(),
                None => break,
            }
        }
    }

    fn sss(&self) -> i16 {
        self.signal_strengths.iter().sum()
    }

    fn new(instructions: VecDeque<Instruction>) -> Cpu {
        Self {
            instructions,
            x: 1,
            cycle: 0,
            breakpoints: [20, 60, 100, 140, 180, 220],
            signal_strengths: vec![],
            crt: Crt::new(),
        }
    }

    fn crt(&self) -> &Crt {
        &self.crt
    }
}

fn simulate_cpu(s: &str) -> Cpu {
    let instructions: VecDeque<Instruction> = s
        .lines()
        .map(|l| l.parse().expect("Couldn't parse line as Instruction"))
        .collect();

    let mut cpu = Cpu::new(instructions);

    cpu.run();

    cpu
}

pub fn step1(s: &str) -> Answer {
    simulate_cpu(s).sss().into()
}

pub fn step2(s: &str) -> Answer {
    let s: String = simulate_cpu(s).crt().to_string();

    s.into()
}
