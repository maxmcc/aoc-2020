use anyhow::{anyhow, bail};
use aoc::{Parse, Result, Solve};

#[derive(Debug)]
struct Adapters {
    adapters: Vec<u64>,
}

impl<'a> Parse<'a> for Adapters {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let lines = input_str.lines().map(str::trim);
        let mut jolts = lines.map(str::parse).collect::<Result<Vec<_>, _>>()?;
        jolts.push(0);
        jolts.sort_unstable();
        match jolts.last().copied() {
            Some(max) => jolts.push(max + 3),
            None => bail!("input is empty"),
        }
        Ok(Adapters { adapters: jolts })
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = Adapters;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let (ones, threes) = input
            .adapters
            .windows(2)
            .fold((0, 0), |(ones, threes), slice| {
                let diff = slice[1] - slice[0];
                match diff {
                    1 => (ones + 1, threes),
                    3 => (ones, threes + 1),
                    _ => (ones, threes),
                }
            });
        Ok(ones * threes)
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = Adapters;
    type Solution = u64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut ways = vec![0; input.adapters.len()];
        ways[0] = 1;
        for i in 0..ways.len() {
            for j in i.saturating_sub(3)..i {
                if input.adapters[i] - input.adapters[j] <= 3 {
                    ways[i] += ways[j];
                }
            }
        }
        ways.last()
            .copied()
            .ok_or_else(|| anyhow!("input was empty"))
    }
}

aoc::main!(day10);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_small() {
        let input = Adapters::parse(indoc! {"
            16
            10
            15
            5
            1
            11
            7
            19
            6
            12
            4
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 35);
        assert_eq!(PartTwo::solve(&input).unwrap(), 8);
    }

    #[test]
    fn example_large() {
        let input = Adapters::parse(indoc! {"
            28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 220);
        assert_eq!(PartTwo::solve(&input).unwrap(), 19208);
    }
}

aoc::solved!(day10, PartOne = 1848, PartTwo = 8099130339328);
