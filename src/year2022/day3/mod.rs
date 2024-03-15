use std::{cmp::Ordering, convert::Into};

// Holds the contents of each of the inputs files.
const RUCKSACKS_TEST: &str = include_str!("rucksack-ex.txt");
const RUCKSACKS_REAL: &str = include_str!("rucksack.txt");

// Ascii normalization constants used to calculate priority for rucksack items.
const ASCII_LOWERCASE_NORMAL: Priority = b"a"[0] - 1;
const ASCII_UPPERCASE_NORMAL: Priority = b"A"[0] - (b"z"[0] - ASCII_LOWERCASE_NORMAL) - 1;

type Priority = u8;

pub fn main() {
    println!("{}", rucksacks_sum(RUCKSACKS_TEST));
    println!("{}", rucksacks_sum(RUCKSACKS_REAL));
}

fn rucksacks_sum(file_contents: &str) -> u32 {
    // For each line (rucksack) determine its priority, then take the sum.
    file_contents.lines().map(rucksack_priority).sum()
}

fn rucksack_priority(rucksack: &str) -> u32 {
    let mut priorities = rucksack
        .chars() // Splitting rucksack into individual items (characters).
        .map(|item| {
            // Attempting to convert each item to its corresponding priority number.
            item_to_priority(item).map_or_else(
                |err| panic!("Could not map item to priority: {err}"),
                |item| item,
            )
        })
        .collect::<Vec<Priority>>();

    // Splits the rucksack into the two compartments.
    let compartments = priorities.split_at_mut(rucksack.len() / 2);

    // Sorts each compartment.
    compartments.0.sort_unstable();
    compartments.1.sort_unstable();

    // Attempts to find a duplicate between the two compartments.
    find_duplicate(compartments.0, compartments.1).map_or(0, Into::into)
}

fn item_to_priority(item: char) -> Result<Priority, String> {
    // Determines the normal to use based on whether the item is upper or lower case.
    let normal = if item.is_uppercase() {
        ASCII_UPPERCASE_NORMAL
    } else if item.is_lowercase() {
        ASCII_LOWERCASE_NORMAL
    } else {
        // If the item is neither, it's invalid. So we throw an InvalidCharacterError.
        return Err(format!(
            "Character '{item}' is not an upper or lowercase letter."
        ));
    };

    // Returns the normalized item.
    Ok((item as Priority) - normal)
}

fn find_duplicate<T>(arr_1: &[T], arr_2: &[T]) -> Option<T>
where
    T: Clone + Ord,
{
    let mut i = 0; // Index for arr_1.
    let mut j = 0; // Index for arr_2.

    // Looping until one of the indices reaches the end of their array.
    while i < arr_1.len() && j < arr_2.len() {
        match arr_1[i].cmp(&arr_2[j]) {
            Ordering::Less => i += 1,
            Ordering::Equal => return Some(arr_1[i].clone()), // Found a duplicate.
            Ordering::Greater => j += 1,
        };
    }

    // No duplicate found.
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(157, rucksacks_sum(RUCKSACKS_TEST));
        assert_eq!(8_240, rucksacks_sum(RUCKSACKS_REAL));
    }

    // #[test]
    // fn part2() {
    // }
}
