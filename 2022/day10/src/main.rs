use std::{fs::read_to_string, path::Path};

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop { cycles: u32 },
    AddX { value: i32, cycles: u32 },
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
struct CPU {
    x: i32,
    program_counter: Option<Instruction>,
}

impl CPU {
    fn new() -> Self {
        Self {
            x: 1,
            program_counter: None,
        }
    }

    fn execute(&mut self) {
        if let Some(instruction) = self.program_counter {
            match instruction {
                Instruction::Noop { mut cycles } => {
                    cycles -= 1;
                    if cycles == 0 {
                        self.program_counter = None;
                    } else {
                        self.program_counter = Some(Instruction::Noop { cycles });
                    }
                }
                Instruction::AddX { mut cycles, value } => {
                    cycles -= 1;
                    if cycles == 0 {
                        self.x += value;
                        self.program_counter = None;
                    } else {
                        self.program_counter = Some(Instruction::AddX { cycles, value });
                    }
                }
            }
        } else {
            panic!("No instruction to execute");
        }
    }
}

fn simulate_cpu(path: &Path) {
    println!("----------------------------------");
    println!("Simulating CPU with instructions from {}", path.display());

    let input = read_to_string(path).unwrap();

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let instruction = parts.next().unwrap();

            if instruction == "addx" {
                let value = parts.next().unwrap().parse::<i32>().unwrap();
                Instruction::AddX { value, cycles: 2 }
            } else if instruction == "noop" {
                Instruction::Noop { cycles: 1 }
            } else {
                panic!("Unknown instruction: {}", instruction);
            }
        })
        .collect();

    let mut cpu = CPU::new();

    let mut global_cycle_counter = 0;
    let mut signal_strengths = [20, 60, 100, 140, 180, 220];
    let mut signal_index = 0;

    let instruction_iter = instructions.iter();
    for instruction in instruction_iter {
        cpu.program_counter = Some(*instruction);
        while cpu.program_counter.is_some() {
            cpu.execute();
            global_cycle_counter += 1;

            if signal_index < signal_strengths.len()
                && global_cycle_counter == signal_strengths[signal_index]
            {
                println!("x = {}", cpu.x);
                signal_strengths[signal_index] *= cpu.x;
                signal_index += 1;
            }
        }
    }

    let sum: i32 = signal_strengths.iter().sum();
    println!("Signals = {:?}", signal_strengths);
    println!("Sum of signal strengths: {}", sum);
}

fn main() {
    simulate_cpu(Path::new("test.txt"));
    simulate_cpu(Path::new("input.txt"));
}
