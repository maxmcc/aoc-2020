// Note: this is day 1 from 2019, not 2020

use aoc::{self, Result, Solve};
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Masses(Vec<i32>);

impl FromStr for Masses {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let masses = s.lines().map(str::parse).collect::<Result<_, _>>()?;
        Ok(Masses(masses))
    }
}

struct PartOne;

impl PartOne {
    fn module_fuel(module: &i32) -> i32 {
        module / 3 - 2
    }
}

impl Solve for PartOne {
    type Input = Masses;
    type Solution = i32;

    fn solve(input: &Masses) -> Result<Self::Solution> {
        Ok(input.0.iter().map(Self::module_fuel).sum())
    }
}

struct PartTwo;

impl PartTwo {
    fn total_module_fuel(module: &i32) -> i32 {
        let init = Some(PartOne::module_fuel(module));
        std::iter::successors(init, |m| Some(PartOne::module_fuel(m)))
            .take_while(|&m| m > 0)
            .sum()
    }
}

impl Solve for PartTwo {
    type Input = Masses;
    type Solution = i32;

    fn solve(input: &Masses) -> Result<Self::Solution> {
        Ok(input.0.iter().map(Self::total_module_fuel).sum())
    }
}

aoc::main!();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regressions() {
        let input = aoc::input!();
        assert_eq!(PartOne::solve(&input).unwrap(), 3262991);
        assert_eq!(PartTwo::solve(&input).unwrap(), 4891620);
    }
}
