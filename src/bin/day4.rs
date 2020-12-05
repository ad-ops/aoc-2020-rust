use aoc_2020_rust::{puzzle_main, puzzle_tests, Puzzle};
use parse_display::FromStr;

#[derive(FromStr, PartialEq, Debug)]
#[from_str(regex = r"byr:(?P<birth_year>\d+)( cid:(?P<country_id>.+))? ecl:(?P<eye_color>.+) eyr:(?P<expiration_year>\d+) hcl:(?P<hair_color>.+) hgt:(?P<height>.+) iyr:(?P<issue_year>\d+) pid:(?P<passport_id>.+)")]
struct PassportLax {
  passport_id: String,
  country_id: String,
  birth_year: u16,
  issue_year: u16,
  expiration_year: u16,
  height: String,
  hair_color: String,
  eye_color: String,
}
impl PassportLax {
    fn new(s: String) -> Result<Self, parse_display::ParseError> {
        let mut fields: Vec<&str> = s
            .split(" ")
            .collect();
        fields.sort();
        fields
            .join(" ")
            .parse::<PassportLax>()
    }
}

#[derive(FromStr, PartialEq, Debug)]
#[from_str(regex = r"byr:(?P<birth_year>\d{4})( cid:(?P<country_id>.+))? ecl:(?P<eye_color>(amb|blu|brn|gry|grn|hzl|oth)) eyr:(?P<expiration_year>\d{4}) hcl:(?P<hair_color>#.[0-9a-f]+) hgt:(?P<height>(\d{3}cm|\d{2}in)) iyr:(?P<issue_year>\d{4}) pid:(?P<passport_id>\d{9})")]
struct PassportStrict {
  passport_id: String,
  country_id: String,
  birth_year: u16,
  issue_year: u16,
  expiration_year: u16,
  height: String,
  hair_color: String,
  eye_color: String,
}
impl PassportStrict {
    fn new(s: String) -> Result<Self, parse_display::ParseError> {
        let mut fields: Vec<&str> = s
            .split(" ")
            .collect();
        fields.sort();
        fields
            .join(" ")
            .parse::<PassportStrict>()
    }
 
    fn valid(&self) -> bool {
        let birth_year_valid = self.birth_year >= 1920 && self.birth_year <= 2002;
        let issue_year_valid = self.issue_year >= 2010 && self.issue_year <= 2020;
        let expiration_year_valid = self.expiration_year >= 2020 && self.expiration_year <= 2030;
        let height_valid = {
            if self.height.ends_with("cm") {
                let height = self.height[..3].parse::<u8>().unwrap_or_default();
                height >= 150 && height <= 193
            }
            else if self.height.ends_with("in") {
                let height = self.height[..2].parse::<u8>().unwrap_or_default();
                height >= 59 && height <= 76
            }
            else {
                false
            }
        };

        birth_year_valid && issue_year_valid && expiration_year_valid && height_valid
    }
}

fn solver_part1(input: Vec<String>) -> String {
    let passports: Vec<PassportLax> = input
        .split(|l| l.is_empty())
        .map(|g| g.join(" "))
        .filter_map(|s| PassportLax::new(s).ok())
        .collect();

    let solution: usize = passports.len();
    solution.to_string()
}

fn solver_part2(input: Vec<String>) -> String {
    let passports: Vec<PassportStrict> = input
        .split(|l| l.is_empty())
        .map(|g| g.join(" "))
        .filter_map(|s| PassportStrict::new(s).ok())
        .filter(|p| p.valid())
        .collect();

    let solution: usize = passports.len();
    solution.to_string()
}

puzzle_main!(solver_part1, solver_part2);

puzzle_tests!("10", "6");