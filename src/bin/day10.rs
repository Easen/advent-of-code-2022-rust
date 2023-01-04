use std::collections::VecDeque;

enum CPUInstruction {
    NOOP,
    AddX(i32),
}

impl CPUInstruction {
    pub fn from_str(input: &str) -> Vec<CPUInstruction> {
        input
            .lines()
            .map(|l| {
                let tokens: Vec<&str> = l.split(" ").collect();
                return match tokens[0] {
                    "noop" => CPUInstruction::NOOP,
                    "addx" => CPUInstruction::AddX(tokens[1].parse::<i32>().unwrap()),
                    &_ => todo!(),
                };
            })
            .collect()
    }

    pub fn cycles(&self) -> usize {
        match self {
            CPUInstruction::NOOP => 1,
            CPUInstruction::AddX(_) => 2,
        }
    }
}

type SampledSignalStrength = Vec<i32>;

#[derive(Clone, Copy)]
struct CPU {
    register_x: i32,
    cycle: usize,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            register_x: 1,
            cycle: 0,
        }
    }

    pub fn execute_program(&mut self, instructions: Vec<CPUInstruction>) -> SampledSignalStrength {
        let sample_points = [20, 60, 100, 140, 180, 220];
        let mut samples = vec![];
        let mut instructions = VecDeque::from_iter(instructions.iter());
        while let Some(instruction) = instructions.pop_front() {
            for _ in 0..instruction.cycles() {
                self.cycle += 1;
                if sample_points.contains(&self.cycle) {
                    samples.push(self.cycle as i32 * self.register_x);
                }
            }
            match instruction {
                CPUInstruction::NOOP => (),
                CPUInstruction::AddX(x) => self.register_x += x,
            }
        }
        return samples;
    }
}

fn main() {
    let input = include_str!("../../inputs/10.txt");
    let instructions: Vec<CPUInstruction> = CPUInstruction::from_str(input);
    let mut cpu = CPU::new();
    let signal_strength = cpu.execute_program(instructions);
    println!("part1: {}", signal_strength.iter().sum::<i32>())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let instructions: Vec<CPUInstruction> = CPUInstruction::from_str(input);
        let mut cpu = CPU::new();
        let samples = cpu.execute_program(instructions);
        assert_eq!(samples, vec![420, 1140, 1800, 2940, 2880, 3960]);
    }
}
