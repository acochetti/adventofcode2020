use solutions::dayone_one::repair_report;

mod solutions;

fn main() {
    match repair_report("/Users/acochetti/Documents/day1_1_input") {
        Ok(number) => { print!("{}", number)},
        Err(error) => { print!("Error occurred during search: {}", error)}
    }
}
