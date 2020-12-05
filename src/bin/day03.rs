use anyhow::anyhow;
use aoc::{self, Error, Result, Solve};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Square>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn parse_line(line: &str) -> Result<Vec<Square>> {
            line.trim()
                .chars()
                .map(|ch| match ch {
                    '.' => Ok(Square::Open),
                    '#' => Ok(Square::Tree),
                    other => Err(anyhow!("unknown char {:?}", other)),
                })
                .collect()
        }
        let grid = input.lines().map(parse_line).collect::<Result<_>>()?;
        Ok(Map { grid })
    }
}

impl Map {
    fn trajectory(&self, dx: usize, dy: usize) -> Trajectory {
        Trajectory {
            map: self,
            x: 0,
            y: 0,
            dx,
            dy,
        }
    }
}

#[derive(Debug)]
struct Trajectory<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
    dx: usize,
    dy: usize,
}

impl<'a> Trajectory<'a> {
    fn tree_count(self) -> usize {
        self.filter(|&it| it == Square::Tree).count()
    }
}

impl Iterator for Trajectory<'_> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.map.grid.len() {
            None
        } else {
            let row = &self.map.grid[self.y];
            let square = row[self.x % row.len()];
            self.x += self.dx;
            self.y += self.dy;
            Some(square)
        }
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = Map;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input.trajectory(3, 1).tree_count())
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = Map;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        Ok(slopes
            .iter()
            .map(|&(dx, dy)| input.trajectory(dx, dy).tree_count())
            .product())
    }
}

aoc::main!();

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "}
        .parse()
        .unwrap();
        assert_eq!(PartOne::solve(&input).unwrap(), 7);
        assert_eq!(PartTwo::solve(&input).unwrap(), 336);
    }
}

aoc::solved! {
    PartOne = 274,
    PartTwo = 6050183040,
}
