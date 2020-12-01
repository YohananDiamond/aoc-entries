use std::io;

mod aoc;

fn main() -> io::Result<()> {
    let input = aoc::read_input_file("day1.txt")?;
    let entries = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse numbers");

    'main: loop {
        for (i, entry) in entries.iter().enumerate() {
            for (i2, entry2) in entries.iter().enumerate() {
                if i != i2 && entry + entry2 == 2020 {
                    println!("{}", entry * entry2);
                    break 'main;
                }
            }
        }

        println!("Couldn't find any.");
        break 'main;
    }

    Ok(())
}
