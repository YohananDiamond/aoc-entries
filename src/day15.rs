mod aoc;
use aoc::parse_number;

use std::collections::HashMap;

type Number = usize;
type Turn = usize;

fn main() {
    aoc::start_with_file("day15_example.txt", part1, part2);
    aoc::start_with_file("day15.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let numbers: Vec<Number> = input
        .split(",")
        .filter(|&n| n != "\n")
        .map(|n| parse_number(n.trim()))
        .collect::<Result<_, _>>()?;

    Ok(format!("{}", common(&numbers, 2020)?))
}

fn part2(input: &str) -> Result<String, String> {
    let numbers: Vec<Number> = input
        .split(",")
        .filter(|&n| n != "\n")
        .map(|n| parse_number(n.trim()))
        .collect::<Result<_, _>>()?;

    Ok(format!("{}", common(&numbers, 30000000)?))
}

fn common(numbers: &[Number], stop_at: Turn) -> Result<Number, String> {
    // This seemed so easy but somehow I struggled a lot trying to bring this one to life.
    //
    // So I got help.
    // https://davidlozzi.com/2020/12/15/advent-of-code-day-15/

    if numbers.is_empty() {
        return Err(format!("Input is empty"));
    } else if has_repeated(&numbers) {
        // I'm lazy
        return Err(format!("Unhandled case - number list with repeated starting numbers"));
    }

    let mut spoken_numbers: HashMap<Number, Turn> = HashMap::new();

    for (i, &number) in numbers.iter().enumerate() {
        let turn = i + 1;
        // println!("TURN {}; starting number {}", turn, number);
        spoken_numbers.insert(number, turn);
    }

    // This is the turn immediately after the starting numbers.
    // I was previously worried about how to calculate the first previous number considered, but I've noticed that, on
    // non-repeated starting number lists, it's always zero.
    let mut previous_number = 0;
    // println!("TURN {}; returning 0", numbers.len() + 1);

    // These are the turns starting from the turn after the last starting turn, and ending on the STOP.
    for turn in (numbers.len() + 2)..=stop_at {
        // if turn % 1000000 == 0 {
        //     println!("TURN {} out of {}", turn, stop_at);
        // }

        // print!("TURN {}; PN = {}: ", turn, previous_number);

        use std::collections::hash_map::Entry::*;

        let previous_turn = turn - 1;

        match spoken_numbers.entry(previous_number) {
            Occupied(mut entry) => {
                let last_spoken_turn = *entry.get(); // the last turn this number was spoken in
                entry.insert(previous_turn); // the last turn it was spoken in is now the previous turn
                // println!("OCCUPIED (last spoken at T{}), returning {}", last_spoken_turn, previous_turn - last_spoken_turn);
                previous_number = previous_turn - last_spoken_turn; // update the "previous number"
            },
            Vacant(entry) => {
                entry.insert(previous_turn); // the last turn it was spoken in is now the previous turn
                // println!("VACANT, returning 0");
                previous_number = 0; // it has been said zero times before, so 0
            },
        }
    }

    Ok(previous_number)
}

fn has_repeated<T>(slice: &[T]) -> bool
where
    T: Eq,
{
    (0..slice.len())
        .any(|i| slice[i+1..].contains(&slice[i]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repetition() {
        assert!(!has_repeated(&[1, 2, 3, 5]));
        assert!(has_repeated(&[1, 2, 3, 2, 5]));
    }
}
