use anyhow::bail;
use aoc::{Parse, Result, Solve};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

#[derive(Debug)]
struct BagContents<'a> {
    count: u32,
    color: &'a str,
}

#[derive(Debug)]
struct Bag<'a> {
    color: &'a str,
    contents: Vec<BagContents<'a>>,
}

lazy_static! {
    static ref CONTENTS_RE: Regex = Regex::new(r"(?P<count>\d+) (?P<color>\w+ \w+) bags?").unwrap();
}

impl<'a> Parse<'a> for Bag<'a> {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let mut split = input_str.splitn(2, " bags contain ");
        let (color, contents) = match (split.next(), split.next()) {
            (Some(color), Some(contents)) => (color, contents),
            _ => bail!("could not split line: {:?}", input_str),
        };
        let contents = CONTENTS_RE
            .captures_iter(contents)
            .map(|captures: Captures| {
                let (count, color) = match (captures.name("count"), captures.name("color")) {
                    (Some(count), Some(color)) => (count.as_str().parse()?, color.as_str()),
                    _ => bail!("failed to match 'count' and 'color'"),
                };
                Ok(BagContents { count, color })
            })
            .collect::<Result<_>>()?;
        Ok(Bag { color, contents })
    }
}

#[derive(Debug)]
struct Bags<'a> {
    bags: Vec<Bag<'a>>,
}

impl<'a> Parse<'a> for Bags<'a> {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let lines = input_str.lines().map(str::trim);
        let bags = lines.map(Bag::parse).collect::<Result<_>>()?;
        Ok(Bags { bags })
    }
}

struct PartOne;

impl<'a> Solve<'a> for PartOne {
    type Input = Bags<'a>;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(0)
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = Bags<'a>;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(0)
    }
}

aoc::main!(day07);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_part_one() {
        let input = Bags::parse(indoc! {"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "})
        .unwrap();

        dbg!(&input);

        assert_eq!(PartOne::solve(&input).unwrap(), 0);
        assert_eq!(PartTwo::solve(&input).unwrap(), 0);
    }
}
