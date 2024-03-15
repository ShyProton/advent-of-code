mod systems;

use std::fs;

fn preprocess_data(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect()
}

// Finds the most and least common bits in a column of a list of bistrings,
// returns the most common bit as the first element, and the least common as the second.
fn common_in_col(bitstrings: &[String], col: usize) -> [char; 2] {
    // Counts the number of ones.
    let mut one_count = 0;
    // Converting the length of a bitstring to u32 to compare with one_count
    let bitstring_len: u32 = bitstrings.len().try_into().unwrap();

    // Iterates through the rows of the bitstrings.
    for bitstring in bitstrings {
        one_count += bitstring.chars().nth(col).unwrap().to_digit(2).unwrap();
    }

    // 1 is the most (or equally) common if it appears in at least half the rows of the current column.
    // Ceiling division is perfomed to avoid errors while the length is odd.
    if one_count >= (bitstring_len + 2 - 1) / 2 {
        ['1', '0']
    } else {
        ['0', '1']
    }
}

fn power_consumption(bitstrings: &[String]) -> i32 {
    // Most common bit for all bitstrings.
    let mut gamma = String::new();

    // Least common bit for all bitstrings
    let mut epsilon = String::new();

    // Iterates through the columns of the bitstrings.
    for i in 0..bitstrings[0].len() {
        let commonalities = common_in_col(bitstrings, i);
        gamma.push(commonalities[0]);
        epsilon.push(commonalities[1]);
    }

    let gamma_decimal = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_decimal = i32::from_str_radix(&epsilon, 2).unwrap();

    gamma_decimal * epsilon_decimal
}

// Gets the rating for a certain specified system in a recursive fashion.
fn system_rating(bitstrings: &[String], col: usize, system_type: systems::SystemType) -> i32 {
    // Base case: If there's only one bitstring left or we've reached the end of the columns,
    // return the first value in the list.
    if bitstrings.len() == 1 || col == bitstrings[0].len() {
        i32::from_str_radix(bitstrings[0].as_str(), 2).unwrap()
    } else {
        // Aqcuires the digit we want for the column we want, for the system we want.
        let wanted_digit = common_in_col(bitstrings, col)[systems::system_idx(&system_type)];

        // Filters the bitstring list to only include digits in the same column that
        // match the most/least common digit
        let filtered: Vec<String> = bitstrings
            .iter()
            .filter(|bitstring| bitstring.chars().nth(col).unwrap() == wanted_digit)
            .cloned()
            .collect();

        // Recursive call of this function, this time for the next column
        system_rating(&filtered, col + 1, system_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut data = preprocess_data("src/year2021/day3/bits-ex.txt");
        assert_eq!(power_consumption(&data), 198);

        data = preprocess_data("src/year2021/day3/bits.txt");
        assert_eq!(power_consumption(&data), 3912944);
    }

    #[test]
    fn part2() {
        let mut data = preprocess_data("src/year2021/day3/bits-ex.txt");
        assert_eq!(
            system_rating(&data, 0, systems::SystemType::Oxygen)
                * system_rating(&data, 0, systems::SystemType::CO2),
            230
        );

        data = preprocess_data("src/year2021/day3/bits.txt");
        assert_eq!(
            system_rating(&data, 0, systems::SystemType::Oxygen)
                * system_rating(&data, 0, systems::SystemType::CO2),
            4996233
        );
    }
}
