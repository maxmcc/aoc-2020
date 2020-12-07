use aoc::{self, Error, Result, Solve};
use std::str::FromStr;

struct Input;

impl FromStr for Input {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
        let input: Input = indoc! {"
        "}
        .parse()
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 0);
        assert_eq!(PartTwo::solve(&input).unwrap(), 0);
    }
}
