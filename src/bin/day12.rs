use aoc::{Parse, Result, Solve};
use num::complex::Complex;
use parse_display::FromStr;

#[derive(Debug, Copy, Clone, FromStr)]
enum Step {
    #[display("N{0}")]
    North(i32),
    #[display("S{0}")]
    South(i32),
    #[display("E{0}")]
    East(i32),
    #[display("W{0}")]
    West(i32),
    #[display("L{0}")]
    Left(i32),
    #[display("R{0}")]
    Right(i32),
    #[display("F{0}")]
    Forward(i32),
}

struct Instructions {
    steps: Vec<Step>,
}

impl<'a> Parse<'a> for Instructions {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let lines = input_str.lines().map(str::trim);
        let steps = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Instructions { steps })
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = Instructions;
    type Solution = i32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut pos = Complex::new(0, 0);
        let mut head = Complex::new(1, 0);
        for &step in input.steps.iter() {
            match step {
                Step::North(dist) => pos += Complex::new(0, dist),
                Step::South(dist) => pos += Complex::new(0, -dist),
                Step::East(dist) => pos += Complex::new(dist, 0),
                Step::West(dist) => pos += Complex::new(-dist, 0),
                Step::Left(deg) => head *= Complex::i().powi(deg / 90),
                Step::Right(deg) => head *= Complex::i().powi(-deg / 90),
                Step::Forward(dist) => pos += head * dist,
            }
        }
        Ok(pos.re.abs() + pos.im.abs())
    }
}

struct PartTwo;

impl Solve<'_> for PartTwo {
    type Input = Instructions;
    type Solution = i32;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut ship = Complex::new(0, 0);
        let mut way = Complex::new(10, 1);
        for &step in input.steps.iter() {
            match step {
                Step::North(dist) => way += Complex::new(0, dist),
                Step::South(dist) => way += Complex::new(0, -dist),
                Step::East(dist) => way += Complex::new(dist, 0),
                Step::West(dist) => way += Complex::new(-dist, 0),
                Step::Left(deg) => way *= Complex::i().powi(deg / 90),
                Step::Right(deg) => way *= Complex::i().powi(-deg / 90),
                Step::Forward(dist) => ship += way * dist,
            }
        }
        Ok(ship.re.abs() + ship.im.abs())
    }
}

aoc::main!(day12);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = Instructions::parse(indoc! {"
            F10
            N3
            F7
            R90
            F11
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 25);
        assert_eq!(PartTwo::solve(&input).unwrap(), 286);
    }
}

aoc::solved!(day12, PartOne = 1838, PartTwo = 89936);
