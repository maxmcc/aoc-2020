use anyhow::{anyhow, bail};
use aoc::{Parse, Result, Solve};

#[derive(Clone, Debug)]
struct Cipher {
    nums: Vec<u64>,
}

impl<'a> Parse<'a> for Cipher {
    fn parse<'b: 'a>(input: &'b str) -> Result<Self> {
        let lines = input.lines().map(str::trim);
        let nums = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Cipher { nums })
    }
}

impl Cipher {
    fn first_invalid(&self, len: usize) -> Option<u64> {
        self.nums.windows(len + 1).find_map(|window| {
            let target = window[len];
            for (i, &x) in window[..len].iter().enumerate() {
                let y = target.wrapping_sub(x);
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

    fn summing_slice(&self, target: u64) -> &[u64] {
        let (mut i, mut j, mut sum) = (0, 0, 0);
        while sum != target {
            if sum < target {
                sum += self.nums[j];
                j += 1;
            } else {
                sum -= self.nums[i];
                i += 1;
            }
        }
        &self.nums[i..j]
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = Cipher;
    type Solution = u64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        input
            .first_invalid(25)
            .ok_or_else(|| anyhow!("no invalid number found"))
    }
}

struct PartTwo;

impl Solve<'_> for PartTwo {
    type Input = Cipher;
    type Solution = u64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let target = 22406676; // Part 1 answer
        let slice = input.summing_slice(target);
        match (slice.iter().min(), slice.iter().max()) {
            (Some(min), Some(max)) => Ok(min + max),
            _ => bail!("no summing slice found"),
        }
    }
}

aoc::main!(day09);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
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

        let slice = input.summing_slice(127);
        assert_eq!(
            slice.iter().min().unwrap() + slice.iter().max().unwrap(),
            62
        );
    }
}

aoc::solved!(day09, PartOne = 22406676, PartTwo = 2942387);
