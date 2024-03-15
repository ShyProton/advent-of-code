use std::fs;

fn preprocess_data(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect()
}

fn horiz_times_depth(instructions: &[String]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for instruction in instructions {
        // Splits each of the instructions into the direction and amount parts
        let (direction, amount_str) = instruction.split_once(' ').unwrap();
        let amount: i32 = amount_str.parse().unwrap();

        // Each direction has a unique operation results
        match direction {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            _ => println!("Invalid direction encountered"),
        }
    }

    horizontal * depth
}

fn horiz_times_depth_with_aim(instructions: &[String]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in instructions {
        // Splits each of the instructions into the direction and amount parts
        let (direction, amount_str) = instruction.split_once(' ').unwrap();
        let amount: i32 = amount_str.parse().unwrap();

        // Each direction has a unique operation results
        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => println!("Invalid direction encountered"),
        }
    }

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut data = preprocess_data("src/year2021/day2/instructions-ex.txt");
        assert_eq!(horiz_times_depth(&data), 150);

        data = preprocess_data("src/year2021/day2/instructions.txt");
        assert_eq!(horiz_times_depth(&data), 1488669);
    }

    #[test]
    fn part2() {
        let mut data = preprocess_data("src/year2021/day2/instructions-ex.txt");
        assert_eq!(horiz_times_depth_with_aim(&data), 900);

        data = preprocess_data("src/year2021/day2/instructions.txt");
        assert_eq!(horiz_times_depth_with_aim(&data), 1176514794);
    }
}
