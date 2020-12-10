use anyhow::bail;
use aoc::{Parse, Result, Solve};
use std::collections::HashSet;

#[derive(Debug)]
struct ExpenseReport {
    entries: HashSet<i32>,
}

impl<'a> Parse<'a> for ExpenseReport {
    fn parse<'b: 'a>(input: &'b str) -> Result<Self> {
        let lines = input.lines().map(str::trim);
        let entries = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(ExpenseReport { entries })
    }
}

const TARGET: i32 = 2020;

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = ExpenseReport;
    type Solution = i32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        for x in input.entries.iter() {
            let y = TARGET - x;
            if input.entries.contains(&y) {
                return Ok(x * y);
            }
        }
        bail!("no two numbers found");
    }
}

struct PartTwo;

impl Solve<'_> for PartTwo {
    type Input = ExpenseReport;
    type Solution = i32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        for x in input.entries.iter() {
            for y in input.entries.iter() {
                let z = TARGET - x - y;
                if input.entries.contains(&z) {
                    return Ok(x * y * z);
                }
            }
        }
        bail!("no three numbers found");
    }
}

aoc::main!(day01);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = ExpenseReport::parse(indoc! {"
            1721
            979
            366
            299
            675
            1456
        "})
        .unwrap();
        assert_eq!(PartOne::solve(&input).unwrap(), 1721 * 299);
        assert_eq!(PartTwo::solve(&input).unwrap(), 979 * 366 * 675);
    }
}

aoc::solved!(day01, PartOne = 1018944, PartTwo = 8446464);
