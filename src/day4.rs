#![allow(unused_variables)]
#![allow(dead_code)]

mod aoc;

fn main() {
    aoc::start("day4.txt", part1, part2);
}

pub fn part1_is_valid(pass: &str) -> Result<bool, String> {
    let mut byr = None;
    let mut iyr = None;
    let mut eyr = None;
    let mut hgt = None;
    let mut hcl = None;
    let mut ecl = None;
    let mut pid = None;
    let mut cid = None;

    for line in pass.split("\n").filter(|s| !s.is_empty()) {
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
                    Some(_) => return Ok(false),
                    None => byr = Some(val),
                },
                "iyr" => match iyr {
                    Some(_) => return Ok(false),
                    None => iyr = Some(val),
                },
                "eyr" => match eyr {
                    Some(_) => return Ok(false),
                    None => eyr = Some(val),
                },
                "hgt" => match hgt {
                    Some(_) => return Ok(false),
                    None => hgt = Some(val),
                },
                "hcl" => match hcl {
                    Some(_) => return Ok(false),
                    None => hcl = Some(val),
                },
                "ecl" => match ecl {
                    Some(_) => return Ok(false),
                    None => ecl = Some(val),
                },
                "pid" => match pid {
                    Some(_) => return Ok(false),
                    None => pid = Some(val),
                },
                "cid" => match cid {
                    Some(_) => return Ok(false),
                    None => cid = Some(val),
                },
                other => return Ok(false),
            }
        }
    }

    if byr.is_none() {
        return Ok(false);
    }

    if iyr.is_none() {
        return Ok(false);
    }

    if eyr.is_none() {
        return Ok(false);
    }

    if hgt.is_none() {
        return Ok(false);
    }

    if hcl.is_none() {
        return Ok(false);
    }

    if ecl.is_none() {
        return Ok(false);
    }

    if pid.is_none() {
        return Ok(false);
    }

    Ok(true)
}

pub fn part2_is_valid(pass: &str) -> Result<bool, String> {
    let mut byr = None;
    let mut iyr = None;
    let mut eyr = None;
    let mut hgt = None;
    let mut hcl = None;
    let mut ecl = None;
    let mut pid = None;
    let mut cid = None;

    for line in pass.split("\n").filter(|s| !s.is_empty()) {
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
                    Some(_) => return Ok(false),
                    None => byr = Some(val),
                },
                "iyr" => match iyr {
                    Some(_) => return Ok(false),
                    None => iyr = Some(val),
                },
                "eyr" => match eyr {
                    Some(_) => return Ok(false),
                    None => eyr = Some(val),
                },
                "hgt" => match hgt {
                    Some(_) => return Ok(false),
                    None => hgt = Some(val),
                },
                "hcl" => match hcl {
                    Some(_) => return Ok(false),
                    None => hcl = Some(val),
                },
                "ecl" => match ecl {
                    Some(_) => return Ok(false),
                    None => ecl = Some(val),
                },
                "pid" => match pid {
                    Some(_) => return Ok(false),
                    None => pid = Some(val),
                },
                "cid" => match cid {
                    Some(_) => return Ok(false),
                    None => cid = Some(val),
                },
                other => return Ok(false),
            }
        }
    }

    match byr {
        Some(x) if matches!(parse_year(x), Some(1920..=2002)) => {},
        _ => return Ok(false),
    }

    match iyr {
        Some(x) if matches!(parse_year(x), Some(2010..=2020)) => {},
        _ => return Ok(false),
    }

    match eyr {
        Some(x) if matches!(parse_year(x), Some(2020..=2030)) => {},
        _ => return Ok(false),
    }

    match hgt {
        Some(x) if matches!(parse_cm(x), Some(150..=193)) => {}
        Some(x) if matches!(parse_in(x), Some(59..=76)) => {}
        _ => return Ok(false),
    }

    match hcl {
        Some(x) if is_hex_color(x) => {}
        _ => return Ok(false),
    }

    match ecl {
        Some(x) if is_eye_color(x) => {}
        _ => return Ok(false),
    }

    match pid {
        Some(x) if is_passport_id(x) => {}
        _ => return Ok(false),
    }

    Ok(true)
}

fn part1(input: &str) -> Result<String, String> {
    Ok(format!("{}", parse_and_validate(input, part1_is_valid)?))
}

fn part2(input: &str) -> Result<String, String> {
    Ok(format!("{}", parse_and_validate(input, part2_is_valid)?))
}

fn parse_and_validate<'a, F>(input: &'a str, parser_fn: F) -> Result<usize, String>
where
    F: Fn(&'a str) -> Result<bool, String>,
{
    Ok(input
        .split("\n\n")
        .map(parser_fn)
        .collect::<Result<Vec<_>, String>>()?
        .iter()
        .filter(|&&x| x)
        .count())
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
        && string.chars().skip(1).fold(true, |result, item| {
            result && matches!(item, '0'..='9' | 'a'..='f')
        })
}

fn is_eye_color(string: &str) -> bool {
    matches!(
        string,
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    )
}

fn is_passport_id(string: &str) -> bool {
    string.len() == 9
        && string
            .chars()
            .fold(true, |result, item| result && item.is_digit(10))
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
