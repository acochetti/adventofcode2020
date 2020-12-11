use std::collections::HashMap;

pub fn parse_passports(input: &str) -> Vec<HashMap<&str, &str>> {
    input.split("\n\n")
        .map(|passport| {
            passport.split_whitespace()
                .map(|field| { 
                    let key_value : Vec<&str> = field.split(':').collect();
                    (key_value[0], key_value[1]) 
                })
                .collect()
        })
        .filter(|passport| passport_has_required_fields(passport))
        .collect()
}

pub fn validate_passports(input: &str) -> usize {
    parse_passports(&input).into_iter().filter(|passport| passport_is_valid(passport)).count()
}

fn passport_has_required_fields(passport: &HashMap<&str, &str>) -> bool {
    ["byr", "iyr", "eyr", "ecl", "pid", "hcl", "hgt"].iter().all(|field| passport.contains_key(field))
}

fn passport_is_valid(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => v.len() == 4 && (1920..=2002).contains(&v.parse().unwrap_or(0)),
        "iyr" => v.len() == 4 && (2010..=2020).contains(&v.parse().unwrap_or(0)),
        "eyr" => v.len() == 4 && (2020..=2030).contains(&v.parse().unwrap_or(0)),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => v.len() == 9 && v.chars().all(|char| char.is_ascii_digit()),
        "hcl" => v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|char| char.is_ascii_hexdigit()),
        "hgt" => {
            let height = v[0..(v.len() - 2)].parse::<i64>().unwrap_or(0);
            match &v[(v.len() - 2)..] {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false
            }
        },
        "cid" => true,
        _ => false
    })
}

#[cfg(test)]
mod tests {
    use super::validate_passports;
    use crate::solutions::day_4::day_4::parse_passports;
    use std::fs::read_to_string;

    #[test]
    fn solve_day_four_one() {
        let test_input_path = format!("{}/src/solutions/day_4/day_4_input.txt", env!("CARGO_MANIFEST_DIR"));
        let input = read_to_string(test_input_path).unwrap();

        println!("Passports with required fields: {}", parse_passports(&input).len())
    }
    
    #[test]
    fn solve_day_four_two() {
        let test_input_path = format!("{}/src/solutions/day_4/day_4_input.txt", env!("CARGO_MANIFEST_DIR"));
        let input = read_to_string(test_input_path).unwrap();
        
        println!("Passports with required and valid fields: {}", validate_passports(&input))
    }
}