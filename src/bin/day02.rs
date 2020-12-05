use aoc::{self, Error, Result, Solve};
use std::str::FromStr;

#[derive(parse_display::FromStr, Debug)]
#[display("{min}-{max} {req}: {pass}")]
struct Entry {
    min: usize,
    max: usize,
    req: char,
    pass: String,
}

#[derive(Debug)]
struct Passwords {
    entries: Vec<Entry>,
}

impl FromStr for Passwords {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::trim);
        let entries = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Passwords { entries })
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = Passwords;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .entries
            .iter()
            .filter(|entry| {
                let count = entry.pass.matches(entry.req).count();
                count >= entry.min && count <= entry.max
            })
            .count())
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = Passwords;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .entries
            .iter()
            .filter(|entry| {
                let x = entry.pass.chars().nth(entry.min - 1);
                let y = entry.pass.chars().nth(entry.max - 1);
                (x == Some(entry.req)) ^ (y == Some(entry.req))
            })
            .count())
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "}
        .parse()
        .unwrap();
        assert_eq!(PartOne::solve(&input).unwrap(), 2);
        assert_eq!(PartTwo::solve(&input).unwrap(), 1);
    }
}

aoc::solved! {
    PartOne = 586,
    PartTwo = 352,
}
