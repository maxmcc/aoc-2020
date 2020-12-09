use anyhow::{anyhow, bail};
use aoc::{self, Parse, Result, Solve};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Seat {
    id: u32,
}

impl Parse for Seat {
    fn parse(input: &str) -> Result<Self> {
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

#[derive(Clone, Debug)]
struct BoardingPasses {
    seats: Vec<Seat>,
}

impl Parse for BoardingPasses {
    fn parse(input: &str) -> Result<Self> {
        let lines = input.lines().map(str::trim);
        let mut seats = lines.map(Seat::parse).collect::<Result<Vec<_>>>()?;
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
                [left, right] if left.id + 1 != right.id => Some(left.id + 1),
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
        assert_eq!(Seat::parse("FBFBBFFRLR").unwrap().id, 357);
        assert_eq!(Seat::parse("BFFFBBFRRR").unwrap().id, 567);
        assert_eq!(Seat::parse("FFFBBBFRRR").unwrap().id, 119);
        assert_eq!(Seat::parse("BBFFBBFRLL").unwrap().id, 820);
    }
}

aoc::solved! {
    PartOne = 822,
    PartTwo = 705,
}
