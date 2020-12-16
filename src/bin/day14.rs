use aoc::{Parse, Result, Solve};
use reformation::Reformation;
use std::collections::HashMap;

#[derive(Debug, Clone, Reformation)]
enum Instr<'a> {
    #[reformation("mem[{}] = {}", no_regex = true)]
    Write(u64, u64),
    #[reformation("mask = {}")]
    SetMask(&'a str),
}

#[derive(Debug)]
struct Program<'a> {
    instrs: Vec<Instr<'a>>,
}

impl<'a> Parse<'a> for Program<'a> {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let lines = input_str.lines().map(str::trim);
        let instrs = lines.map(Instr::parse).collect::<Result<_, _>>()?;
        Ok(Program { instrs })
    }
}

trait Memory {
    fn set_mask(&mut self, mask: &str) -> Result<()>;
    fn write(&mut self, addr: u64, val: u64) -> Result<()>;
    fn sum_contents(&self) -> u64;
}

#[derive(Debug)]
struct Machine<'a, M> {
    program: &'a Program<'a>,
    mask: &'a str,
    mem: M,
}

impl<'a, M: Memory> Machine<'a, M> {
    fn new(program: &'a Program, mem: M) -> Self {
        Self {
            program,
            mask: "",
            mem,
        }
    }

    fn run(&mut self) -> Result<()> {
        for instr in &self.program.instrs {
            match *instr {
                Instr::Write(addr, val) => {
                    self.mem.write(addr, val)?;
                }
                Instr::SetMask(mask) => {
                    self.mem.set_mask(mask)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct MemV1 {
    mask_zeros: u64,
    mask_ones: u64,
    contents: HashMap<u64, u64>,
}

impl Memory for MemV1 {
    fn set_mask(&mut self, mask: &str) -> Result<()> {
        self.mask_zeros = u64::from_str_radix(&mask.replace("X", "1"), 2)?;
        self.mask_ones = u64::from_str_radix(&mask.replace("X", "0"), 2)?;
        Ok(())
    }

    fn write(&mut self, addr: u64, mut val: u64) -> Result<()> {
        val &= self.mask_zeros;
        val |= self.mask_ones;
        self.contents.insert(addr, val);
        Ok(())
    }

    fn sum_contents(&self) -> u64 {
        self.contents.values().sum()
    }
}

struct PartOne;

impl<'a> Solve<'a> for PartOne {
    type Input = Program<'a>;
    type Solution = u64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut mach = Machine::new(input, MemV1::default());
        mach.run()?;
        Ok(mach.mem.sum_contents())
    }
}

#[derive(Debug, Default)]
struct MemV2 {
    mask: String,
    contents: HashMap<u64, u64>,
}

impl Memory for MemV2 {
    fn set_mask(&mut self, mask: &str) -> Result<()> {
        self.mask = mask.to_string();
        Ok(())
    }

    fn write(&mut self, addr: u64, val: u64) -> Result<()> {
        fn write_rec(
            mem: &mut HashMap<u64, u64>,
            mask: &str,
            addr: u64,
            val: u64,
            bit: usize,
        ) -> Result<()> {
            match mask.chars().nth(bit) {
                Some('0') => write_rec(mem, mask, addr, val, bit + 1),
                Some('1') => write_rec(mem, mask, addr | (1 << (35 - bit)), val, bit + 1),
                Some('X') => {
                    write_rec(mem, mask, addr & !(1 << (35 - bit)), val, bit + 1)?;
                    write_rec(mem, mask, addr | (1 << (35 - bit)), val, bit + 1)
                }
                _ => {
                    mem.insert(addr, val);
                    Ok(())
                }
            }
        }
        write_rec(&mut self.contents, &self.mask, addr, val, 0)
    }

    fn sum_contents(&self) -> u64 {
        self.contents.values().sum()
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = Program<'a>;
    type Solution = u64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut mach = Machine::new(input, MemV2::default());
        mach.run()?;
        Ok(mach.mem.sum_contents())
    }
}

aoc::main!(day14);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = Program::parse(indoc! {"
            mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 165);
    }

    #[test]
    fn example_part_two() {
        let input = Program::parse(indoc! {"
            mask = 000000000000000000000000000000X1001X
            mem[42] = 100
            mask = 00000000000000000000000000000000X0XX
            mem[26] = 1
        "})
        .unwrap();

        assert_eq!(PartTwo::solve(&input).unwrap(), 208);
    }
}

aoc::solved!(day14, PartOne = 8566770985168, PartTwo = 4832039794082);
