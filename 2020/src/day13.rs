mod aoc;
mod math;

fn main() {
    aoc::start("day13_example.txt", part1, part2);
    aoc::start("day13.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let mut split = input.split("\n").filter(|line| !line.is_empty());

    let init_timestamp = match split.next() {
        Some(line) => line
            .parse::<usize>()
            .map_err(|e| format!("Failed to parse number {:?}: {}", line, e))?,
        None => return Err(format!("Empty input (expected initial timestamp)")),
    };

    let ids: Vec<usize> = match split.next() {
        Some(line) => line
            .split(",")
            .filter(|&c| c != "x")
            .map(|n| {
                n.parse()
                    .map_err(|e| format!("Failed to parse number {:?}: {}", n, e))
            })
            .collect::<Result<Vec<_>, _>>()?,
        None => return Err(format!("Incomplete input (expected ID list)")),
    };

    let (timestamp, id) = part1_earliest_timestamp_id(&ids, init_timestamp);

    Ok(format!("{}", (timestamp - init_timestamp) * id))
}

fn part2(input: &str) -> Result<String, String> {
    let mut split = input.split("\n").filter(|line| !line.is_empty());

    let init_timestamp = match split.next() {
        Some(line) => line
            .parse::<usize>()
            .map_err(|e| format!("Failed to parse number {:?}: {}", line, e))?,
        None => return Err(format!("Empty input (expected initial timestamp)")),
    };

    let ids = match split.next() {
        Some(line) => line
            .split(",")
            .map(|element| {
                if element == "x" {
                    Ok(None)
                } else {
                    Ok(Some(element.parse().map_err(|e| {
                        format!("Failed to parse number {:?}: {}", element, e)
                    })?))
                }
            })
            .collect::<Result<Vec<Option<usize>>, String>>()?,
        None => return Err(format!("Incomplete input (expected ID list)")),
    };

    Ok(format!(
        "{}",
        part2_earliest_timestamp(&ids, init_timestamp)
    ))
}

fn part1_earliest_timestamp_id(ids: &[usize], init_timestamp: usize) -> (usize, usize) {
    for timestamp in init_timestamp.. {
        for &id in ids {
            if timestamp % id == 0 {
                return (timestamp, id);
            }
        }
    }

    unreachable!()
}

fn part2_earliest_timestamp(ids: &[Option<usize>], init_timestamp: usize) -> usize {
    // Slow method (by me)
    // {
    //     let mut timestamp = init_timestamp;
    //     loop {
    //         if ids.iter().enumerate().fold(true, |prev, (i, &id)| {
    //             if let Some(id) = id {
    //                 prev && (timestamp + i) % id as usize == 0
    //             } else {
    //                 prev
    //             }
    //         }) {
    //             return Ok(timestamp);
    //         }

    //         timestamp += 1;
    //     }
    // }

    // A faster/cheaper method
    //
    // Adapted from @seoane8's solution on a DEV.to post:
    // Solution (comment): https://dev.to/seoane8/comment/19673
    // Post: https://dev.to/rpalo/advent-of-code-2020-solution-megathread-day-13-shuttle-search-313f
    {
        use std::collections::HashMap;

        // I saw this trick somewhere on the post above and it's seemingly pretty useful to make things more readable.
        type BusID = usize;
        type Offset = usize;

        let offset_map: HashMap<BusID, Offset> = ids
            .iter()
            .enumerate()
            .filter_map(|(i, &bus)| {
                bus.map(|bus_id| {
                    let bus_id = bus_id as isize;
                    let i = i as isize;

                    // the same as doing -i % bus_id in python
                    let id_with_offset = ((-i % bus_id) + bus_id) % bus_id;

                    (bus_id as BusID, id_with_offset as Offset)
                })
            })
            .collect();

        let mut timestamp = init_timestamp;
        let mut increment = 1;

        for (bus_id, &id_with_offset) in offset_map.iter() {
            while timestamp % bus_id != id_with_offset {
                timestamp += increment;
            }

            increment *= bus_id;
        }

        timestamp
    }
}
