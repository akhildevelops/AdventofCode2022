use crate::{Day, FromFile};
use std::fs;
pub struct Day1 {
    food: Vec<u32>,
}

impl Day1 {
    fn elvis_top_3_calories(&mut self) -> &[u32] {
        self.food.sort();
        &self.food[self.food.len() - 3..]
    }
}
impl FromFile for Day1 {
    fn from_input_file(path: &str) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        Self {
            food: contents
                .split("\n\n")
                .map(|x| x.split("\n").map(|x| x.parse().unwrap_or(0)).sum())
                .collect(),
        }
    }
}
impl Day<u32, u32> for Day1 {
    fn part1(&mut self) -> u32 {
        self.elvis_top_3_calories()[2]
    }
    fn part2(&mut self) -> u32 {
        self.elvis_top_3_calories().iter().sum()
    }
}
