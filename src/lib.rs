use std::{fmt::Display, path::Path, time::Instant};

pub use anyhow::Context;
pub use anyhow::Error;
pub use anyhow::Result;

pub trait Parse: Sized {
    fn parse(input: &str) -> Result<Self>;
}

pub trait Solve {
    type Input: Parse;
    type Solution: Display;
    fn solve(input: &Self::Input) -> Result<Self::Solution>;
}

#[macro_export]
macro_rules! main {
    () => {
        fn main() -> $crate::Result<()> {
            let path = $crate::input_path!();
            $crate::_main::<_, _, PartOne, PartTwo>(&path)
        }
    };
}

#[macro_export]
macro_rules! input_path {
    () => {{
        use std::path::{Path, PathBuf};
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("input/");
        path.push(Path::new(file!()).file_stem().expect("missing file stem"));
        path.set_extension("txt");
        path
    }};
}

#[macro_export]
macro_rules! input {
    () => {{
        let path = $crate::input_path!();
        $crate::_input(&path).expect("failed to get input")
    }};
}

#[macro_export]
macro_rules! solved {
    ($part1:ty = $soln1:expr) => { solved!($part1 = $soln1,) };
    ($part1:ty = $soln1:expr, $($part2:ty = $soln2:expr),* $(,)?) => {
        #[cfg(test)]
        mod solutions {
            use super::*;
            use $crate::{Solve, Parse};

            #[test]
            fn test_solutions() {
                let path = $crate::input_path!();
                let text = std::fs::read_to_string(path)
                    .expect("failed to read input file");
                let input = <<$part1 as Solve>::Input as Parse>::parse(&text)
                    .expect("failed to parse input text");
                assert_eq!(
                    <$part1 as Solve>::solve(&input).unwrap(),
                    $soln1
                );

                $(
                    assert_eq!(
                        <$part2 as Solve>::solve(&input).unwrap(),
                        $soln2
                    );
                )+
            }
        }
    }
}

pub fn _main<P, I, S1, S2>(path: P) -> Result<()>
where
    P: AsRef<Path>,
    I: Parse,
    S1: Solve<Input = I>,
    S2: Solve<Input = I>,
{
    let path = path.as_ref();
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read input file:\n{}", path.display()))?;

    let input_start = Instant::now();
    let input = I::parse(&text).context("failed to parse input text")?;
    println!(
        "Successfully parsed input  (completed in {:.0?})",
        input_start.elapsed()
    );

    let part_one_start = Instant::now();
    let part_one = S1::solve(&input).context("failed to solve part 1")?;
    println!(
        "Part One: {}  (completed in {:.0?})",
        part_one,
        part_one_start.elapsed()
    );

    let part_two_start = Instant::now();
    let part_two = S2::solve(&input).context("failed to solve part 2")?;
    println!(
        "Part Two: {}  (completed in {:.0?})",
        part_two,
        part_two_start.elapsed()
    );

    Ok(())
}
