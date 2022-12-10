use crate::{Day, FromFile, FromStr as CrateFromStr};
pub struct Day15;

impl CrateFromStr for Day15 {
    fn from_input(_input: String) -> Self {
        Self
    }
}

impl FromFile for Day15 {}

impl Day<u32, u32> for Day15 {
    fn part1(&mut self) -> u32 {
        10
    }
    fn part2(&mut self) -> u32 {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(Day15::from_input(INPUT.to_string()).part1(), 10);
    }
    #[test]
    fn test_part2() {
        assert_eq!(Day15::from_input(INPUT.to_string()).part2(), 10);
    }
}