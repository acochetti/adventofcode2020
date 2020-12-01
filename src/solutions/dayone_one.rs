use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn repair_report(path: &str) -> Result<i64, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut test_input : Vec<i64> = Vec::new();
    for line in buffered.lines() {
        match line.unwrap().parse::<i64>() {
            Ok(n) => test_input.push(n),
            Err(..) => {}
        }
    }
    
    test_input.sort();
    
    let mut operand_a= 0;
    let mut operand_b= 0;
    for number in &test_input {
        if test_input.contains(&(2020 - number)) {
            operand_a = *number;
            operand_b = 2020 - number;
            break;
        }
    }
    
    Ok(operand_a * operand_b)
}