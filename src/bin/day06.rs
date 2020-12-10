use anyhow::ensure;
use aoc::{Parse, Result, Solve};
use std::ops::{BitAnd, BitOr};

#[derive(Clone, Debug)]
struct CustomsForms {
    groups: Vec<Vec<u32>>,
}

impl<'a> Parse<'a> for CustomsForms {
    fn parse<'b: 'a>(input: &'b str) -> Result<Self> {
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

impl Solve<'_> for PartOne {
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

impl Solve<'_> for PartTwo {
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

aoc::main!(day06);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = CustomsForms::parse(indoc! {"
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
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 11);
        assert_eq!(PartTwo::solve(&input).unwrap(), 6);
    }
}

aoc::solved!(day06, PartOne = 6630, PartTwo = 3437);
