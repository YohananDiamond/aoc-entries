mod aoc;

fn main() {
    aoc::start("day1.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let ints = common(input)?;

    loop {
        for (i, entry) in ints.iter().enumerate() {
            for (i2, entry2) in ints.iter().enumerate() {
                if i != i2 && entry + entry2 == 2020 {
                    return Ok(format!("{}", entry * entry2));
                }
            }
        }

        return Ok(format!("Couldn't find any."));
    }
}

fn part2(input: &str) -> Result<String, String> {
    let ints = common(input)?;

    loop {
        for (i, entry) in ints.iter().enumerate() {
            for (i2, entry2) in ints.iter().enumerate() {
                for (i3, entry3) in ints.iter().enumerate() {
                    if i != i2 && i2 != i3 && i != i3 && entry + entry2 + entry3 == 2020 {
                        return Ok(format!("{}", entry * entry2 * entry3));
                    }
                }
            }
        }

        return Ok(format!("Couldn't find any."));
    }
}

fn common(input: &str) -> Result<Vec<i32>, String> {
    Ok(input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to parse numbers: {}", e))?)
}
