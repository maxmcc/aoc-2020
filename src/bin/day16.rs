use aoc::{Parse, Result, Solve};
use range_collections::RangeSet;
use std::collections::HashMap;
use std::ops::Range;

struct Input<'a> {
    fields: HashMap<&'a str, (Range<u32>, Range<u32>)>,
    mine: Vec<u32>,
    nearby: Vec<Vec<u32>>,
}

impl<'a> Parse<'a> for Input<'a> {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let mut groups = input_str.split("\n\n");
        let fields = groups
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut split = line.splitn(2, ": ");
                let field = split.next().unwrap();
                let ranges = split.next().unwrap().splitn(2, " or ");
                let mut ranges = ranges.map(|range| {
                    let mut bounds = range.split("-");
                    let low = bounds.next().unwrap().parse::<u32>().unwrap();
                    let hi = bounds.next().unwrap().parse::<u32>().unwrap();
                    low..(hi + 1)
                });
                (field, (ranges.next().unwrap(), ranges.next().unwrap()))
            })
            .collect();

        let mine = groups
            .next()
            .unwrap()
            .lines()
            .nth(1)
            .unwrap()
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();

        let nearby = groups
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
            .collect();

        Ok(Input {
            fields,
            mine,
            nearby,
        })
    }
}

struct PartOne;

impl<'a> Solve<'a> for PartOne {
    type Input = Input<'a>;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut ranges = RangeSet::<u32>::empty();
        for (r1, r2) in input.fields.values() {
            ranges |= RangeSet::from(r1.start..r1.end);
            ranges |= RangeSet::from(r2.start..r2.end);
        }
        let mut sum = 0;
        for other in input.nearby.iter() {
            for value in other {
                if !ranges.contains(&value) {
                    sum += value;
                }
            }
        }
        Ok(sum)
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = Input<'a>;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut ranges = RangeSet::<u32>::empty();
        for (r1, r2) in input.fields.values() {
            ranges |= RangeSet::from(r1.start..r1.end);
            ranges |= RangeSet::from(r2.start..r2.end);
        }
        let mut valid = Vec::new();
        for other in input.nearby.iter() {
            if other.iter().all(|value| ranges.contains(&value)) {
                valid.push(other);
            }
        }

        use itertools::Itertools;

        let order = input
            .fields
            .iter()
            .permutations(input.fields.len())
            .find(|order| {
                input.nearby.iter().all(|nearby| {
                    nearby
                        .iter()
                        .zip(order)
                        .all(|(val, (_, (r1, r2)))| r1.contains(&val) || r2.contains(&val))
                })
            })
            .unwrap();

        let mut prod = 1;
        for ((key, _), val) in order.iter().zip(input.mine.iter()) {
            if key.starts_with("departure") {
                prod *= val;
            }
        }

        Ok(prod)
    }
}

aoc::main!(day16);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = Input::parse(indoc! {"
            class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 71);
        assert_eq!(PartTwo::solve(&input).unwrap(), 0);
    }
}
