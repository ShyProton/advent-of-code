const DOCUMENT_TEST_1: &str = include_str!("calibration-ex1.txt");
const DOCUMENT_TEST_2: &str = include_str!("calibration-ex2.txt");
const DOCUMENT_REAL: &str = include_str!("calibration.txt");

const DIGIT_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn main() {
    println!("{}", calval_sum(DOCUMENT_TEST_1));
    println!("{}", calval_sum(DOCUMENT_TEST_2));
    println!("{}", calval_sum(DOCUMENT_REAL));
}

fn replace_words(line: &str) -> String {
    let mut replaced_line = line.to_string();

    // (Found Index, Digit to fill)
    let mut found_words: Vec<(usize, usize)> = Vec::new();

    for (word_idx, word) in DIGIT_WORDS.iter().enumerate() {
        let finds = replaced_line.match_indices(word);

        for (find_idx, _) in finds {
            found_words.push((find_idx, word_idx + 1));
        }
    }

    found_words.sort_by(|a, b| a.0.cmp(&b.0));

    for (i, (find_idx, digit)) in found_words.iter().enumerate() {
        replaced_line.insert_str(find_idx + i, digit.to_string().as_str());
    }

    replaced_line
}

fn line_to_num(line: &str) -> u32 {
    let nums = line
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();

    let first_digit = nums.chars().next().unwrap_or('0');
    let last_digit = nums.chars().next_back().unwrap_or('0');

    format!("{first_digit}{last_digit}").parse().unwrap_or(0)
}

fn calval_sum(file_contents: &str) -> u32 {
    file_contents
        .lines()
        .map(replace_words)
        .map(|line| line_to_num(line.as_str()))
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(calval_sum(DOCUMENT_TEST_1), 142);
        // assert_eq!(calval_sum(DOCUMENT_REAL), 53_194);
    }

    #[test]
    fn part2() {
        assert_eq!(calval_sum(DOCUMENT_TEST_2), 281);
        assert_eq!(calval_sum(DOCUMENT_REAL), 54_249);
    }
}
