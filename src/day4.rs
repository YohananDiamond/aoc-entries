#![allow(unused_variables)]
#![allow(dead_code)]

mod aoc;

fn main() {
    aoc::start("day4.txt", part1, part2);
}

struct PassportData<'a> {
    byr: &'a str,
    iyr: &'a str,
    eyr: &'a str,
    hgt: &'a str,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
    cid: Option<&'a str>,
}

impl<'a> PassportData<'a> {
    pub fn new_part1(pass_data: &'a str) -> Result<Option<Self>, String> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        for line in pass_data.split("\n").filter(|s| !s.is_empty()) {
            for kval in line.split(" ") {
                let (key, val) = {
                    let split: Vec<&str> = kval.split(":").collect();
                    if split.len() != 2 {
                        return Err(format!(
                            "key:value pair ({:?}) length ({}) is not 2",
                            split,
                            split.len()
                        ));
                    }

                    (split[0], split[1])
                };

                // pain and suffering
                match key {
                    "byr" => match byr {
                        Some(_) => return Ok(None),
                        None => byr = Some(val),
                    },
                    "iyr" => match iyr {
                        Some(_) => return Ok(None),
                        None => iyr = Some(val),
                    },
                    "eyr" => match eyr {
                        Some(_) => return Ok(None),
                        None => eyr = Some(val),
                    },
                    "hgt" => match hgt {
                        Some(_) => return Ok(None),
                        None => hgt = Some(val),
                    },
                    "hcl" => match hcl {
                        Some(_) => return Ok(None),
                        None => hcl = Some(val),
                    },
                    "ecl" => match ecl {
                        Some(_) => return Ok(None),
                        None => ecl = Some(val),
                    },
                    "pid" => match pid {
                        Some(_) => return Ok(None),
                        None => pid = Some(val),
                    },
                    "cid" => match cid {
                        Some(_) => return Ok(None),
                        None => cid = Some(val),
                    },
                    other => return Ok(None),
                }
            }
        }

        Ok(Some(Self {
            byr: match byr {
                Some(thing) => thing,
                None => return Ok(None),
            },
            iyr: match iyr {
                Some(thing) => thing,
                None => return Ok(None),
            },
            eyr: match eyr {
                Some(thing) => thing,
                None => return Ok(None),
            },
            hgt: match hgt {
                Some(thing) => thing,
                None => return Ok(None),
            },
            hcl: match hcl {
                Some(thing) => thing,
                None => return Ok(None),
            },
            ecl: match ecl {
                Some(thing) => thing,
                None => return Ok(None),
            },
            pid: match pid {
                Some(thing) => thing,
                None => return Ok(None),
            },
            cid: cid,
        }))
    }

    pub fn new_part2(pass_data: &'a str) -> Result<Option<Self>, String> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        for line in pass_data.split("\n").filter(|s| !s.is_empty()) {
            for kval in line.split(" ") {
                let (key, val) = {
                    let split: Vec<&str> = kval.split(":").collect();
                    if split.len() != 2 {
                        return Err(format!(
                            "key:value pair ({:?}) length ({}) is not 2",
                            split,
                            split.len()
                        ));
                    }

                    (split[0], split[1])
                };

                // pain and suffering
                match key {
                    "byr" => match byr {
                        Some(_) => return Ok(None),
                        None => byr = Some(val),
                    },
                    "iyr" => match iyr {
                        Some(_) => return Ok(None),
                        None => iyr = Some(val),
                    },
                    "eyr" => match eyr {
                        Some(_) => return Ok(None),
                        None => eyr = Some(val),
                    },
                    "hgt" => match hgt {
                        Some(_) => return Ok(None),
                        None => hgt = Some(val),
                    },
                    "hcl" => match hcl {
                        Some(_) => return Ok(None),
                        None => hcl = Some(val),
                    },
                    "ecl" => match ecl {
                        Some(_) => return Ok(None),
                        None => ecl = Some(val),
                    },
                    "pid" => match pid {
                        Some(_) => return Ok(None),
                        None => pid = Some(val),
                    },
                    "cid" => match cid {
                        Some(_) => return Ok(None),
                        None => cid = Some(val),
                    },
                    other => return Ok(None),
                }
            }
        }

        Ok(Some(Self {
            byr: match byr {
                Some(thing) if matches!(parse_year(thing), Some(1920..=2002)) => thing,
                _ => return Ok(None),
            },
            iyr: match iyr {
                Some(thing) if matches!(parse_year(thing), Some(2010..=2020)) => thing,
                _ => return Ok(None),
            },
            eyr: match eyr {
                Some(thing) if matches!(parse_year(thing), Some(2020..=2030)) => thing,
                _ => return Ok(None),
            },
            hgt: match hgt {
                Some(thing) if matches!(parse_cm(thing), Some(150..=193)) => thing,
                Some(thing) if matches!(parse_in(thing), Some(59..=76)) => thing,
                _ => return Ok(None),
            },
            hcl: match hcl {
                Some(thing) if is_hex_color(thing) => thing,
                _ => return Ok(None),
            },
            ecl: match ecl {
                Some(thing) if is_eye_color(thing) => thing,
                _ => return Ok(None),
            },
            pid: match pid {
                Some(thing) if is_passport_id(thing) => thing,
                _ => return Ok(None),
            },
            cid: cid,
        }))
    }
}

fn part1(input: &str) -> Result<String, String> {
    let passports = parse_and_validate(input, PassportData::new_part1)?;

    Ok(format!("{}", passports.len()))
}

fn part2(input: &str) -> Result<String, String> {
    let passports = parse_and_validate(input, PassportData::new_part2)?;

    Ok(format!("{}", passports.len()))
}

fn parse_and_validate<'a, F>(input: &'a str, parser: F) -> Result<Vec<PassportData<'a>>, String>
where
    F: Fn(&'a str) -> Result<Option<PassportData<'a>>, String>,
{
    let mut result = Vec::new();

    for entry in input.split("\n\n") {
        if let Some(pass) = parser(entry)? {
            result.push(pass);
        }
    }

    Ok(result)
}

fn parse_year(string: &str) -> Option<u32> {
    if string.len() == 4 {
        string.parse().ok()
    } else {
        None
    }
}

fn parse_cm(string: &str) -> Option<u32> {
    if string.ends_with("cm") {
        (&string[..string.len() - 2]).parse().ok()
    } else {
        None
    }
}

fn parse_in(string: &str) -> Option<u32> {
    if string.ends_with("in") {
        (&string[..string.len() - 2]).parse().ok()
    } else {
        None
    }
}

fn is_hex_color(string: &str) -> bool {
    string.len() == 7
        && string.chars().nth(0).unwrap() == '#'
        && string
            .chars()
            .skip(1)
            .fold(true, |result, item| result && matches!(item, '0'..='9' | 'a'..='f'))
}

fn is_eye_color(string: &str) -> bool {
    matches!(
        string,
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    )
}

fn is_passport_id(string: &str) -> bool {
    string.len() == 9
        && string.chars().fold(true, |result, item| result && item.is_digit(10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cm_tests() {
        assert_eq!(parse_cm("30cm"), Some(30));
        assert_eq!(parse_cm("30"), None);
        assert_eq!(parse_cm("30in"), None);
    }

    #[test]
    fn parse_in_tests() {
        assert_eq!(parse_in("30cm"), None);
        assert_eq!(parse_in("30"), None);
        assert_eq!(parse_in("30in"), Some(30));
    }
}
