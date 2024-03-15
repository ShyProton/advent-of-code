use super::{traits::Poppable, Stack};

pub enum CraneMode {
    Mode9000,
    Mode9001,
}

impl CraneMode {
    pub fn perform_procedure(
        &self,
        stacks: &mut [Stack],
        moves: usize,
        source: usize,
        destination: usize,
    ) {
        match *self {
            Self::Mode9000 => Self::procedure_9000(stacks, moves, source, destination),
            Self::Mode9001 => Self::procedure_9001(stacks, moves, source, destination),
        }
    }

    fn append_to_stack(stacks: &mut [Stack], popped: &mut Stack, destination: usize) {
        stacks
            .get_mut(destination - 1)
            .map_or_else(
                || panic!("Could not get destination stack during move"),
                |dest| dest,
            )
            .append(popped);
    }

    fn procedure_9000(stacks: &mut [Stack], moves: usize, source: usize, destination: usize) {
        for _ in 0..moves {
            let mut popped = stacks.get_mut(source - 1).map_or_else(
                || panic!("Could not get source stack during move"),
                |stack| vec![stack.try_pop("Could not pop from stack during procedure 9000")],
            );

            Self::append_to_stack(stacks, &mut popped, destination);
        }
    }

    fn procedure_9001(stacks: &mut [Stack], moves: usize, source: usize, destination: usize) {
        let mut popped = stacks.get_mut(source - 1).map_or_else(
            || panic!("Could not get source stack during move"),
            |stack| {
                stack
                    // Gets 'moves' items from the top of the stack
                    .splice(stack.len() - moves.., std::iter::empty())
                    .collect::<Stack>()
            },
        );

        Self::append_to_stack(stacks, &mut popped, destination);
    }
}
