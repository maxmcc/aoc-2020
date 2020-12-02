use aoc::{self, Result, Solve};
use scan_fmt::scan_fmt;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Eq, PartialEq)]
struct PasswordEntry {
    first_req: usize,
    second_req: usize,
    character: char,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (first_req, second_req, character, password) =
            scan_fmt!(input, "{}-{} {}: {}", usize, usize, char, String)?;
        Ok(PasswordEntry {
            first_req,
            second_req,
            character,
            password,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Passwords {
    entries: Vec<PasswordEntry>,
}

impl FromStr for Passwords {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::trim);
        let entries = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Passwords { entries })
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = Passwords;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .entries
            .iter()
            .filter(|entry| {
                let mut freqs = HashMap::new();
                for ch in entry.password.chars() {
                    let freq = freqs.entry(ch).or_insert(0);
                    *freq += 1;
                }
                let range = entry.first_req..=entry.second_req;
                freqs
                    .get(&entry.character)
                    .map(|freq| range.contains(&freq))
                    .unwrap_or(false)
            })
            .count())
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = Passwords;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .entries
            .iter()
            .filter(|entry| {
                let first = entry.password.chars().nth(entry.first_req - 1);
                let second = entry.password.chars().nth(entry.second_req - 1);
                (first == Some(entry.character)) ^ (second == Some(entry.character))
            })
            .count())
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
            1-3 a: abcde
            1-3 b: cdefg
            2-9 c: ccccccccc
        "}
        .parse()
        .unwrap();
        assert_eq!(PartOne::solve(&input).unwrap(), 2);
        assert_eq!(PartTwo::solve(&input).unwrap(), 1);
    }
}

aoc::solved! {
    PartOne = 586,
    PartTwo = 352,
}
