fn something() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        something();
    }

    #[test]
    fn part2() {
        something();
    }
}
