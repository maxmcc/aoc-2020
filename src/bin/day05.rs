use anyhow::{anyhow, bail};
use aoc::{self, Error, Result, Solve};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct BoardingPass {
    id: u32,
}

impl FromStr for BoardingPass {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let binary = input
            .chars()
            .map(|c| match c {
                'F' | 'L' => Ok('0'),
                'B' | 'R' => Ok('1'),
                _ => bail!("unexpected char {:?}", c),
            })
            .collect::<Result<String>>()?;
        Ok(BoardingPass {
            id: u32::from_str_radix(&binary, 2)?,
        })
    }
}

struct BoardingPasses {
    passes: Vec<BoardingPass>,
}

impl FromStr for BoardingPasses {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::trim);
        let mut passes = lines.map(str::parse).collect::<Result<Vec<_>>>()?;
        passes.sort_unstable();
        Ok(BoardingPasses { passes })
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = BoardingPasses;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        input
            .passes
            .last()
            .map(|pass| pass.id)
            .ok_or_else(|| anyhow!("no maximum seat ID found"))
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = BoardingPasses;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        input
            .passes
            .windows(2)
            .find_map(|seats| match seats {
                // Empty seat will be the one after `left`.
                &[left, right] if left.id + 1 != right.id => Some(left.id + 1),
                _ => None,
            })
            .ok_or_else(|| anyhow!("no empty seat found"))
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!("FBFBBFFRLR".parse::<BoardingPass>().unwrap().id, 357);
        assert_eq!("BFFFBBFRRR".parse::<BoardingPass>().unwrap().id, 567);
        assert_eq!("FFFBBBFRRR".parse::<BoardingPass>().unwrap().id, 119);
        assert_eq!("BBFFBBFRLL".parse::<BoardingPass>().unwrap().id, 820);
    }
}

aoc::solved! {
    PartOne = 822,
    PartTwo = 705,
}
