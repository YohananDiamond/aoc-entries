mod aoc;

fn main() {
    aoc::start("day10_example.txt", part1, part2);
    aoc::start("day10.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let numbers = get_numbers(input)?;

    let mut j1_differences = 0;
    let mut j3_differences = 0;

    for i in 0..numbers.len() {
        if i > 0 {
            let current = unsafe { numbers.get_unchecked(i) };
            let previous = unsafe { numbers.get_unchecked(i - 1) };

            match current - previous {
                1 => j1_differences += 1,
                3 => j3_differences += 1,
                _ => {}
            }
        }
    }

    Err(format!(
        "{} x {} = {}",
        j1_differences,
        j3_differences,
        j1_differences * j3_differences
    ))
}

fn part2(input: &str) -> Result<String, String> {
    let numbers = get_numbers(input)?;

    Ok(format!("{}", possible_arrangements(&numbers)))
}

fn get_numbers(input: &str) -> Result<Vec<u32>, String> {
    let mut v = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse()
                .map_err(|e| format!("Failed to parse number {:?}: {}", line, e))
        })
        .collect::<Result<Vec<u32>, String>>()?;

    v.sort();
    v.insert(0, 0);
    v.push(*v.last().unwrap() + 3);

    Ok(v)
}

fn possible_arrangements(range: &[u32]) -> usize {
    // Coded with indirect help from Todd Ginsberg
    // - https://todd.ginsberg.com/post/advent-of-code/2020/day10/

    // Make a vector with the same size as the range, but fill it with zeros.
    // The original code had a map but this way is more efficient, I suppose.
    let mut paths: Vec<usize> = std::iter::repeat(0).take(range.len()).collect();

    for (i, &num) in range.iter().enumerate() {
        paths[i] = (1..=3)
            .map(|lookback| {
                // Here, for lookback = 1, 2 or 3, we check if the element at [current element's position - lookback]
                // exists and, if so, we check if the difference between the current element and said element is at most
                // 3. If that's the case, we can get the amount of possible paths for that previous element and add it
                // to us. If it was zero, we just place 1 there or the result would end up being zero.
                if lookback <= i && num - range[i - lookback] <= 3 {
                    paths[i - lookback].max(1)
                } else {
                    0
                }
            })
            .fold(0, std::ops::Add::add);
    }

    *paths.last().unwrap()
}
