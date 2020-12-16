use aoc::{Parse, Result, Solve};

#[derive(Debug)]
struct MemoryGame {
    start: Vec<u32>,
}

impl<'a> Parse<'a> for MemoryGame {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let start = input_str
            .split(",")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(MemoryGame { start })
    }
}

impl MemoryGame {
    fn nth(&self, n: usize) -> u32 {
        let mut said = vec![u32::MAX; n];
        for (turn, &num) in self.start.iter().enumerate() {
            said[num as usize] = turn as u32;
        }
        let mut last = self.start[self.start.len() - 1];
        for turn in self.start.len()..n {
            let turn = turn as u32;
            let mut prev = turn - 1;
            std::mem::swap(&mut prev, &mut said[last as usize]);
            last = (turn - 1).saturating_sub(prev);
        }
        last
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = MemoryGame;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input.nth(2020))
    }
}

struct PartTwo;

impl Solve<'_> for PartTwo {
    type Input = MemoryGame;
    type Solution = u32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input.nth(30_000_000))
    }
}

aoc::main!(day15);

#[cfg(test)]
mod examples {
    use super::*;

    #[test]
    fn example() {
        let input = MemoryGame::parse("0,3,6").unwrap();
        assert_eq!(PartOne::solve(&input).unwrap(), 436);
        assert_eq!(PartTwo::solve(&input).unwrap(), 175594);
    }

    #[test]
    fn more_examples() {
        let game = MemoryGame {
            start: vec![1, 3, 2],
        };
        assert_eq!(game.nth(2020), 1);
        assert_eq!(game.nth(30_000_000), 2578);

        let game = MemoryGame {
            start: vec![2, 1, 3],
        };
        assert_eq!(game.nth(2020), 10);
        assert_eq!(game.nth(30_000_000), 3544142);

        let game = MemoryGame {
            start: vec![1, 2, 3],
        };
        assert_eq!(game.nth(2020), 27);
        assert_eq!(game.nth(30_000_000), 261214);

        let game = MemoryGame {
            start: vec![2, 3, 1],
        };
        assert_eq!(game.nth(2020), 78);
        assert_eq!(game.nth(30_000_000), 6895259);

        let game = MemoryGame {
            start: vec![3, 2, 1],
        };
        assert_eq!(game.nth(2020), 438);
        assert_eq!(game.nth(30_000_000), 18);

        let game = MemoryGame {
            start: vec![3, 1, 2],
        };
        assert_eq!(game.nth(2020), 1836);
        assert_eq!(game.nth(30_000_000), 362);
    }
}

aoc::solved!(day15, PartOne = 662, PartTwo = 37312);
