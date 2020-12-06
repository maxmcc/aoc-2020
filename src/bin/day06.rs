use anyhow::ensure;
use aoc::{self, Error, Result, Solve};
use std::{
    ops::{BitAnd, BitOr},
    str::FromStr,
};

#[derive(Clone, Debug)]
struct CustomsForms {
    groups: Vec<Vec<u32>>,
}

impl FromStr for CustomsForms {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn parse_answers(line: &str) -> Result<u32> {
            line.trim().chars().try_fold(0, |acc, ch| {
                ensure!(('a'..='z').contains(&ch), "unexpected char {:?}", ch);
                Ok(acc | (1 << ch as u8 - b'a'))
            })
        }
        let groups = input
            .split("\n\n")
            .map(|group| group.lines().map(parse_answers).collect::<Result<Vec<_>>>())
            .collect::<Result<_>>()?;
        Ok(CustomsForms { groups })
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = CustomsForms;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .groups
            .iter()
            .map(|group| group.iter().fold(0, u32::bitor).count_ones())
            .sum())
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = CustomsForms;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .groups
            .iter()
            .map(|group| group.iter().fold(0x3FFFFFF, u32::bitand).count_ones())
            .sum())
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input: CustomsForms = indoc! {"
            abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b
        "}
        .parse()
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 11);
        assert_eq!(PartTwo::solve(&input).unwrap(), 6);
    }
}

aoc::solved! {
    PartOne = 6630,
    PartTwo = 3437,
}
