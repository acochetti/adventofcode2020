use grid::*;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub fn navigate_map(path: &str, move_right: usize, move_down: usize) -> Result<i64, Error> {
    let input_file = File::open(path)?;
    let file_buffer = BufReader::new(input_file);
    
    let slope_map = create_map(file_buffer, move_right, move_down);
    
    let mut num_of_trees : i64 = 0;
    let mut continue_travel = true;
    let mut x_position : usize = 0;
    let mut y_position : usize = 0;
    while continue_travel {
        match slope_map.get(y_position, x_position) {
            Some(char) => {
                if '#'.eq(char) { num_of_trees += 1}
                x_position += move_right;
                y_position += move_down;
            }
            None => { continue_travel = false }
        }
    }
    
    Ok(num_of_trees)
}

fn create_map(file_buffer: BufReader<File>, move_right: usize, move_down: usize) -> Grid<char> {
    let mut map : Grid<char> = grid!();
    
    for line_result in file_buffer.lines() {
        let line = line_result.unwrap();
        //                             this is hacky and wouldn't work if the input file was longer
        map.push_row(line.repeat((line.len()) * move_right * move_down).chars().collect())
    }
    
    map
}

#[cfg(test)]
mod tests {
    use super::navigate_map;

    #[test]
    fn solve_day_three_one() {
        let test_input_path = format!("{}/src/solutions/day_3/day_3_input.txt", env!("CARGO_MANIFEST_DIR"));
        
        println!("Number of trees encountered: {}", navigate_map(&test_input_path, 3, 1).unwrap());
    }
    
    #[test]
    fn solve_day_three_two() {
        let test_input_path = format!("{}/src/solutions/day_3/day_3_input.txt", env!("CARGO_MANIFEST_DIR"));
        let test_input_slopes = vec!([1, 1], [3, 1], [5, 1], [7, 1], [1, 2]);
        
        let tree_product : i64 = test_input_slopes.iter()
            .map(|input| navigate_map(&test_input_path, input[0], input[1]).unwrap())
            .product();

        println!("Product of trees encountered: {}", tree_product);
    }
}