#![allow(unused)]

use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;

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

#[inline(always)]
pub fn start<F, G>(filename: &str, part1: F, part2: G)
where
    F: Fn(&str) -> Result<String, String>,
    G: Fn(&str) -> Result<String, String>,
{
    start_with_file(filename, part1, part2);
}

pub fn start_with_file<F, G>(filename: &str, part1: F, part2: G)
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

pub fn start_with_input<F, G>(input: &str, part1: F, part2: G)
where
    F: Fn(&str) -> Result<String, String>,
    G: Fn(&str) -> Result<String, String>,
{
    println!("Current input: {:?}", input);
    println!("{{");

    match part1(&input) {
        Ok(o) => println!("Part 1 (OK): {}", o),
        Err(e) => println!("Part 1 (ERR): {}", e),
    }

    match part2(&input) {
        Ok(o) => println!("Part 2 (OK): {}", o),
        Err(e) => println!("Part 2 (ERR): {}", e),
    }

    println!("}}");
}

/// Prints self and returns again the same value.
/// Useful for debugging.
#[deprecated]
pub trait PrintAndForward: Sized {
    fn print_forward(self) -> Self;
}

#[allow(deprecated)]
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
#[deprecated]
pub trait DebugAndForward: Sized {
    fn debug_forward(self) -> Self;
}

#[allow(deprecated)]
impl<T> DebugAndForward for T
where
    T: Sized + std::fmt::Debug,
{
    fn debug_forward(self) -> Self {
        println!("{:?}", self);
        self
    }
}

/// Forwards a value through a function.
pub trait Forward: Sized {
    fn forward<F, O>(self, f: F) -> O
    where
        F: FnOnce(Self) -> O,
    {
        f(self)
    }
}

impl<T> Forward for T {}

/// Checks if an expression `expr` matches a set of patterns.
/// If it matches, returns `Ok(expr)`
/// If it doesnt, returns `Err(msg)` where `msg` is a string containing an error message.
#[macro_export]
macro_rules! expect_match {
    ($expression:expr, $( $pattern:pat )|+ $( if $guard:expr )? $(,)?) => ({
        let expr = $expression;

        match &expr {
            $( $pattern )|+ $( if $guard )? => ::std::result::Result::Ok(expr),
            _ => ::std::result::Result::Err(format!("Expression {{ {} }} -> {:?} does not match with {}", stringify!($expression), expr, stringify!($( $pattern )|+ $( if $guard )?))),
        }
    })
}

/// Parses a number, returning `Ok(num)` if successful or `Err(e)`, where `e` is a sring message with a detailed
/// description of the errror.
pub fn parse_number<N, I>(input: I) -> Result<N, String>
where
    N: FromStr,
    N::Err: std::fmt::Display,
    I: AsRef<str> + std::fmt::Debug,
{
    input
        .as_ref()
        .parse::<N>()
        .map_err(|err| format!("Failed to parse number string {:?}: {}", input, err))
}

#[allow(dead_code)]
pub fn dummy_part(_: &str) -> Result<String, String> {
    Ok(format!("<Dummy>"))
}
