use std::fs;

// The tables will be a 2D vector holding optional unsigned integers.
// They're optional since they'll be set to None if they get marked.
type Table = Vec<Vec<Option<u32>>>;

#[derive(PartialEq)]
enum Order {
    First,
    Last,
}

struct BingoGroup {
    bingo_idxs: Vec<usize>,
    moves: Vec<u32>,
    tables: Vec<Table>,
}

impl BingoGroup {
    // Reads data from file into the table group.
    fn new(path: &str) -> BingoGroup {
        let binding = fs::read_to_string(path).unwrap();

        let mut data = binding
            .lines() // Splitting file into a vector of lines.
            .map(String::from); // Converting the lines from &str to String

        let moves = data
            .next() // Moves are defined in the first line.
            .unwrap()
            .split(',') // The moves are comma seperated.
            .map(|num| num.parse().unwrap()) // Converting each number to u32.
            .collect();

        let mut table_size = 0;

        let tables: Vec<Table> = data
            .filter(|line| !line.is_empty()) // Remove all empty lines.
            .map(|line| {
                // For each line, split into a vector and convert each number to u32.
                let split_row: Vec<Option<u32>> = line
                    .split_whitespace()
                    .map(|num| Some(num.parse().unwrap()))
                    .collect();

                // Set the table size to the length of the row since we know it'll be square.
                table_size = split_row.len();

                split_row
            })
            .collect::<Table>() // Collects into one big table.
            .chunks(table_size) // Splits big table into many smaller tables of a constant size.
            .map(<[Vec<Option<u32>>]>::to_vec) // Converts each chunk into a vector.
            .collect();

        BingoGroup {
            bingo_idxs: Vec::with_capacity(tables.len()),
            moves,
            tables,
        }
    }

    fn mark_tables(&mut self, move_idx: usize) {
        // Flattening the tables to iterate through each value easily.
        let flattened_tables = self.tables.iter_mut().flatten().flatten();

        // Values will be marked if it matches with the current move passed in.
        for num in flattened_tables {
            // Only consider values not already marked (is_some)
            if num.is_some() && num.unwrap() == self.moves[move_idx] {
                *num = None;
            }
        }
    }

    // Checking for a bingo only involves rows and columns.
    fn check_bingo(&mut self, order: &Order) -> Option<usize> {
        // Enumerating through each of the tables.
        for (i, table) in self.tables.iter().enumerate() {
            // Enumerating through each of the rows of the table.
            for (j, row) in table.iter().enumerate() {
                // Row and column iterators.
                let mut row = row.iter();
                let mut col = table.iter().map(|row| &row[j]);

                // Checks if the current row or column are all marked
                if row.all(Option::is_none) || col.all(Option::is_none) {
                    // Returns the index immediately if we want the first winning table.
                    if order == &Order::First {
                        return Some(i);
                    }

                    // Or if we want the last winning table...
                    if order == &Order::Last {
                        // Add the current table index the the winning table list if it isn't in.
                        if !self.bingo_idxs.contains(&i) {
                            self.bingo_idxs.push(i);
                        }

                        // Then return the table index if every table has now won.
                        if self.bingo_idxs.len() == self.tables.len() {
                            return Some(i);
                        }
                    }
                }
            }
        }

        // Will return None if no bingos are found.
        None
    }

    fn get_score(&self, move_idx: usize, table_idx: usize) -> u32 {
        // Gets the sum of all unmarked numbers of the winning table.
        let sum_unmarked: u32 = self.tables[table_idx].iter().flatten().flatten().sum();

        // Returns the sum multiplied by the winning move.
        sum_unmarked * self.moves[move_idx]
    }
}

// Runs through the bingo game and returns the score if a bingo was reached.
fn bingo(bingo_group: &mut BingoGroup, order: &Order) -> Option<u32> {
    let num_moves = bingo_group.moves.len();

    // Goes through each move in the move list.
    for move_idx in 0..num_moves {
        // For each move, mark the tables.
        bingo_group.mark_tables(move_idx);

        // Check to see if a bingo has been achieved depending on order definition.
        if let Some(table_idx) = bingo_group.check_bingo(order) {
            return Some(bingo_group.get_score(move_idx, table_idx));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test utility function.
    fn bingo_test(path: &str, order: &Order, expected: u32) {
        let mut bingo_group = BingoGroup::new(path);
        let result = bingo(&mut bingo_group, order);

        match result {
            Some(score) => assert_eq!(score, expected),
            None => println!("No bingo was reached..."),
        }
    }

    #[test]
    fn part1() {
        bingo_test("src/year2021/day4/bingo-ex.txt", &Order::First, 4512);
        bingo_test("src/year2021/day4/bingo.txt", &Order::First, 11536);
    }

    #[test]
    fn part2() {
        bingo_test("src/year2021/day4/bingo-ex.txt", &Order::Last, 1924);
        bingo_test("src/year2021/day4/bingo.txt", &Order::Last, 1284);
    }
}
