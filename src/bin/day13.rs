use anyhow::anyhow;
use aoc::{Parse, Result, Solve};

#[derive(Debug, Copy, Clone)]
struct Bus {
    headway: u64,
    index: u64,
}

struct BusSchedule {
    depart: u64,
    buses: Vec<Bus>,
}

impl<'a> Parse<'a> for BusSchedule {
    fn parse<'b: 'a>(input_str: &'b str) -> Result<Self> {
        let mut lines = input_str.lines().map(str::trim);
        let depart = lines
            .next()
            .ok_or_else(|| anyhow!("no timestamp"))?
            .parse()?;
        let buses = lines
            .next()
            .ok_or_else(|| anyhow!("no buses"))?
            .split(',')
            .enumerate()
            .filter(|&(_, bus)| bus != "x")
            .map(|(index, bus)| {
                let headway = bus.parse()?;
                let index = index as u64;
                Ok(Bus { headway, index })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(BusSchedule { depart, buses })
    }
}

struct PartOne;

impl Solve<'_> for PartOne {
    type Input = BusSchedule;
    type Solution = u64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let (bus, wait) = input
            .buses
            .iter()
            .map(|bus| (bus, bus.headway - input.depart % bus.headway))
            .min_by_key(|&(_, wait)| wait)
            .ok_or_else(|| anyhow!("no bus found"))?;
        Ok(bus.headway * wait)
    }
}

struct PartTwo;

impl Solve<'_> for PartTwo {
    type Input = BusSchedule;
    type Solution = u64;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        let mut time = 1;
        let mut wait = 1;
        for bus in input.buses.iter() {
            while (time + bus.index) % bus.headway != 0 {
                time += wait;
            }
            wait *= bus.headway;
        }
        Ok(time)
    }
}

aoc::main!(day13);

#[cfg(test)]
mod examples {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example_part_one() {
        let input = BusSchedule::parse(indoc! {"
            939
            7,13,x,x,59,x,31,19
        "})
        .unwrap();

        assert_eq!(PartOne::solve(&input).unwrap(), 295);
    }

    #[test]
    fn example_part_two() {
        let input = BusSchedule::parse("0\n7,13,x,x,59,x,31,19").unwrap();
        assert_eq!(PartTwo::solve(&input).unwrap(), 1068781);

        let input = BusSchedule::parse("0\n17,x,13,19").unwrap();
        assert_eq!(PartTwo::solve(&input).unwrap(), 3417);

        let input = BusSchedule::parse("0\n67,7,59,61").unwrap();
        assert_eq!(PartTwo::solve(&input).unwrap(), 754018);

        let input = BusSchedule::parse("0\n67,x,7,59,61").unwrap();
        assert_eq!(PartTwo::solve(&input).unwrap(), 779210);

        let input = BusSchedule::parse("0\n67,7,x,59,61").unwrap();
        assert_eq!(PartTwo::solve(&input).unwrap(), 1261476);

        let input = BusSchedule::parse("0\n1789,37,47,1889").unwrap();
        assert_eq!(PartTwo::solve(&input).unwrap(), 1202161486);
    }
}

aoc::solved!(day13, PartOne = 5257, PartTwo = 538703333547789);
