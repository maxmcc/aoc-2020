use anyhow::{bail, ensure};
use aoc::{self, Error, Result, Solve};
use std::{collections::HashMap, convert::TryFrom, str::FromStr};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, parse_display::FromStr)]
pub enum FieldName {
    #[display("byr")]
    BirthYear,
    #[display("iyr")]
    IssueYear,
    #[display("eyr")]
    ExpirationYear,
    #[display("hgt")]
    Height,
    #[display("hcl")]
    HairColor,
    #[display("ecl")]
    EyeColor,
    #[display("pid")]
    PassportId,
    #[display("cid")]
    CountryId,
}

impl FieldName {
    const REQUIRED_FIELDS: [FieldName; 7] = [
        FieldName::BirthYear,
        FieldName::IssueYear,
        FieldName::ExpirationYear,
        FieldName::Height,
        FieldName::HairColor,
        FieldName::EyeColor,
        FieldName::PassportId,
    ];
}

#[derive(Clone, Debug)]
struct PassportData {
    fields: HashMap<FieldName, String>,
}

impl FromStr for PassportData {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let fields = input
            .split_whitespace()
            .map(|field| {
                let mut split = field.split(':');
                match (split.next(), split.next()) {
                    (Some(name), Some(data)) => Ok((name.parse()?, data.into())),
                    _ => bail!("invalid format {}", field),
                }
            })
            .collect::<Result<_>>()?;
        Ok(PassportData { fields })
    }
}

#[derive(Clone, Debug)]
struct BatchFile {
    passports: Vec<PassportData>,
}

impl FromStr for BatchFile {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let passports = input.split("\n\n").map(str::parse).collect::<Result<_>>()?;
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
            .filter(|pass| {
                FieldName::REQUIRED_FIELDS
                    .iter()
                    .all(|field| pass.fields.contains_key(field))
            })
            .count())
    }
}

mod field {
    /// Birth year: four digits, at least 1920 and at most 2002.
    #[derive(Copy, Clone, Debug, parse_display::FromStr)]
    #[from_str(regex = r"^(?P<0>19[2-9]\d|200[0-2])$")]
    pub struct BirthYear(pub u32);

    /// Issue year: four digits, at least 2010 and at most 2020.
    #[derive(Copy, Clone, Debug, parse_display::FromStr)]
    #[from_str(regex = r"^(?P<0>20(1\d|20))$")]
    pub struct IssueYear(pub u32);

    /// Expiration year: four digits, at leat 2020 and at most 2030.
    #[derive(Copy, Clone, Debug, parse_display::FromStr)]
    #[from_str(regex = r"^(?P<0>20(2\d|30))$")]
    pub struct ExpirationYear(pub u32);

    /// Height: a number followed by either cm or in.
    #[derive(Copy, Clone, Debug, parse_display::FromStr)]
    pub enum Height {
        /// If cm: at least 150 and at most 193.
        #[from_str(regex = r"^(?P<0>1([5-8]\d|9[0-3]))cm$")]
        Centimeters(u32),
        /// If in: at least 59 and at most 76.
        #[from_str(regex = r"^(?P<0>(59|6\d|7[0-6]))in$")]
        Inches(u32),
    }

    /// Hair color: a '#' followed by exactly six characters 0-9 or a-f.
    #[derive(Clone, Debug, parse_display::FromStr)]
    #[from_str(regex = r"^(?P<0>#[0-9a-f]{6})$")]
    pub struct HairColor(pub String);

    /// Eye color: exactly one of: amb blu brn gry grn hzl oth.
    #[derive(Copy, Clone, Debug, parse_display::FromStr)]
    #[display(style = "lowercase")]
    pub enum EyeColor {
        Amb,
        Blu,
        Brn,
        Gry,
        Grn,
        Hzl,
        Oth,
    }

    /// A nine-digit number, including leading zeroes.
    #[derive(Clone, Debug, parse_display::FromStr)]
    #[from_str(regex = r"^(?P<0>\d{9})$")]
    pub struct PassportId(pub String);

    /// Ignored, missing or not.
    #[derive(Clone, Debug, parse_display::FromStr)]
    #[display("{0}")]
    pub struct CountryId(pub String);
}

#[derive(Clone, Debug)]
struct ValidPassport {
    birth_year: field::BirthYear,
    issue_year: field::IssueYear,
    expiration_year: field::ExpirationYear,
    height: field::Height,
    hair_color: field::HairColor,
    eye_color: field::EyeColor,
    passport_id: field::PassportId,
    country_id: Option<field::CountryId>,
}

impl TryFrom<&PassportData> for ValidPassport {
    type Error = Error;

    fn try_from(data: &PassportData) -> Result<Self, Self::Error> {
        ensure!(
            FieldName::REQUIRED_FIELDS
                .iter()
                .all(|field| data.fields.contains_key(field)),
            "missing required field"
        );
        Ok(ValidPassport {
            birth_year: data.fields[&FieldName::BirthYear].parse()?,
            issue_year: data.fields[&FieldName::IssueYear].parse()?,
            expiration_year: data.fields[&FieldName::ExpirationYear].parse()?,
            height: data.fields[&FieldName::Height].parse()?,
            hair_color: data.fields[&FieldName::HairColor].parse()?,
            eye_color: data.fields[&FieldName::EyeColor].parse()?,
            passport_id: data.fields[&FieldName::PassportId].parse()?,
            country_id: data
                .fields
                .get(&FieldName::CountryId)
                .map(|id| field::CountryId(id.clone())),
        })
    }
}

struct PartTwo;

impl Solve for PartTwo {
    type Input = BatchFile;
    type Solution = usize;

    fn solve(input: &Self::Input) -> Result<Self::Solution> {
        Ok(input
            .passports
            .iter()
            .filter(|&pass| ValidPassport::try_from(pass).is_ok())
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

        assert_eq!(PartOne::solve(&input).unwrap(), 2);
    }

    #[test]
    fn test_examples_part_two() {
        assert!("2002".parse::<field::BirthYear>().is_ok());
        assert!("2003".parse::<field::BirthYear>().is_err());

        assert!("60in".parse::<field::Height>().is_ok());
        assert!("190cm".parse::<field::Height>().is_ok());
        assert!("190in".parse::<field::Height>().is_err());
        assert!("190".parse::<field::Height>().is_err());

        assert!("#123abc".parse::<field::HairColor>().is_ok());
        assert!("#123abz".parse::<field::HairColor>().is_err());
        assert!("123abc".parse::<field::HairColor>().is_err());

        assert!("brn".parse::<field::EyeColor>().is_ok());
        assert!("wat".parse::<field::EyeColor>().is_err());

        assert!("000000001".parse::<field::PassportId>().is_ok());
        assert!("0123456789".parse::<field::PassportId>().is_err());
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

        for data in input.passports {
            assert!(ValidPassport::try_from(&data).is_err());
        }
    }

    #[test]
    fn test_valid_passport_part_two() {
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

        for data in input.passports {
            assert!(ValidPassport::try_from(&data).is_ok());
        }
    }
}

aoc::solved! {
    PartOne = 247,
    PartTwo = 145,
}
