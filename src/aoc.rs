use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub fn input_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("input");

    p
}

pub fn read_input_file(filename: &str) -> Result<String, io::Error> {
    let mut input_dir = input_dir();
    input_dir.push(filename);
    let mut file = File::open(input_dir.as_path())?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;

    Ok(string)
}

pub fn start<F, G>(filename: &str, part1: F, part2: G)
where
    F: Fn(&str) -> Result<String, String>,
    G: Fn(&str) -> Result<String, String>,
{
    println!("Current input file: {}", filename);
    println!("{{");

    match read_input_file(filename) {
        Ok(input) => {
            match part1(&input) {
                Ok(o) => println!("Part 1 (OK): {}", o),
                Err(e) => println!("Part 1 (ERR): {}", e),
            }

            match part2(&input) {
                Ok(o) => println!("Part 2 (OK): {}", o),
                Err(e) => println!("Part 2 (ERR): {}", e),
            }
        }
        Err(e) => {
            println!("Failed to load file: {}", e);
        }
    };

    println!("}}");
}

/// Prints self and returns again the same value.
/// Useful for debugging.
pub trait PrintAndForward: Sized {
    fn print_forward(self) -> Self;
}

impl<T> PrintAndForward for T
where
    T: Sized + std::fmt::Display,
{
    fn print_forward(self) -> Self {
        println!("{}", self);
        self
    }
}

/// Prints the debug representation of self and returns again the same value.
/// Useful for debugging.
pub trait DebugAndForward: Sized {
    fn debug_forward(self) -> Self;
}

impl<T> DebugAndForward for T
where
    T: Sized + std::fmt::Debug,
{
    fn debug_forward(self) -> Self {
        println!("{:?}", self);
        self
    }
}
