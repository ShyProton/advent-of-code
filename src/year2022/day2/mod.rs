const STRATEGY_TEST: &str = include_str!("strategy-ex.txt");
const STRATEGY_REAL: &str = include_str!("strategy.txt");

const ASCII_NORMAL: u8 = b"A"[0];
const PLAYER_ASCII_DIFF: u8 = b"X"[0] - b"A"[0];

type Score = u32;

fn rock_paper_scissors(file_contents: &str, decrypt: bool) -> Score {
    // Takes each line of the strategy, computes the score, and takes the sum.
    file_contents
        .lines()
        .map(|line| play_round(line, decrypt))
        .sum()
}

// Takes a line and plays a single round of RPS with it.
fn play_round(play: &str, decrypt: bool) -> Score {
    // Splits the line into the opponent and player moves.
    match play.split_whitespace().collect::<Vec<_>>()[..] {
        [opponent, player] => preprocess_plays(opponent, player),
        _ => panic!("The line must have exactly two characters seperated by whitespace"),
    }
    .into_iter()
    // Determines the winner given the two plays.
    .reduce(|opponent, player| determine_winner(i16::from(opponent), i16::from(player), decrypt))
    .map_or_else(|| panic!("Empty line encountered"), |score| score)
    .into()
}

// Takes the individual characters of a play as strings,
// then preprocesses them to be normalized ascii values.
fn preprocess_plays(opponent: &str, player: &str) -> [u8; 2] {
    [
        // 'A' maps to 0, 'B' maps to 1, 'C' maps to 2
        (opponent.as_bytes().first().map_or_else(
            || panic!("Could find the opponent character"),
            |ascii| ascii,
        ) - ASCII_NORMAL),
        // 'X' maps to 0, 'Y' maps to 1, 'Z' maps to 2
        (player.as_bytes().first().map_or_else(
            || panic!("Could not find the player character"),
            |ascii| ascii,
        ) - (ASCII_NORMAL + PLAYER_ASCII_DIFF)),
    ]
}

// Takes the opponent and player's character ascii values, determines the winner,
// and returns the appropriate score.
// The ascii values are converted to integers, since the solution involves working with
// negative numbers, and we don't want to overflow.
fn determine_winner(opponent: i16, player: i16, decrypt: bool) -> u8 {
    let player_adj = if decrypt {
        // If decrypting, we adjust to either win, lose or tie depending on what 'player' is.
        (opponent + (player - 1)).rem_euclid(3)
    } else {
        player // If not decrypting, we assume the player corresponds to another move.
    };

    // Computes the score, by having 3 * the win status, followed by adding the player's move.
    (3 * (player_adj - opponent + 1).rem_euclid(3) + (player_adj + 1))
        .try_into()
        .map_or_else(
            |err| panic!("Could not convert this line's score to a byte {err}"),
            |a| a,
        )
}

pub fn main() {
    println!("{}", rock_paper_scissors(STRATEGY_TEST, true));
    println!("{}", rock_paper_scissors(STRATEGY_REAL, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(15, rock_paper_scissors(STRATEGY_TEST, false));
        assert_eq!(11_386, rock_paper_scissors(STRATEGY_REAL, false));
    }

    #[test]
    fn part2() {
        assert_eq!(12, rock_paper_scissors(STRATEGY_TEST, true));
        assert_eq!(13_600, rock_paper_scissors(STRATEGY_REAL, true));
    }
}
