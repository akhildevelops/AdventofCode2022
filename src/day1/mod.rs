use crate::{Day, FromFile};
use std::fs;
pub struct Day1 {
    food: Vec<Vec<u32>>,
}

impl Day1 {
    fn elvis_calories(&self) -> impl Iterator<Item = u32> + '_ {
        self.food.iter().map(|x| x.iter().sum())
    }
}
impl FromFile for Day1 {
    fn from_input_file(path: &str) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        Self {
            food: contents
                .split("\n\n")
                .map(|x| x.split("\n").map(|x| x.parse().unwrap_or(0)).collect())
                .collect(),
        }
    }
}
impl Day<u32, u32> for Day1 {
    fn part1(&self) -> u32 {
        self.elvis_calories().max().unwrap()
    }
    fn part2(&self) -> u32 {
        let mut sort_calories = self.elvis_calories().collect::<Vec<u32>>();
        sort_calories.sort();
        sort_calories.reverse();
        sort_calories.iter().take(3).sum()
    }
}
