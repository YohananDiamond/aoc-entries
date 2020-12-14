mod aoc;

fn main() {
    aoc::start("day9_example.txt", |i| part1(i, 5), |i| part2(i, 5));
    aoc::start("day9.txt", |i| part1(i, 25), |i| part2(i, 25));
}

fn part1(input: &str, preamble_size: usize) -> Result<String, String> {
    let data: Vec<_> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.parse()
                .map_err(|e| format!("Failed to parse {:?}: {}", line, e))
        })
        .collect::<Result<_, _>>()?;

    Ok(format!(
        "{}",
        get_first_invalid(&data, preamble_size)
            .ok_or_else(|| format!("Couldn't find invalid number"))?
    ))
}

fn part2(input: &str, preamble_size: usize) -> Result<String, String> {
    let data: Vec<_> = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|line| {
            line.parse()
                .map_err(|e| format!("Failed to parse {:?}: {}", line, e))
        })
        .collect::<Result<_, _>>()?;

    let invalid = get_first_invalid(&data, preamble_size)
        .ok_or_else(|| format!("Couldn't find invalid number"))?;

    let sum_range = get_sum_range_for(invalid, &data)
        .ok_or_else(|| format!("Couldn't find 2+ number range that sums up to {}", invalid))?;

    Ok(format!(
        "{}",
        sum_range.iter().fold(sum_range[0], |a, &b| a.min(b))
            + sum_range.iter().fold(sum_range[0], |a, &b| a.max(b)),
    ))
}

fn get_first_invalid(data: &[usize], preamble_size: usize) -> Option<usize> {
    'blk: for (idx, &num) in data.iter().enumerate().skip(preamble_size) {
        let slice = &data[idx - preamble_size..idx];

        for (i, n) in slice.iter().enumerate() {
            for (i2, n2) in slice.iter().enumerate() {
                if i != i2 && n + n2 == num {
                    continue 'blk;
                }
            }
        }

        return Some(num);
    }

    None
}

fn get_sum_range_for(num: usize, main_range: &[usize]) -> Option<&[usize]> {
    if main_range.len() < 2 {
        return None;
    }

    for idx in 0..(main_range.len() - 1) {
        let mut len = 2;
        while idx + len < main_range.len() {
            let range = &main_range[idx..idx + len];

            if range.iter().fold(0, |a, &b| a + b) == num {
                return Some(range);
            }

            len += 1;
        }
    }

    None
}
