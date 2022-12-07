use crate::{Day, FromFile, FromStr};
use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct Day6 {
    stream: Vec<char>,
}

impl FromStr for Day6 {
    fn from_input(contents: String) -> Self {
        Self {
            stream: contents.chars().collect(),
        }
    }
}

impl Day6 {
    fn get_start_marker(&self, length: usize) -> u32 {
        let n = self.stream.len();
        let mut n_chars: usize = 0;
        let mut first_flag: bool = false;
        for index in 0..=n - length {
            let mut chars_4 = self.stream[index..index + length].to_vec();
            for s_i in 0..length {
                let anchor_c = chars_4.remove(s_i);
                if !chars_4.contains(&anchor_c) {
                    if s_i == length - 1 {
                        first_flag = true;
                        n_chars = index + length;
                    }
                } else {
                    break;
                }
                chars_4.insert(s_i, anchor_c);
            }
            if first_flag {
                break;
            }
        }
        n_chars as u32
    }
}

impl FromFile for Day6 {}
impl Day<u32, u32> for Day6 {
    fn part1(&mut self) -> u32 {
        self.get_start_marker(4)
    }
    fn part2(&mut self) -> u32 {
        self.get_start_marker(14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = "nppdvjthqldpwncqszvftbrmjlhg";

    #[test]
    fn test_moves_stack() {
        Day6::from_input(input.to_string());
    }
    #[test]
    fn test_part1() {
        assert_eq!(Day6::from_input(input.to_string()).part1(), 6)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day6::from_input(input.to_string()).part2(), 23)
    }
}
