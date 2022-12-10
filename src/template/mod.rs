use crate::{Day, FromFile, FromStr as CrateFromStr};
pub struct Template;

impl CrateFromStr for Template {
    fn from_input(_input: String) -> Self {
        Self
    }
}

impl FromFile for Template {}

impl Day<u32, u32> for Template {
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
        assert_eq!(Template::from_input(INPUT.to_string()).part1(), 10);
    }
    #[test]
    fn test_part2() {
        assert_eq!(Template::from_input(INPUT.to_string()).part2(), 10);
    }
}
