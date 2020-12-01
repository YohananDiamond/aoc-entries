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
