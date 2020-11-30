use anyhow::{self, Context};
use std::{error::Error, fmt::Display, path::Path, str::FromStr};

pub use anyhow::Result;

pub trait Solve {
    type Input: FromStr;
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
    ($($part:ty = $solution:expr),* $(,)?) => {
        #[cfg(test)]
        mod solutions {
            use super::*;
            #[test]
            fn test_solutions() {
                let input = $crate::input!();
                $(
                    assert_eq!(
                        <$part as $crate::Solve>::solve(&input).unwrap(),
                        $solution
                    );
                )*
            }
        }
    }
}

pub fn _main<P, I, S1, S2>(path: P) -> Result<()>
where
    P: AsRef<Path>,
    I: FromStr,
    <I as FromStr>::Err: Error + Send + Sync + 'static,
    S1: Solve<Input = I>,
    S2: Solve<Input = I>,
{
    let input = _input(path)?;

    let part_one = S1::solve(&input).context("failed to solve part 1")?;
    println!("Part One: {}", part_one);

    let part_two = S2::solve(&input).context("failed to solve part 2")?;
    println!("Part Two: {}", part_two);

    Ok(())
}

pub fn _input<P, I>(path: P) -> Result<I>
where
    P: AsRef<Path>,
    I: FromStr,
    <I as FromStr>::Err: Error + Send + Sync + 'static,
{
    let path = path.as_ref();
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read input file:\n{}", path.display()))?;
    text.parse::<I>()
        .map_err(anyhow::Error::new)
        .context("failed to parse input text")
}
