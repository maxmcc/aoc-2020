use anyhow::{anyhow, bail};
use aoc::{self, Error, Result, Solve};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Seat {
    id: u32,
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let id = input.chars().try_fold(0, |acc, ch| {
            let next = match ch {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => bail!("unexpected char {:?}", ch),
            };
            Ok((acc << 1) | next)
        })?;
        Ok(Seat { id })
    }
}

struct BoardingPasses {
    seats: Vec<Seat>,
}

impl FromStr for BoardingPasses {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::trim);
        let mut seats = lines.map(str::parse).collect::<Result<Vec<_>>>()?;
        seats.sort_unstable();
        Ok(BoardingPasses { seats })
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = BoardingPasses;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        input
            .seats
            .last()
            .map(|pass| pass.id)
            .ok_or_else(|| anyhow!("no maximum seat ID found"))
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = BoardingPasses;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        input
            .seats
            .windows(2)
            .find_map(|seats| match seats {
                // Empty seat will be the one after `left`.
                &[left, right] if left.id + 1 != right.id => Some(left.id + 1),
                _ => None,
            })
            .ok_or_else(|| anyhow!("no empty seat found"))
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!("FBFBBFFRLR".parse::<Seat>().unwrap().id, 357);
        assert_eq!("BFFFBBFRRR".parse::<Seat>().unwrap().id, 567);
        assert_eq!("FFFBBBFRRR".parse::<Seat>().unwrap().id, 119);
        assert_eq!("BBFFBBFRLL".parse::<Seat>().unwrap().id, 820);
    }
}

aoc::solved! {
    PartOne = 822,
    PartTwo = 705,
}
