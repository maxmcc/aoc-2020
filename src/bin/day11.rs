use anyhow::bail;
use aoc::{Parse, Result, Solve};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

impl Position {
    fn is_empty(&self) -> bool {
        matches!(self, Position::Empty)
    }

    fn is_occupied(&self) -> bool {
        matches!(self, Position::Occupied)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct SeatLayout {
    seats: Vec<Vec<Position>>,
}

impl<'a> Parse<'a> for SeatLayout {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let lines = input_str.lines().map(str::trim);
        let seats = lines
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '.' => Ok(Position::Floor),
                        'L' => Ok(Position::Empty),
                        '#' => Ok(Position::Occupied),
                        _ => bail!("unexpected char {:?}", ch),
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<_>>()?;
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
    fn get(&self, row: isize, col: isize) -> Option<Position> {
        if (0..self.seats.len() as isize).contains(&row)
            && (0..self.seats[0].len() as isize).contains(&col)
        {
            Some(self.seats[row as usize][col as usize])
        } else {
            None
        }
    }

    fn neighbors<'a>(&'a self, row: isize, col: isize) -> impl Iterator<Item = Position> + 'a {
        DIRECTIONS
            .iter()
            .flat_map(move |(dy, dx)| self.get(row + dy, col + dx))
    }

    // TODO: Memoize the indices generated here.
    fn visible_seats(&'a self, row: usize, col: usize) -> impl Iterator<Item = Position> + 'a {
        let rows = self.seats.len();
        let cols = self.seats[0].len();

        let up = (0..row).rev().map(|r| (r, col));
        let down = (row + 1..rows).map(|r| (r, col));
        let left = (0..col).rev().map(|c| (row, c));
        let right = (col + 1..cols).map(|c| (row, c));

        let up_r = (0..row).rev().zip(col + 1..cols);
        let dn_r = (row + 1..rows).zip(col + 1..cols);

        let up_l = (0..row).rev().zip((0..col).rev());
        let dn_l = (row + 1..rows).zip((0..col).rev());

        fn find(seats: &SeatLayout, mut iter: impl Iterator<Item = (usize, usize)>) -> Position {
            iter.find_map(|(r, c)| match seats.seats[r][c] {
                Position::Empty => None,
                seat => Some(seat),
            })
            .unwrap_or(Position::Empty)
        }

        vec![
            find(self, up),
            find(self, down),
            find(self, left),
            find(self, right),
            find(self, up_r),
            find(self, dn_r),
            find(self, up_l),
            find(self, dn_l),
        ]
    }

    fn step_one(&self) -> Self {
        let mut update = self.clone();
        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {
                let seat = &mut update.seats[row][col];
                match self.seats[row][col] {
                    Position::Empty
                        if self
                            .neighbors(row, col)
                            .all(|&seat| seat != Position::Occupied) =>
                    {
                        *seat = Position::Occupied;
                    }
                    Position::Occupied
                        if self
                            .neighbors(row, col)
                            .filter(|&&seat| seat == Position::Occupied)
                            .count()
                            >= 4 =>
                    {
                        *seat = Position::Empty;
                    }
                    _ => {}
                }
            }
        }
        update
    }

    fn step_two(&self) -> Self {
        let mut update = self.clone();
        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {
                let seat = &mut update.seats[row][col];
                match self.seats[row][col] {
                    Position::Empty
                        if self
                            .first_visible_seat(row, col)
                            .iter()
                            .all(|&seat| seat != Position::Occupied) =>
                    {
                        *seat = Position::Occupied;
                    }
                    Position::Occupied
                        if self
                            .first_visible_seat(row, col)
                            .iter()
                            .filter(|&&seat| seat == Position::Occupied)
                            .count()
                            >= 5 =>
                    {
                        *seat = Position::Empty;
                    }
                    _ => {}
                }
            }
        }
        update
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = SeatLayout;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut seats = input.clone();
        loop {
            let next = seats.step_one();
            if next == seats {
                break;
            }
            seats = next;
        }

        Ok(seats
            .seats
            .iter()
            .flatten()
            .filter(|&&seat| seat == Position::Occupied)
            .count())
    }
}

struct PartTwo;

impl<'a> Solve<'a> for PartTwo {
    type Input = SeatLayout;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut seats = input.clone();
        loop {
            let next = seats.step_two();
            if next == seats {
                break;
            }
            seats = next;
        }

        Ok(seats
            .seats
            .iter()
            .flatten()
            .filter(|&&seat| seat == Position::Occupied)
            .count())
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

        assert_eq!(
            input.step_one(),
            SeatLayout::parse(indoc! {"
                #.##.##.##
                #######.##
                #.#.#..#..
                ####.##.##
                #.##.##.##
                #.#####.##
                ..#.#.....
                ##########
                #.######.#
                #.#####.##
            "})
            .unwrap()
        );

        assert_eq!(
            input.step_one().step_one(),
            SeatLayout::parse(indoc! {"
                #.LL.L#.##
                #LLLLLL.L#
                L.L.L..L..
                #LLL.LL.L#
                #.LL.LL.LL
                #.LLLL#.##
                ..L.L.....
                #LLLLLLLL#
                #.LLLLLL.L
                #.#LLLL.##
            "})
            .unwrap()
        );

        assert_eq!(PartOne::solve(&input).unwrap(), 37);
        assert_eq!(PartTwo::solve(&input).unwrap(), 26);
    }
}
