mod aoc;

fn main() {
    aoc::start("day2.txt", part1, part2);
}

/// Common code between part1 and part2.
fn common<F>(input: &str, fun: F) -> Result<String, String>
where
    F: FnMut(&&Entry) -> bool,
{
    let count = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(Entry::new)
        .collect::<Result<Vec<_>, String>>()
        .map_err(|e| format!("Failed to parse line: {}", e))?
        .iter()
        .filter(fun)
        .count();

    Ok(format!("{}", count))
}

fn part1(input: &str) -> Result<String, String> {
    common(input, |e| e.is_valid_part1())
}

fn part2(input: &str) -> Result<String, String> {
    common(input, |e| e.is_valid_part2())
}

struct Entry<'a> {
    nums: (u32, u32),
    chr: char,
    password: &'a str,
}

impl<'a> Entry<'a> {
    pub fn new(line: &'a str) -> Result<Self, String> {
        let parts: Vec<_> = line.split(" ").collect();

        let (num1, num2) = {
            let nums = parts
                .get(0)
                .ok_or_else(|| format!("Failed to get 'numbers' part of line"))?
                .split("-")
                .take(2)
                .map(|s| s.parse::<u32>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("Failed to parse the 'numbers' part of line: {}", e))?;

            (nums[0], nums[1])
        };

        let chr = parts
            .get(1)
            .ok_or_else(|| format!("Failed to get 'char' part of line"))?
            .chars()
            .nth(0)
            .ok_or_else(|| format!("Failed to get first character at the 'char' part of line"))?;

        let password = parts
            .get(2)
            .ok_or_else(|| format!("Failed to get 'password' part of line"))?;

        Ok(Self {
            nums: (num1, num2),
            chr: chr,
            password: password,
        })
    }

    pub fn is_valid_part1(&self) -> bool {
        let (num1, num2) = self.nums;
        let mut count = 0;

        for c in self.password.chars() {
            if c == self.chr {
                count += 1;
            }

            if count > num2 {
                return false;
            }
        }

        // no need to check count < num2 here because it would have returned already
        count >= num1
    }

    pub fn is_valid_part2(&self) -> bool {
        let is_ok = |i| {
            matches!(
                self.password.chars().nth(i - 1), // it took me way too long to realize it was i - 1 and not i + 1
                Some(c) if c == self.chr
            )
        };

        let (num1, num2) = self.nums;
        is_ok(num1 as usize) ^ is_ok(num2 as usize)
    }
}
