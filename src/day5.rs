mod aoc;
mod math;

use math::Point2;

use std::collections::HashSet;

fn main() {
    aoc::start("day5.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    Ok(format!(
        "{}",
        input
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|ins| seat_pos(ins).map(seat_id))
            .collect::<Result<Vec<u32>, String>>()?
            .iter()
            .fold(0, |result, id| result.max(*id))
    ))
}

fn part2(input: &str) -> Result<String, String> {
    let seats = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|ins| seat_pos(ins).map(seat_id))
        .collect::<Result<HashSet<u32>, String>>()?;

    let max_seat_id = seat_id((127, 7));

    'blk: loop {
        for i in 0..max_seat_id + 1 {
            if i != 0
                && i != max_seat_id
                && !seats.contains(&i)
                && seats.contains(&(i - 1))
                && seats.contains(&(i + 1))
            {
                break 'blk Ok(format!("{}", i));
            }
        }

        break Err(format!("Could not find seat ID with specified conditions"));
    }
}

fn seat_id((row, column): (u32, u32)) -> u32 {
    row * 8 + column
}

fn seat_pos(instruction: &str) -> Result<(u32, u32), String> {
    let mut range = Point2 {
        x: (0, 7),   // column
        y: (0, 127), // row
    };

    if instruction.len() != 10 {
        return Err(format!(
            "Length of instruction should be exactly of 10 characters (found {}).",
            instruction.len()
        ));
    }

    range.y = (&instruction[0..7])
        .chars()
        .enumerate()
        .try_fold(range.y, |old, (i, half)| match half {
            'F' => Ok(lower_half(old)),
            'B' => Ok(upper_half(old)),
            other => Err(format!(
                "Unknown meaning for char {:?} at index #{}",
                other, i
            )),
        })?;

    // for (i, half) in (&instruction[0..7]).chars().enumerate() {
    //     match half {
    //         'F' => range.y = lower_half(range.y),
    //         'B' => range.y = upper_half(range.y),
    //         half => {
    //             return Err(format!(
    //                 "Unknown meaning for char {:?} at index #{}",
    //                 half, i
    //             ))
    //         }
    //     }
    // }

    range.x = (&instruction[7..10])
        .chars()
        .enumerate()
        .try_fold(range.x, |old, (i, half)| match half {
            'L' => Ok(lower_half(old)),
            'R' => Ok(upper_half(old)),
            other => Err(format!(
                "Unknown meaning for char {:?} at index #{}",
                other, i
            )),
        })?;

    // for (i, half) in (&instruction[7..10]).chars().enumerate() {
    //     match half {
    //         'L' => range.x = lower_half(range.x),
    //         'R' => range.x = upper_half(range.x),
    //         half => {
    //             return Err(format!(
    //                 "Unknown meaning for char {:?} at index #{}",
    //                 half, i
    //             ))
    //         }
    //     }
    // }

    let (row, _) = range.y; // lower value
    let (_, column) = range.x; // upper value

    Ok((row, column))
}

fn lower_half((lower, upper): (u32, u32)) -> (u32, u32) {
    (lower, (upper - lower) / 2 + lower)
}

fn upper_half((lower, upper): (u32, u32)) -> (u32, u32) {
    ((upper - lower) / 2 + lower + 1, upper)
}
