use std::fs;

fn preprocess_data(path: &str) -> Vec<i32> {
    // Reads lines of depths as a vector of integers
    fs::read_to_string(path)
        .expect("Was not able to read the file.")
        .lines()
        .map(|depth| depth.parse().unwrap())
        .collect()
}

fn compare_depths(depths: &[i32]) -> i32 {
    let mut increased_counter = 0;

    // Enumerating through depths (skipping the first value)
    for (i, depth) in depths.iter().enumerate().skip(1) {
        // Increase the count of the current depth is larger than the last
        if depth > &depths[i - 1] {
            increased_counter += 1;
        }
    }

    increased_counter
}

fn compare_windows(depths: &[i32]) -> i32 {
    let mut increased_counter = 0;
    let window_size = 3;

    // Gets the sum of a window of values in the given array.
    let get_window_sum = |depths: &[i32], i: usize| depths[i..i + window_size].iter().sum();

    // Compares sums across windows instead of individual values.
    for i in 1..depths.len() - window_size + 1 {
        let current_window: i32 = get_window_sum(depths, i);
        let prev_window: i32 = get_window_sum(depths, i - 1);

        if current_window > prev_window {
            increased_counter += 1;
        }
    }

    increased_counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut data = preprocess_data("src/year2021/day1/depths-ex.txt");
        assert_eq!(compare_depths(&data), 7);

        data = preprocess_data("src/year2021/day1/depths.txt");
        assert_eq!(compare_depths(&data), 1184);
    }

    #[test]
    fn part2() {
        let mut data = preprocess_data("src/year2021/day1/depths-ex.txt");
        assert_eq!(compare_windows(&data), 5);

        data = preprocess_data("src/year2021/day1/depths.txt");
        assert_eq!(compare_windows(&data), 1158);
    }
}
