use aoc::{self, Result, Solve};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<Option<String>>,
}

#[derive(Debug)]
struct BatchFile {
    passports: Vec<Passport>,
}

impl FromStr for BatchFile {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut passports = Vec::new();
        let mut lines = input.lines();
        let mut p = Passport::default();
        loop {
            match lines.next() {
                None => {
                    passports.push(p);
                    break;
                }
                Some(l) if l.trim().is_empty() => {
                    passports.push(p);
                    p = Passport::default();
                    continue;
                }
                Some(components) => {
                    let s = components
                        .split(|x: char| x.is_whitespace() || x == ':')
                        .collect::<Vec<_>>();

                    use std::convert::TryFrom;
                    for &[field, data] in s.chunks(2).flat_map(<&[_; 2]>::try_from) {
                        match field {
                            "byr" => p.byr = Some(data.into()),
                            "iyr" => p.iyr = Some(data.into()),
                            "eyr" => p.eyr = Some(data.into()),
                            "hgt" => p.hgt = Some(data.into()),
                            "hcl" => p.hcl = Some(data.into()),
                            "ecl" => p.ecl = Some(data.into()),
                            "pid" => p.pid = Some(data.into()),
                            "cid" => p.cid = Some(Some(data.into())),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        Ok(BatchFile { passports })
    }
}

struct PartOne;

impl Solve for PartOne {
    type Input = BatchFile;
    type Solution = usize;
    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .passports
            .iter()
            .filter(|p| {
                p.byr.is_some()
                    && p.iyr.is_some()
                    && p.eyr.is_some()
                    && p.hgt.is_some()
                    && p.hcl.is_some()
                    && p.ecl.is_some()
                    && p.pid.is_some()
            })
            .count())
    }
}

struct PartTwo;

impl PartTwo {
    fn is_year_range(s: &str, lo: usize, hi: usize) -> bool {
        if s.len() != 4 {
            return false;
        }
        match s.parse::<usize>() {
            Ok(year) => year >= lo && year <= hi,
            Err(_) => false,
        }
    }

    fn is_valid_height(s: &str) -> bool {
        if s.ends_with("cm") {
            match s.strip_suffix("cm").unwrap().parse::<usize>() {
                Ok(h) => h >= 150 && h <= 193,
                Err(_) => false,
            }
        } else if s.ends_with("in") {
            match s.strip_suffix("in").unwrap().parse::<usize>() {
                Ok(h) => h >= 59 && h <= 76,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

impl Solve for PartTwo {
    type Input = BatchFile;
    type Solution = usize;


    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .passports
            .iter()
            .filter(|p| {
                p.byr
                    .clone()
                    .map(|s| PartTwo::is_year_range(&s, 1920, 2002))
                    .unwrap_or_else(|| {
                        println!("failed byr");
                        false
                    })
                    && p.iyr
                        .clone()
                        .map(|s| PartTwo::is_year_range(&s, 2010, 2020))
                        .unwrap_or_else(|| {
                            println!("failed iyr");
                            false
                        })
                    && p.eyr
                        .clone()
                        .map(|s| PartTwo::is_year_range(&s, 2020, 2030))
                        .unwrap_or_else(|| {
                            println!("failed eyr");
                            false
                        })
                    && p.hgt
                        .clone()
                        .map(|s| PartTwo::is_valid_height(&s))
                        .unwrap_or_else(|| {
                            println!("failed hgt");
                            false
                        })
                    && p.hcl
                        .clone()
                        .map(|s| Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(&s))
                        .unwrap_or_else(|| {
                            println!("failed hcl");
                            false
                        })
                    && p.ecl
                        .clone()
                        .map(|s| {
                            matches!(&*s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                        })
                        .unwrap_or_else(|| {
                            println!("failed ecl");
                            false
                        })
                    && p.pid
                        .clone()
                        .map(|s| Regex::new("^\\d{9}$").unwrap().is_match(&s))
                        .unwrap_or_else(|| {
                            println!("failed pid");
                            false
                        })
            })
            .count())
    }
}

aoc::main!();

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example_part_one() {
        let input: BatchFile = indoc! {"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "}
        .parse()
        .unwrap();

        dbg!(&input);

        assert_eq!(PartOne::solve(&input).unwrap(), 2);
    }

    #[test]
    fn test_invalid_example_part_two() {
        let input: BatchFile = indoc! {"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        "}
        .parse()
        .unwrap();

        assert_eq!(PartTwo::solve(&input).unwrap(), 0);
    }

    #[test]
    fn test_valid_example_part_two() {
        let input: BatchFile = indoc! {"
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "}
        .parse()
        .unwrap();

        assert_eq!(PartTwo::solve(&input).unwrap(), 4);
    }
}
