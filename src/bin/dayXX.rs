#![allow(non_snake_case)]
#![allow(unused_variables)]

use aoc::{self, Parse, Result, Solve};

struct Input;

impl Parse for Input {
    fn parse(input: &str) -> Result<Self> {
        unimplemented!()
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = Input;
    type Solution = i32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        unimplemented!()
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = Input;
    type Solution = i32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        unimplemented!()
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = Input::parse(indoc! {"
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 0);
        assert_eq!(PartTwo::solve(&input).unwrap(), 0);
    }
}
