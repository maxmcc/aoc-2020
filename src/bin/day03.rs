use anyhow::bail;
use aoc::{Parse, Result, Solve};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

#[derive(Debug)]
struct Terrain {
    grid: Vec<Vec<Square>>,
}

impl<'a> Parse<'a> for Terrain {
    fn parse<'b: 'a>(input: &'a str) -> Result<Self> {
        fn parse_line(line: &str) -> Result<Vec<Square>> {
            line.trim()
                .chars()
                .map(|ch| match ch {
                    '.' => Ok(Square::Open),
                    '#' => Ok(Square::Tree),
                    other => bail!("unexpected char {:?}", other),
                })
                .collect()
        }
        let grid = input.lines().map(parse_line).collect::<Result<_>>()?;
        Ok(Terrain { grid })
    }
}

impl Terrain {
    fn trajectory(&self, dx: usize, dy: usize) -> Trajectory {
        Trajectory {
            terrain: self,
            x: 0,
            y: 0,
            dx,
            dy,
        }
    }
}

#[derive(Debug)]
struct Trajectory<'a> {
    terrain: &'a Terrain,
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
        if self.y >= self.terrain.grid.len() {
            None
        } else {
            let row = &self.terrain.grid[self.y];
            let square = row[self.x % row.len()];
            self.x += self.dx;
            self.y += self.dy;
            Some(square)
        }
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = Terrain;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input.trajectory(3, 1).tree_count())
    }
}

struct PartTwo;

impl Solve<'_> for PartTwo {
    type Input = Terrain;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        Ok(slopes
            .iter()
            .map(|&(dx, dy)| input.trajectory(dx, dy).tree_count())
            .product())
    }
}

aoc::main!(day03);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = Terrain::parse(indoc! {"
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
        "})
        .unwrap();
        assert_eq!(PartOne::solve(&input).unwrap(), 7);
        assert_eq!(PartTwo::solve(&input).unwrap(), 336);
    }
}

aoc::solved!(day03, PartOne = 274, PartTwo = 6050183040);
