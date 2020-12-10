use anyhow::bail;
use aoc::{Parse, Result, Solve};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
struct BagContents<'a> {
    count: u32,
    color: &'a str,
}

impl<'a> BagContents<'a> {
    fn new(count: u32, color: &'a str) -> Self {
        BagContents { count, color }
    }
}

#[derive(Debug)]
struct BagRule<'a> {
    color: &'a str,
    contents: Vec<BagContents<'a>>,
}

impl<'a> BagRule<'a> {
    fn new(color: &'a str) -> Self {
        BagRule {
            color,
            contents: Vec::new(),
        }
    }
}

lazy_static! {
    static ref CONTENTS_RE: Regex = Regex::new(r"(?P<count>\d+) (?P<color>\w+ \w+) bags?").unwrap();
}

impl<'a> Parse<'a> for BagRule<'a> {
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
        Ok(BagRule { color, contents })
    }
}

#[derive(Debug)]
struct BagRules<'a>(HashMap<&'a str, BagRule<'a>>);

impl<'a> Parse<'a> for BagRules<'a> {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let lines = input_str.lines().map(str::trim);
        let rules = lines
            .map(|line| {
                let bag = BagRule::parse(line)?;
                Ok((bag.color, bag))
            })
            .collect::<Result<_>>()?;
        Ok(BagRules(rules))
    }
}

impl BagRules<'_> {
    fn reversed(&self) -> BagRules {
        let inverted = self.0.iter().flat_map(|(outer, rule)| {
            rule.contents.iter().map(move |contents| {
                let invert = BagContents::new(contents.count, outer);
                (contents.color, invert)
            })
        });
        let mut rules = HashMap::new();
        for (inner, contents) in inverted {
            rules
                .entry(inner)
                .or_insert_with(|| BagRule::new(inner))
                .contents
                .push(contents)
        }
        BagRules(rules)
    }

    fn contents(&self, bag: &str) -> Option<&[BagContents]> {
        self.0.get(bag).map(|rule| &rule.contents[..])
    }
}

struct PartOne;

impl<'a> Solve<'a> for PartOne {
    type Input = BagRules<'a>;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        fn run<'a>(rules: &'a BagRules, bag: &'a str, seen: &mut HashSet<&'a str>) -> u32 {
            if seen.insert(bag) == false {
                return 0; // Already counted.
            }
            match rules.contents(bag) {
                None => 1,
                Some(contents) => {
                    1 + contents
                        .iter()
                        .map(|contents| run(rules, contents.color, seen))
                        .sum::<u32>()
                }
            }
        }
        Ok(run(&input.reversed(), "shiny gold", &mut HashSet::new()) - 1)
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = BagRules<'a>;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        fn run<'a>(rules: &'a BagRules, bag: &'a str) -> u32 {
            match rules.contents(bag) {
                None => 1,
                Some(contents) => {
                    1 + contents
                        .iter()
                        .map(|contents| contents.count * run(rules, contents.color))
                        .sum::<u32>()
                }
            }
        }
        Ok(run(&input, "shiny gold") - 1)
    }
}

aoc::main!(day07);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_part_one() {
        let input = BagRules::parse(indoc! {"
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

        assert_eq!(PartOne::solve(&input).unwrap(), 4);
        assert_eq!(PartTwo::solve(&input).unwrap(), 32);
    }

    #[test]
    fn example_part_two() {
        let input = BagRules::parse(indoc! {"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        "})
        .unwrap();

        assert_eq!(PartTwo::solve(&input).unwrap(), 126);
    }
}

aoc::solved!(day07, PartOne = 348, PartTwo = 18885);
