mod modes;
mod traits;

use modes::CraneMode;
use std::vec;
use traits::Poppable;

// Holds the contents of each of the inputs files.
const INSTRUCTIONS_TEST: &str = include_str!("instructions-ex.txt");
const INSTRUCTIONS_REAL: &str = include_str!("instructions.txt");

const ITEM_CHAR_SPACING: usize = 4;

type Instruction = usize;
type Stack = Vec<char>;
type Procedure = (Instruction, Instruction, Instruction);

pub fn main() {
    let mut operator = CraneOperator::new();
    let mut mode = CraneMode::Mode9000;

    operator.load_instructions(INSTRUCTIONS_TEST);
    operator.rearrange_stacks(&mode);
    println!("{}", operator.pop_top_string());

    mode = CraneMode::Mode9001;

    operator.load_instructions(INSTRUCTIONS_REAL);
    operator.rearrange_stacks(&mode);
    println!("{}", operator.pop_top_string());
}

/// `CraneOperator` struct.
/// * `stacks` - Vector of stacks that will be manipulated according to the procuedures.
/// * `procedures` - Vector of procedures composing of Instructions to be carried out.
struct CraneOperator {
    stacks: Vec<Stack>,
    procedures: Vec<Procedure>,
}

impl CraneOperator {
    /// Creates a new `CraneOperator` with empty vectors as default for `stacks` and `procedures`.
    pub const fn new() -> Self {
        Self {
            stacks: vec![],
            procedures: vec![],
        }
    }

    /// Loads instructions from `file_contents` into `stacks` and `procedures accordingly`.
    fn load_instructions(&mut self, file_contents: &str) {
        // Seperates the two sections by finding an empty line as a divider.
        let (items_section, procedures_section) = file_contents.split_once("\n\n").map_or_else(
            || panic!("Could not split the contents into two segments: No empty line found."),
            |sections| sections,
        );

        self.stacks = vec![Stack::new(); Self::get_stack_count(items_section)];
        self.procedures = vec![];

        Self::load_items(items_section, &mut self.stacks);
        Self::load_procedures(procedures_section, &mut self.procedures);
    }

    /// Gets the appropriate amount of stacks to hold the items in `items_section`.
    fn get_stack_count(items_section: &str) -> usize {
        (items_section
            .lines()
            .next() // First line of the items section
            .map_or_else(
                || panic!("Could not read contents of the 'items' segment"),
                |line| line,
            )
            .len()
            + 1)
            / ITEM_CHAR_SPACING // Dividing by the space between the items yields the count.
    }

    /// Loads items from `items_section` into `stacks`.
    fn load_items(items_section: &str, stacks: &mut [Stack]) {
        let mut push_item = |i: usize, item: char| {
            stacks
                .get_mut(i)
                .map_or_else(
                    || panic!("Attempted to index out of bounds item when loading items"),
                    |stack| stack,
                )
                .push(item);
        };

        for line in items_section.lines() {
            for (i, item) in line.chars().skip(1).step_by(ITEM_CHAR_SPACING).enumerate() {
                if item.is_uppercase() {
                    push_item(i, item);
                }
            }
        }

        // After adding all the items, reverse each of the stacks
        // so the items added first are 'on top' of the stack.
        for stack in stacks {
            stack.reverse();
        }
    }

    /// Loads procedures from `procedures_section` into `procedures`.
    fn load_procedures(procedures_section: &str, procedures: &mut Vec<Procedure>) {
        let parse_instruction = |instruction: &str| {
            instruction.parse::<Instruction>().map_or_else(
                |err| panic!("Could not convert string {instruction} to instruction: {err}"),
                |converted| converted,
            )
        };

        let preprocess_line = |line: &str| -> Procedure {
            match line
                .split(' ')
                .skip(1) // Skips first word.
                .step_by(2) // Skips other words.
                .map(parse_instruction)
                .collect::<Vec<_>>()[..]
            {
                [moves, source, destination] => (moves, source, destination),
                _ => panic!("Procedure must have exactly three instructions."),
            }
        };

        for line in procedures_section.lines() {
            procedures.push(preprocess_line(line));
        }
    }

    fn rearrange_stacks(&mut self, mode: &CraneMode) {
        for (moves, source, destination) in &self.procedures {
            mode.perform_procedure(&mut self.stacks, *moves, *source, *destination);
        }
    }

    fn pop_top_string(&mut self) -> String {
        self.stacks
            .iter_mut()
            .map(|stack| stack.try_pop("Could not pop from stack after procedures."))
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut operator = CraneOperator::new();
        let mode = CraneMode::Mode9000;

        operator.load_instructions(INSTRUCTIONS_TEST);
        operator.rearrange_stacks(&mode);
        assert_eq!("CMZ", operator.pop_top_string());

        operator.load_instructions(INSTRUCTIONS_REAL);
        operator.rearrange_stacks(&mode);
        assert_eq!("MQTPGLLDN", operator.pop_top_string());
    }

    #[test]
    fn part2() {
        let mut operator = CraneOperator::new();
        let mode = CraneMode::Mode9001;

        operator.load_instructions(INSTRUCTIONS_TEST);
        operator.rearrange_stacks(&mode);
        assert_eq!("MCD", operator.pop_top_string());

        operator.load_instructions(INSTRUCTIONS_REAL);
        operator.rearrange_stacks(&mode);
        assert_eq!("LVZPSTTCZ", operator.pop_top_string());
    }
}
