use std::io::{Error, BufReader, BufRead};
use std::fs::File;
use regex::Regex;

pub fn validate_passwords(path: String) -> Result<usize, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let number_of_valid_passwords = buffered.lines()
        .map(| result | result.unwrap())
        .filter(| line | { validate_password(parse_line(line)) })
        .count();

    Ok(number_of_valid_passwords)
}

fn validate_password((position_one, position_two, character, password) : (usize, usize, char, &str)) -> bool {
    let char_is_in_position_one = password.chars().nth(position_one).unwrap() == character;
    let char_is_in_position_two = password.chars().nth(position_two).unwrap() == character;
    
    char_is_in_position_one != char_is_in_position_two
}

fn parse_line(line : &str) -> (usize, usize, char, &str) {
    let regex = Regex::new(r"-|\s|:").unwrap();

    let split_line = regex.split(&line).collect::<Vec<&str>>();

    (
        split_line[0].parse::<usize>().unwrap() - 1,
        split_line[1].parse::<usize>().unwrap() - 1,
        split_line[2].parse::<char>().unwrap(),
        split_line[4]
    )
}

#[cfg(test)]
mod tests {
    use super::validate_passwords;

    #[test]
    fn solve_day_two_two() {
        let test_input_path = format!("{}/src/solutions/day_2/day_2_input.txt", env!("CARGO_MANIFEST_DIR"));

        match validate_passwords(test_input_path) {
            Ok(number) => { print!("{}", number)},
            Err(error) => { print!("Error occurred during search: {}", error)}
        }
    }
}