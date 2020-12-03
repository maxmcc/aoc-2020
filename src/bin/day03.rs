use aoc::{self, Result, Solve};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

#[derive(Debug, Clone)]
struct Map {
    pos: (usize, usize),
    grid: Vec<Vec<Square>>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().map(str::trim);
        let grid = lines
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Square::Open,
                        '#' => Square::Tree,
                        other => panic!("unknown square type {}", other),
                    })
                    .collect()
            })
            .collect();
        Ok(Map { pos: (0, 0), grid })
    }
}

impl Map {
    fn can_advance(&self) -> bool {
        self.pos.1 < self.grid.len() - 1
    }

    fn advance(&mut self, right: usize, down: usize) {
        self.pos.0 += right;
        self.pos.1 += down;
    }

    fn square_at(&self, x: usize, y: usize) -> Square {
        *self.grid[y].iter().cycle().nth(x).unwrap()
    }
}
struct PartOne;

impl Solve for PartOne {
    type Input = Map;
    type Solution = i32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut input = input.clone();

        let mut count = 0;
        while input.can_advance() {
            input.advance(3, 1);
            match input.square_at(input.pos.0, input.pos.1) {
                Square::Tree => count += 1,
                Square::Open => {}
            }
        }
        Ok(count)
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = Map;
    type Solution = i64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut total = 1;
        for (right, down) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            let mut input = input.clone();
            let mut count = 0;
            while input.can_advance() {
                input.advance(*right, *down);
                match input.square_at(input.pos.0, input.pos.1) {
                    Square::Tree => count += 1,
                    Square::Open => {}
                }
            }
            total *= count;
        }
        Ok(total)
    }
}

aoc::main!();

aoc::solved! {
    PartOne = 274,
    PartTwo = 6050183040,
}
