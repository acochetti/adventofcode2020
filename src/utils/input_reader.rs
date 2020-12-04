use std::fs::File;
use std::io::{Error, BufReader, BufRead};

pub fn read_file(path: &str) -> Result<Vec<String>, Error> {
    let input_file = File::open(path)?;
    let input_buffer = BufReader::new(input_file);
    
    Ok(input_buffer.lines().map(|result | result.unwrap()).collect())
}