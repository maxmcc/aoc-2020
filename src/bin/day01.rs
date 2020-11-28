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
    fn fuel_for_mass(mass: i32) -> i32 {
        mass / 3 - 2
    }
}

impl Solve for PartOne {
    type Input = Masses;
    type Solution = i32;

    fn solve(input: &Masses) -> Result<Self::Solution> {
        Ok(input.0.iter().copied().map(Self::fuel_for_mass).sum())
    }
}

struct PartTwo;

impl PartTwo {
    fn fuel_for_module(module_mass: i32) -> i32 {
        let mut remaining = module_mass;
        let mut total = 0;
        while remaining > 2 {
            let fuel = PartOne::fuel_for_mass(remaining).max(0);
            total += fuel;
            remaining = fuel;
        }
        total
    }
}

impl Solve for PartTwo {
    type Input = Masses;
    type Solution = i32;

    fn solve(input: &Masses) -> Result<Self::Solution> {
        Ok(input.0.iter().copied().map(Self::fuel_for_module).sum())
    }
}

aoc::main!();

aoc::solved! {
    PartOne = 3262991,
    PartTwo = 4891620,
}
