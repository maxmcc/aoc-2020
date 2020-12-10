use aoc::{Parse, Result, Solve};
use reformation::Reformation;

#[derive(Debug, Reformation)]
#[reformation("{min}-{max} {req}: {pass}")]
struct Entry<'a> {
    min: usize,
    max: usize,
    req: char,
    pass: &'a str,
}

#[derive(Debug)]
struct Passwords<'a> {
    entries: Vec<Entry<'a>>,
}

impl<'a> Parse<'a> for Passwords<'a> {
    fn parse<'b: 'a>(input: &'b str) -> Result<Self> {
        let lines = input.lines().map(str::trim);
        let entries = lines.map(Entry::parse).collect::<Result<_, _>>()?;
        Ok(Passwords { entries })
    }
}

struct PartOne;

impl<'a> Solve<'a> for PartOne {
    type Input = Passwords<'a>;
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

impl<'a> Solve<'a> for PartTwo {
    type Input = Passwords<'a>;
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

aoc::main!(day02);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = Passwords::parse(indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "})
        .unwrap();
        assert_eq!(PartOne::solve(&input).unwrap(), 2);
        assert_eq!(PartTwo::solve(&input).unwrap(), 1);
    }
}

aoc::solved!(day02, PartOne = 586, PartTwo = 352);
