use crate::{Day, FromFile, FromStr};
use std::collections::hash_map::RandomState;
use std::collections::hash_set::Intersection;
use std::collections::HashSet;
pub struct Day3 {
    rucksacks: Vec<String>,
}

fn score_from_sets(sets: Intersection<char, RandomState>) -> u32 {
    sets.map(|x| {
        if x.is_uppercase() {
            27 + *x as u32 - 65
        } else {
            1 + *x as u32 - 97
        }
    })
    .sum::<u32>()
}

impl FromStr for Day3 {
    fn from_input(input: String) -> Self {
        Self {
            rucksacks: input
                .strip_suffix("\n")
                .unwrap()
                .split("\n")
                .map(|x| x.to_string())
                .collect(),
        }
    }
}

impl FromFile for Day3 {}
impl Day<u32, u32> for Day3 {
    fn part1(&mut self) -> u32 {
        self.rucksacks
            .iter()
            .map(|x| {
                let n = x.len();
                let (rack_1, rack_2) = x.split_at(n / 2);
                let rack1_itemtypes: HashSet<char> = HashSet::from_iter(rack_1.chars());
                let rack2_itemtypes: HashSet<char> = HashSet::from_iter(rack_2.chars());
                let common_types = rack1_itemtypes.intersection(&rack2_itemtypes);
                score_from_sets(common_types)
            })
            .sum()
    }
    fn part2(&mut self) -> u32 {
        self.rucksacks
            .chunks(3)
            .map(|x| {
                let rack1_itemtypes: HashSet<char> = HashSet::from_iter(x[0].chars());
                let rack2_itemtypes: HashSet<char> = HashSet::from_iter(x[1].chars());
                let rack3_itemtypes: HashSet<char> = HashSet::from_iter(x[2].chars());
                let r1_r2: HashSet<char> = rack1_itemtypes
                    .intersection(&rack2_itemtypes)
                    .copied()
                    .collect();
                let r1_r2_r3 = r1_r2.intersection(&rack3_itemtypes);
                score_from_sets(r1_r2_r3)
            })
            .sum()
    }
}
