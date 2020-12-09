use anyhow::anyhow;
use aoc::{self, Parse, Result, Solve};

#[derive(Clone, Debug)]
struct Cipher {
    nums: Vec<i64>,
}

impl Parse for Cipher {
    fn parse(input: &str) -> Result<Self> {
        let lines = input.lines().map(str::trim);
        let nums = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Cipher { nums })
    }
}

impl Cipher {
    fn first_invalid(&self, len: usize) -> Option<i64> {
        self.nums.windows(len + 1).find_map(|window| {
            let target = window[len];
            for (i, &x) in window[..len].iter().enumerate() {
                let y = target - x;
                if x == y {
                    continue;
                }
                if window[i + 1..].contains(&y) {
                    return None;
                }
            }
            Some(target)
        })
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = Cipher;
    type Solution = i64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        input
            .first_invalid(25)
            .ok_or_else(|| anyhow!("no invalid number found"))
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = Cipher;
    type Solution = i64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        const PART_ONE_ANSWER: i64 = 22406676;

        for len in 2.. {
            for slice in input.nums.windows(len) {
                if slice.iter().sum::<i64>() == PART_ONE_ANSWER {
                    let min = slice.iter().min().unwrap();
                    let max = slice.iter().max().unwrap();
                    return Ok(min + max);
                }
            }
        }
        anyhow::bail!("no such range found");
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = Cipher::parse(indoc! {"
            35
            20
            15
            25
            47
            40
            62
            55
            65
            95
            102
            117
            150
            182
            127
            219
            299
            277
            309
            576
        "})
        .unwrap();

        assert_eq!(input.first_invalid(5).unwrap(), 127);
        assert_eq!(PartTwo::solve(&input).unwrap(), 62);
    }
}

aoc::solved! {
    PartOne = 22406676,
    PartTwo = 2942387,
}
