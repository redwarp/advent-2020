use lazy_static::lazy_static;
use std::fs;

use regex::Regex;

static REQUIRED: [&str; 7] = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

lazy_static! {
    static ref BYR: Regex = Regex::new("byr:([1-2][0-9]{3})( |\\r|$)").unwrap();
    static ref IYR: Regex = Regex::new("iyr:(20[0-9]{2})( |\\r|$)").unwrap();
    static ref EYR: Regex = Regex::new("eyr:(20[0-9]{2})( |\\r|$)").unwrap();
    static ref HGT_CM: Regex = Regex::new("hgt:(1[0-9]{2})cm( |\\r|$)").unwrap();
    static ref HGT_IN: Regex = Regex::new("hgt:([0-9]{2})in( |\\r|$)").unwrap();
    static ref HCL: Regex = Regex::new("hcl:#([0-9]|[a-f]){6}( |\\r|$)").unwrap();
    static ref ECL: Regex = Regex::new("ecl:(amb|blu|brn|gry|grn|hzl|oth)( |\\r|$)").unwrap();
    static ref PID: Regex = Regex::new("pid:([0-9]{9})( |\\r|$)").unwrap();
}

struct Passport<'a> {
    content: &'a str,
}

impl<'a> Passport<'a> {
    fn new(content: &'a str) -> Self {
        Passport { content }
    }

    fn is_valid_simple(&self) -> bool {
        for &required in REQUIRED.iter() {
            if !self.content.contains(required) {
                return false;
            }
        }

        true
    }

    fn is_valid(&self) -> bool {
        self.is_valid_byr()
            && self.is_valid_iyr()
            && self.is_valid_eyr()
            && self.is_valid_hgt()
            && self.is_valid_hcl()
            && self.is_valid_ecl()
            && self.is_valid_pid()
    }

    fn is_valid_byr(&self) -> bool {
        if let Some(captures) = BYR.captures(self.content) {
            if let Some(date) = captures.get(1) {
                let date = date.as_str().parse::<i32>().unwrap_or(0);
                return date >= 1920 && date <= 2002;
            }
        }
        false
    }
    fn is_valid_iyr(&self) -> bool {
        if let Some(captures) = IYR.captures(self.content) {
            if let Some(date) = captures.get(1) {
                let date = date.as_str().parse::<i32>().unwrap_or(0);
                return date >= 2010 && date <= 2020;
            }
        }
        false
    }
    fn is_valid_eyr(&self) -> bool {
        if let Some(captures) = EYR.captures(self.content) {
            if let Some(date) = captures.get(1) {
                let date = date.as_str().parse::<i32>().unwrap_or(0);
                return date >= 2020 && date <= 2030;
            }
        }
        false
    }

    fn is_valid_hgt(&self) -> bool {
        self.is_valid_hgt_cm() || self.is_valid_hgt_in()
    }

    fn is_valid_hgt_cm(&self) -> bool {
        if let Some(captures) = HGT_CM.captures(self.content) {
            if let Some(height) = captures.get(1) {
                let height = height.as_str().parse::<i32>().unwrap_or(0);
                return height >= 150 && height <= 193;
            }
        }
        false
    }
    fn is_valid_hgt_in(&self) -> bool {
        if let Some(captures) = HGT_IN.captures(self.content) {
            if let Some(height) = captures.get(1) {
                let height = height.as_str().parse::<i32>().unwrap_or(0);
                return height >= 59 && height <= 76;
            }
        }
        false
    }

    fn is_valid_hcl(&self) -> bool {
        HCL.is_match(self.content)
    }

    fn is_valid_ecl(&self) -> bool {
        ECL.is_match(self.content)
    }

    fn is_valid_pid(&self) -> bool {
        PID.is_match(self.content)
    }
}

pub fn solve() {
    let file = fs::read_to_string("inputs/day04.txt").unwrap();

    let passports: Vec<_> = file
        .split("\n\r")
        .map(|content| Passport::new(content))
        .collect();

    let valid_count: i32 = passports
        .iter()
        .map(|passport| if passport.is_valid_simple() { 1 } else { 0 })
        .sum();

    println!(
        "Number of valid passport with simple method: {}",
        valid_count
    );

    let valid_count: i32 = passports
        .iter()
        .map(|passport| if passport.is_valid() { 1 } else { 0 })
        .sum();

    println!(
        "Number of valid passport with complex method: {}",
        valid_count
    );
}
