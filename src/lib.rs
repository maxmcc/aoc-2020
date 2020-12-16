#![warn(clippy::all)]

use std::{fmt::Display, time::Instant};

pub use anyhow::{Context, Error, Result};

pub trait Parse<'a>: Sized {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self>;
}

pub trait Solve<'a> {
    type Input: Parse<'a>;
    type Solution: Display;
    fn solve(input: &Self::Input) -> Result<Self::Solution>;
}

#[macro_export]
macro_rules! input_str {
    ($day:ident) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/input/",
            stringify!($day),
            ".txt"
        ))
    };
}

#[macro_export]
macro_rules! main {
    ($day:ident) => {
        fn main() -> $crate::Result<()> {
            let input_str = $crate::input_str!($day);
            $crate::main_impl::<_, PartOne, PartTwo>(stringify!($day), input_str)
        }
    };
}

pub fn main_impl<'a, I, S1, S2>(day: &str, input_str: &'a str) -> Result<()>
where
    I: Parse<'a>,
    S1: Solve<'a, Input = I>,
    S2: Solve<'a, Input = I>,
{
    let parse_start = Instant::now();
    let input = I::parse(input_str).context("failed to parse input string")?;
    println!(
        "[{}] Parsed input\t\t(completed in {:.0?})",
        day,
        parse_start.elapsed()
    );

    let part_one_start = Instant::now();
    let part_one = S1::solve(&input).context("failed to solve part 1")?;
    println!(
        "[{}] Solved part 1: \t{}\t(completed in {:.0?})",
        day,
        part_one,
        part_one_start.elapsed()
    );

    let part_two_start = Instant::now();
    let part_two = S2::solve(&input).context("failed to solve part 2")?;
    println!(
        "[{}] Solved part 2: \t{}\t(completed in {:.0?})",
        day,
        part_two,
        part_two_start.elapsed()
    );

    Ok(())
}

#[macro_export]
macro_rules! solved {
    ($day:ident, $part1:ty = $soln1:expr, $part2:ty = $soln2:expr $(,)?) => {
        #[cfg(test)]
        mod solutions {
            use super::*;
            use $crate::{input_str, Parse, Solve};

            #[test]
            fn part_one_solution() {
                let input_str = input_str!($day);
                let input = <$part1 as Solve>::Input::parse(input_str)
                    .expect("failed to parse input string");
                assert_eq!(
                    <$part1 as Solve>::solve(&input).expect("failed to solve part 1"),
                    $soln1
                );
            }

            #[test]
            fn part_two_solution() {
                let input_str = input_str!($day);
                let input = <$part2 as Solve>::Input::parse(input_str)
                    .expect("failed to parse input string");
                assert_eq!(
                    <$part2 as Solve>::solve(&input).expect("failed to solve part 2"),
                    $soln2
                );
            }
        }
    };
}
