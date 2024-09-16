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
        let Some(split) = group_str.split_once(' ') else {
            panic!("Could not find space to split group '{group_str}' at!");
        };

        let amount: u32 = split.0.parse().map_or_else(
            |err| panic!("Could not parse amount '{}' to u32! ({})", split.0, err),
            |amount| amount,
        );

        let color = match split.1 {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Could not assign colour to group '{group_str}'!"),
        };

        Self { amount, color }
    }

    const fn is_possible(&self) -> bool {
        match self.color {
            Color::Red => self.amount <= 12,
            Color::Green => self.amount <= 13,
            Color::Blue => self.amount <= 14,
        }
    }
}

pub fn main() {
    println!("{}", game_id_sum(DOCUMENT_TEST));
    println!("{}", game_id_sum(DOCUMENT_REAL));
}

fn game_id_sum(game_list: &str) -> u32 {
    game_list
        .lines()
        .map(game_possible)
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn game_possible(game: &str) -> u32 {
    let (id, plays) = {
        let Some(game_split) = game.split_once(':') else {
            panic!("Could not find colon to split game '{game}' at!");
        };

        let id: u32 = {
            let Some(id_split) = game_split.0.split_once(' ') else {
                panic!(
                    "Could not find space to split game declaration '{}'!",
                    game_split.0
                );
            };

            id_split.1.parse::<u32>().map_or_else(
                |err| panic!("Could not parse game id '{}'! ({})", id_split.1, err),
                |id| id,
            )
        };

        let plays: Vec<&str> = game_split.1.split(';').collect();

        (id, plays)
    };

    for play in plays {
        for group in play.split(',') {
            if !Group::from_group_str(group.trim()).is_possible() {
                return 0;
            }
        }
    }

    id
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
