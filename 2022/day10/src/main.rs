use std::{fs::read_to_string, path::Path};

// Define the possible instructions
#[derive(Clone, Copy, Debug)]
enum Instruction {
    Noop { cycles: u32 },
    AddX { value: i32, cycles: u32 },
}

// Define the CPU structure
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
struct CPU {
    x: i32,
    current_instruction: Option<Instruction>,
}

impl CPU {
    // Initialize the CPU
    fn new() -> Self {
        Self {
            x: 1,
            current_instruction: None,
        }
    }

    // Execute the current instruction
    fn execute(&mut self) {
        if let Some(instruction) = self.current_instruction {
            match instruction {
                Instruction::Noop { mut cycles } => {
                    cycles -= 1;
                    if cycles == 0 {
                        self.current_instruction = None;
                    } else {
                        self.current_instruction = Some(Instruction::Noop { cycles });
                    }
                }
                Instruction::AddX { mut cycles, value } => {
                    cycles -= 1;
                    if cycles == 0 {
                        self.x += value;
                        self.current_instruction = None;
                    } else {
                        self.current_instruction = Some(Instruction::AddX { cycles, value });
                    }
                }
            }
        } else {
            panic!("No instruction to execute");
        }
    }
}

// Parse instructions from the input file
fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
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
        .collect()
}

// Run the CPU simulation
fn simulate_cpu(path: &Path) {
    println!("----------------------------------");
    println!("Simulating CPU with instructions from {}", path.display());

    // Read input from the file and parse the instructions
    let input = read_to_string(path).unwrap();
    let instructions = parse_instructions(&input);

    // Initialize the CPU
    let mut cpu = CPU::new();
    let mut global_cycle_counter = 0;
    let mut signal_strengths = [20, 60, 100, 140, 180, 220];
    let mut signal_index = 0;

    // Execute each instruction in the list
    for instruction in instructions.iter() {
        // Set the current instruction in the CPU
        cpu.current_instruction = Some(*instruction);

        // Execute the current instruction
        while cpu.current_instruction.is_some() {
            // Increment the global cycle counter
            global_cycle_counter += 1;

            // Check if a signal needs to be modified
            if signal_index < signal_strengths.len()
                && global_cycle_counter == signal_strengths[signal_index]
            {
                println!("x = {}", cpu.x);
                signal_strengths[signal_index] *= cpu.x;
                signal_index += 1;
            }

            // Execute the current instruction
            cpu.execute();
        }
    }

    // Calculate and display the sum of signal strengths
    let sum: i32 = signal_strengths.iter().sum();
    println!("Signals = {:?}", signal_strengths);
    println!("Sum of signal strengths: {}", sum);
}

fn main() {
    simulate_cpu(Path::new("test.txt"));
    simulate_cpu(Path::new("input.txt"));
}
