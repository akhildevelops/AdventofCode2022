use crate::{Day, FromFile};
use std::fs;
pub struct Day2 {
    strategy: Vec<(u32, u32)>,
}

impl Day2 {
    fn states_score(&self) -> u32 {
        self.strategy
            .iter()
            .map(|x| match x.1 as i32 - x.0 as i32 {
                1 | -2 => 6 + x.1 + 1,
                0 => 3 + x.1 + 1,
                _ => x.1 + 1,
            })
            .sum()
    }

    fn wins_score(&self) -> u32 {
        self.strategy
            .iter()
            .map(|x| match (x.0 as i32, x.1) {
                (y, 0) => {
                    if y - 1 < 0 {
                        3
                    } else {
                        y
                    }
                }
                (y, 1) => y + 1 + 3,
                (y, _) => (y + 1) % 3 + 1 + 6,
            })
            .sum::<i32>() as u32
    }
}
impl FromFile for Day2 {
    fn from_input_file(path: &str) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        Day2 {
            strategy: contents
                .strip_suffix("\n")
                .unwrap()
                .split("\n")
                .map(|x| {
                    let mut states = x.chars();
                    let opp_state = states.next().unwrap() as u32 - 65;
                    states.next();
                    let my_state = states.next().unwrap() as u32 - 88;
                    (opp_state, my_state)
                })
                .collect(),
        }
    }
}
impl Day<u32, u32> for Day2 {
    fn part1(&mut self) -> u32 {
        self.states_score()
    }
    fn part2(&mut self) -> u32 {
        self.wins_score()
    }
}
