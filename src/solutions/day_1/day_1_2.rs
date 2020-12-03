use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn day_one_two(path: String) -> Result<i64, Error> {
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
    let mut answer = 0;
    while test_input.len() > 0 {
        let number = test_input.pop().unwrap();
        let new_max = 2020 - number;
        
        for num in &test_input {
            if test_input.contains(&(new_max - num)) {
                answer = (new_max - num) * num * number;
                break;
            }
        }
    }

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::day_one_two;

    #[test]
    fn solve_day_one_two() {
        let test_input_path = format!("{}/src/solutions/day_1/day_1_input.txt", env!("CARGO_MANIFEST_DIR"));
        
        match day_one_two(test_input_path) {
            Ok(number) => { print!("{}", number)},
            Err(error) => { print!("Error occurred during search: {}", error)}
        }
    }
}