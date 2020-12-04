use std::io::{Error};
use crate::utils::input_reader::read_file;

pub fn repair_report(path: &str) -> Result<i64, Error> {
    let mut test_input : Vec<i64> = Vec::new();
    for line in read_file(path).unwrap() {
        if let Ok(n) = line.parse::<i64>() { test_input.push(n) }
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

#[cfg(test)]
mod tests {
    use super::repair_report;
    
    #[test]
    fn solve_day_one_one() {
        let test_input_path = format!("{}/src/solutions/day_1/day_1_input.txt", env!("CARGO_MANIFEST_DIR"));
        
        match repair_report(&test_input_path) {
            Ok(number) => { print!("{}", number)},
            Err(error) => { print!("Error occurred during search: {}", error)}
        }
    }
}