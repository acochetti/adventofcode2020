use std::io::{Error};
use regex::Regex;
use crate::utils::input_reader::read_file;

pub fn validate_passwords(path: &str) -> Result<usize, Error> {
    let number_of_valid_passwords = read_file(path).unwrap().iter()
        .filter(| line | { validate_password(parse_line(line)) })
        .count();
    
    Ok(number_of_valid_passwords)
}

fn validate_password((minimum, maximum, character, password) : (usize, usize, char, &str)) -> bool {
    let chars_in_password : Vec<char> = password.chars()
        .filter(| char | character.eq_ignore_ascii_case(char))
        .collect();
    
    (minimum..=maximum).contains(&chars_in_password.len())
}

fn parse_line(line : &str) -> (usize, usize, char, &str) {
    let regex = Regex::new(r"-|\s|:").unwrap();
    
    let split_line = regex.split(&line).collect::<Vec<&str>>();
    
    (
        split_line[0].parse::<usize>().unwrap(),
        split_line[1].parse::<usize>().unwrap(),
        split_line[2].parse::<char>().unwrap(),
        split_line[4]
    )
}

#[cfg(test)]
mod tests {
    use super::validate_passwords;

    #[test]
    fn solve_day_two_one() {
        let test_input_path = format!("{}/src/solutions/day_2/day_2_input.txt", env!("CARGO_MANIFEST_DIR"));

        match validate_passwords(&test_input_path) {
            Ok(number) => { print!("{}", number)},
            Err(error) => { print!("Error occurred during search: {}", error)}
        }
    }
}