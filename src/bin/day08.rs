use anyhow::bail;
use aoc::{self, Error, Result, Solve};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, parse_display::FromStr)]
enum Instr {
    #[display("acc {0}")]
    Acc(isize),
    #[display("jmp {0}")]
    Jmp(isize),
    #[display("nop {0}")]
    Nop(isize),
}

#[derive(Clone, Debug)]
struct Program {
    instrs: Vec<Instr>,
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::trim);
        let instrs = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Program { instrs })
    }
}

struct InfiniteLoop;

struct Machine {
    accum: isize,
    instrs: Vec<Instr>,
    pc: usize,
    seen: Vec<bool>,
}

impl Machine {
    fn new(instrs: &[Instr]) -> Machine {
        Machine {
            accum: 0,
            instrs: instrs.to_vec(),
            pc: 0,
            seen: vec![false; instrs.len()],
        }
    }

    fn step(&mut self) -> usize {
        match self.instrs[self.pc] {
            Instr::Acc(arg) => {
                self.accum += arg;
                self.pc += 1;
            }
            Instr::Jmp(arg) => {
                self.pc = (self.pc as isize + arg) as usize;
            }
            Instr::Nop(_) => {
                self.pc += 1;
            }
        }
        self.pc
    }

    fn run(&mut self) -> Result<(), InfiniteLoop> {
        loop {
            let next = self.step();
            if next == self.instrs.len() {
                return Ok(());
            } else if self.seen[next] {
                return Err(InfiniteLoop);
            }
            self.seen[next] = true;
        }
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = Program;
    type Solution = isize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut machine = Machine::new(&input.instrs);
        match machine.run() {
            Err(InfiniteLoop) => return Ok(machine.accum),
            Ok(()) => bail!("machine failed to infinite loop"),
        }
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = Program;
    type Solution = isize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        for index in 0..input.instrs.len() {
            let mut input = input.instrs.clone();
            input[index] = match input[index] {
                Instr::Acc(arg) => Instr::Acc(arg),
                Instr::Jmp(arg) => Instr::Nop(arg),
                Instr::Nop(arg) => Instr::Jmp(arg),
            };

            let mut machine = Machine::new(&input);
            match machine.run() {
                Ok(()) => return Ok(machine.accum),
                Err(InfiniteLoop) => continue,
            }
        }
        bail!("machine always infinite loops");
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input: Program = indoc! {"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "}
        .parse()
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 5);
        assert_eq!(PartTwo::solve(&input).unwrap(), 8);
    }
}

aoc::solved! {
    PartOne = 1563,
    PartTwo = 767,
}
