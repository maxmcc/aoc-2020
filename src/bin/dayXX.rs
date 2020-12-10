#![allow(non_snake_case)]
#![allow(unused_variables)]

use anyhow::bail;
use aoc::{Parse, Result, Solve};

struct Input;

impl<'a> Parse<'a> for Input {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        bail!("unimplemented");
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = Input;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(0)
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = Input;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(0)
    }
}

aoc::main!(day01);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = Input::parse(indoc! {"
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 0);
        assert_eq!(PartTwo::solve(&input).unwrap(), 0);
    }
}
