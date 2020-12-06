use std::fs::File;
use std::io::{BufRead, BufReader};

/*
byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)

hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)   - OPTIONAL
 */

#[derive(Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,

    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport {
    pub fn new() -> Self {
        Passport {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
            cid: false,
        }
    }

    pub fn scan_line(&mut self, line: &str) {
        if !self.byr && line.contains("byr:") {
            self.byr = true;
        }
        if !self.iyr && line.contains("iyr:") {
            self.iyr = true;
        }
        if !self.eyr && line.contains("eyr:") {
            self.eyr = true;
        }
        if !self.hgt && line.contains("hgt:") {
            self.hgt = true;
        }
        if !self.hcl && line.contains("hcl:") {
            self.hcl = true;
        }
        if !self.ecl && line.contains("ecl:") {
            self.ecl = true;
        }
        if !self.pid && line.contains("pid:") {
            self.pid = true;
        }
        if !self.cid && line.contains("cid:") {
            self.cid = true;
        }
    }

    pub fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}

pub fn solve() -> i32 {
    let filename = "input/day4.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut number = 0;

    let mut chunk: Vec<String> = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            if line == "" {
                if check_chunk(&chunk) {
                    number += 1;
                }
                chunk.clear();
            } else {
                chunk.push(line);
            }
        }
    }
    if !chunk.is_empty() && check_chunk(&chunk) {
        number += 1;
    }

    number
}

fn check_chunk(chunk: &[String]) -> bool {
    let mut passport = Passport::new();
    chunk.iter().for_each(|line| passport.scan_line(line));
    passport.is_valid()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(check_chunk(&vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_owned(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_owned()
        ]));
        assert!(check_chunk(&vec![
            "hcl:#ae17e1 iyr:2013".to_owned(),
            "eyr:2024".to_owned(),
            "ecl:brn pid:760753108 byr:1931".to_owned(),
            "hgt:179cm".to_owned(),
        ]))
    }
}
