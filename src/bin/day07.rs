use anyhow::bail;
use aoc::{Parse, Result, Solve};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct BagRule<'a> {
    color: &'a str,
    contents: HashMap<&'a str, u32>,
}

lazy_static! {
    static ref BAG_CONTENTS_REGEX: Regex =
        Regex::new(r"(?P<count>\d+) (?P<color>\w+ \w+) bags?").unwrap();
}

impl<'a> Parse<'a> for BagRule<'a> {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let mut split = input_str.splitn(2, " bags contain ");
        let (color, contents) = match (split.next(), split.next()) {
            (Some(color), Some(contents)) => (color, contents),
            _ => bail!("could not split line: {:?}", input_str),
        };
        let contents = BAG_CONTENTS_REGEX
            .captures_iter(contents)
            .map(|caps| match (caps.name("color"), caps.name("count")) {
                (Some(color), Some(count)) => Ok((color.as_str(), count.as_str().parse()?)),
                _ => bail!("failed to match count and color in captures: {:?}", caps),
            })
            .collect::<Result<_>>()?;
        Ok(BagRule { color, contents })
    }
}

#[derive(Debug)]
struct BagRules<'a> {
    rules: HashMap<&'a str, BagRule<'a>>,
}

impl<'a> Parse<'a> for BagRules<'a> {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let lines = input_str.lines().map(str::trim);
        let rules = lines
            .map(|line| {
                let rule = BagRule::parse(line)?;
                Ok((rule.color, rule))
            })
            .collect::<Result<_>>()?;
        Ok(BagRules { rules })
    }
}

struct PartOne;

impl<'a> Solve<'a> for PartOne {
    type Input = BagRules<'a>;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        fn can_hold_gold<'a>(
            rules: &'a HashMap<&str, BagRule>,
            bag: &'a str,
            cache: &mut HashMap<&'a str, bool>,
        ) -> bool {
            cache.get(bag).copied().unwrap_or_else(|| {
                let mut contents = rules[bag].contents.keys();
                let ans = contents.any(|bag| can_hold_gold(rules, bag, cache));
                cache.insert(bag, ans);
                ans
            })
        }

        let mut cache = HashMap::new();
        cache.insert("shiny gold", true);
        let count = input
            .rules
            .keys()
            .copied()
            .filter(|bag| can_hold_gold(&input.rules, bag, &mut cache))
            .count();
        Ok(count - 1)
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = BagRules<'a>;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        fn total_bags(rules: &HashMap<&str, BagRule>, bag: &str) -> u32 {
            1 + rules[bag]
                .contents
                .iter()
                .map(|(bag, count)| count * total_bags(rules, bag))
                .sum::<u32>()
        }
        let count = total_bags(&input.rules, "shiny gold");
        Ok(count - 1)
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
