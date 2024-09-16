const DOCUMENT_TEST: &str = include_str!("games-ex.txt");
const DOCUMENT_REAL: &str = include_str!("games.txt");

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

fn game_possible(game: &str) -> Result<(), String> {
    let (_, plays) = attempt_split_once(game, ':');

    plays
        .split(';')
        .flat_map(|play| play.split(','))
        .try_for_each(|group| Group::from_group_str(group.trim()).is_possible())
}

pub fn main() {
    println!("{}", game_id_sum(DOCUMENT_TEST));
    println!("{}", game_id_sum(DOCUMENT_REAL));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(game_id_sum(DOCUMENT_TEST), 8);
        assert_eq!(game_id_sum(DOCUMENT_REAL), 2162);
    }

    // #[test]
    // fn part2() {
    //     assert_eq!(calval_sum(DOCUMENT_TEST_2), 281);
    //     assert_eq!(calval_sum(DOCUMENT_REAL), 54_249);
    // }
}
