use anyhow::bail;
use aoc::{Parse, Result, Solve};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SeatLayout {
    seats: Vec<Vec<Position>>,
}

impl<'a> Parse<'a> for SeatLayout {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        fn parse_line(line: &str) -> Result<Vec<Position>> {
            line.chars()
                .map(|ch| match ch {
                    '.' => Ok(Position::Floor),
                    'L' => Ok(Position::EmptySeat),
                    '#' => Ok(Position::OccupiedSeat),
                    _ => bail!("unexpected char {:?}", ch),
                })
                .collect()
        }
        let lines = input_str.lines().map(str::trim);
        let seats = lines.map(parse_line).collect::<Result<_>>()?;
        Ok(SeatLayout { seats })
    }
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl SeatLayout {
    fn checked_get(&self, row: isize, col: isize) -> Option<Position> {
        use std::convert::TryFrom;
        let row = usize::try_from(row).ok()?;
        let col = usize::try_from(col).ok()?;
        if row < self.seats.len() && col < self.seats[0].len() {
            Some(self.seats[row][col])
        } else {
            None
        }
    }

    fn occupied_moore(&self, row: usize, col: usize) -> usize {
        DIRECTIONS
            .iter()
            .map(|(dr, dc)| {
                let (row, col) = (row as isize + dr, col as isize + dc);
                self.checked_get(row, col)
            })
            .filter(|&pos| pos == Some(Position::OccupiedSeat))
            .count()
    }

    fn occupied_visible(&self, row: usize, col: usize) -> usize {
        DIRECTIONS
            .iter()
            .map(|(dr, dc)| {
                for dist in 1.. {
                    let row = row as isize + dist * dr;
                    let col = col as isize + dist * dc;
                    match self.checked_get(row, col) {
                        None => return None,
                        Some(Position::Floor) => continue,
                        Some(seat) => return Some(seat),
                    }
                }
                None
            })
            .filter(|&pos| pos == Some(Position::OccupiedSeat))
            .count()
    }
}

#[derive(Debug)]
struct Simulator<O, R> {
    read: SeatLayout,
    write: SeatLayout,
    occupied: O,
    rule: R,
}

impl<O, R> Simulator<O, R>
where
    O: Fn(&SeatLayout, usize, usize) -> usize,
    R: Fn(Position, usize) -> Position,
{
    fn step(&mut self) {
        for row in 0..self.read.seats.len() {
            for col in 0..self.read.seats[row].len() {
                let pos = self.read.seats[row][col];
                let occupied = (self.occupied)(&self.read, row, col);
                self.write.seats[row][col] = (self.rule)(pos, occupied);
            }
        }
        std::mem::swap(&mut self.read, &mut self.write);
    }

    fn run(&mut self) {
        loop {
            self.step();
            if self.read == self.write {
                break;
            }
        }
    }

    fn occupied_count(&self) -> usize {
        self.read
            .seats
            .iter()
            .flatten()
            .filter(|&&seat| seat == Position::OccupiedSeat)
            .count()
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = SeatLayout;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        fn rule(position: Position, occupied: usize) -> Position {
            match position {
                Position::OccupiedSeat if occupied >= 4 => Position::EmptySeat,
                Position::EmptySeat if occupied == 0 => Position::OccupiedSeat,
                other => other,
            }
        }

        let mut sim = Simulator {
            read: input.clone(),
            write: input.clone(),
            occupied: SeatLayout::occupied_moore,
            rule,
        };

        sim.run();

        Ok(sim.occupied_count())
    }
}

struct PartTwo;

impl Solve<'_> for PartTwo {
    type Input = SeatLayout;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        fn rule(position: Position, occupied: usize) -> Position {
            match position {
                Position::OccupiedSeat if occupied >= 5 => Position::EmptySeat,
                Position::EmptySeat if occupied == 0 => Position::OccupiedSeat,
                other => other,
            }
        }

        let mut sim = Simulator {
            read: input.clone(),
            write: input.clone(),
            occupied: SeatLayout::occupied_visible,
            rule,
        };

        sim.run();

        Ok(sim.occupied_count())
    }
}

aoc::main!(day11);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = SeatLayout::parse(indoc! {"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 37);
        assert_eq!(PartTwo::solve(&input).unwrap(), 26);
    }
}

aoc::solved!(day11, PartOne = 2324, PartTwo = 2068);
