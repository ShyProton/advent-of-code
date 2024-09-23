use std::collections::HashMap;

const DOCUMENT_TEST: &str = include_str!("games-ex.txt");
const DOCUMENT_REAL: &str = include_str!("games.txt");

#[derive(Eq, Hash, PartialEq, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Group {
    amount: u32,
    color: Color,
}

impl Group {
    fn from_group_str(group_str: &str) -> Self {
        let (amount_str, color_str) = attempt_split_once(group_str, ' ');

        let amount: u32 = amount_str.parse().map_or_else(
            |err| panic!("Could not parse amount '{amount_str}' to u32! ({err})"),
            |amount| amount,
        );

        let color = match color_str {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Could not assign colour to group '{group_str}'!"),
        };

        Self { amount, color }
    }

    fn is_possible(&self) -> Result<(), String> {
        if match self.color {
            Color::Red => self.amount <= 12,
            Color::Green => self.amount <= 13,
            Color::Blue => self.amount <= 14,
        } {
            Ok(())
        } else {
            Err("Group is not possible.".to_string())
        }
    }
}

fn id_to_u32(id: usize) -> u32 {
    u32::try_from(id).map_or_else(
        |err| panic!("Could not convert id from usize to u32 ({err})"),
        |id| id,
    )
}

fn attempt_split_once(str: &str, delim: char) -> (&str, &str) {
    let Some((a, b)) = str.split_once(delim) else {
        panic!("Could not find character '{delim}' to split string '{str}'");
    };

    (a, b)
}

fn game_id_sum(game_list: &str) -> u32 {
    game_list
        .lines()
        .map(game_possible)
        .enumerate()
        .filter(|(_, item)| item.is_ok())
        .map(|(id, _)| id_to_u32(id + 1))
        .sum()
}

fn game_power(game_list: &str) -> u32 {
    game_list
        .lines()
        .map(max_of_colors)
        .map(|max_by_color| max_by_color.values().product::<u32>())
        .sum()
}

fn game_possible(game: &str) -> Result<(), String> {
    let (_, plays) = attempt_split_once(game, ':');

    plays
        .split(';')
        .flat_map(|play| play.split(','))
        .try_for_each(|group| Group::from_group_str(group.trim()).is_possible())
}

fn max_of_colors(game: &str) -> HashMap<Color, u32> {
    let (_, plays) = attempt_split_once(game, ':');

    let mut groups_by_color: HashMap<Color, Vec<Group>> = vec![
        (Color::Red, vec![]),
        (Color::Green, vec![]),
        (Color::Blue, vec![]),
    ]
    .into_iter()
    .collect();

    let mut max_by_color: HashMap<Color, u32> = HashMap::new();

    for group in plays
        .split(';')
        .flat_map(|play| play.split(','))
        .map(|group| Group::from_group_str(group.trim()))
    {
        let Some(color_list) = groups_by_color.get_mut(&group.color) else {
            panic!("Could not find key '{:?}' in color groups.", group.color);
        };

        color_list.push(group);
    }

    for (color, groups) in groups_by_color {
        let maximum = groups
            .iter()
            .max_by_key(|g| g.amount)
            .map_or(0, |group| group.amount);

        max_by_color.insert(color, maximum);
    }

    max_by_color
}

pub fn main() {
    println!("{}", game_id_sum(DOCUMENT_TEST));
    println!("{}", game_id_sum(DOCUMENT_REAL));

    println!("{}", game_power(DOCUMENT_TEST));
    println!("{}", game_power(DOCUMENT_REAL));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(game_id_sum(DOCUMENT_TEST), 8);
        assert_eq!(game_id_sum(DOCUMENT_REAL), 2162);
    }

    #[test]
    fn part2() {
        assert_eq!(game_power(DOCUMENT_TEST), 2286);
        assert_eq!(game_power(DOCUMENT_REAL), 72_513);
    }
}
