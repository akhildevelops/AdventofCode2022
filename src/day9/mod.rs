use crate::{Day, FromFile, FromStr};

// Credits: https://github.com/schubart/AdventOfCode_2022_Rust/blob/master/day09/src/lib.rs

pub struct Day9 {
    stream: String,
}

impl FromStr for Day9 {
    fn from_input(contents: String) -> Self {
        Self { stream: contents }
    }
}

use std::collections::HashSet;

type Point = (i32, i32);

fn lead(direction: &str, head: &mut Point) {
    match direction {
        "R" => head.0 += 1,
        "L" => head.0 -= 1,
        "D" => head.1 += 1,
        "U" => head.1 -= 1,
        _ => panic!("{direction}"),
    };
}

fn follow(leader: Point, follower: &mut Point) {
    let delta = (leader.0 - follower.0, leader.1 - follower.1);
    if delta.0.abs() == 2 || delta.1.abs() == 2 {
        follower.0 += delta.0.signum();
        follower.1 += delta.1.signum();
    }
}

impl FromFile for Day9 {}
impl Day<u32, u32> for Day9 {
    fn part1(&mut self) -> u32 {
        let mut rope = vec![(0, 0); 2];
        let mut visited = HashSet::from([(0, 0)]);

        for line in self.stream.lines() {
            let mut split = line.split(' ');
            let direction = split.next().unwrap();
            let count: i32 = split.next().unwrap().parse().unwrap();

            for _ in 0..count {
                lead(direction, &mut rope[0]);

                for i in 1..rope.len() {
                    follow(rope[i - 1], &mut rope[i]);
                }

                visited.insert(*rope.last().unwrap());
            }
        }
        visited.len() as u32
    }
    fn part2(&mut self) -> u32 {
        let mut rope = vec![(0, 0); 10];
        let mut visited = HashSet::from([(0, 0)]);

        for line in self.stream.lines() {
            let mut split = line.split(' ');
            let direction = split.next().unwrap();
            let count: i32 = split.next().unwrap().parse().unwrap();

            for _ in 0..count {
                lead(direction, &mut rope[0]);

                for i in 1..rope.len() {
                    follow(rope[i - 1], &mut rope[i]);
                }

                visited.insert(*rope.last().unwrap());
            }
        }
        visited.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_moves_stack() {
        Day9::from_input(INPUT.to_string());
    }
    #[test]
    fn test_part1() {
        assert_eq!(Day9::from_input(INPUT.to_string()).part1(), 13)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day9::from_input(INPUT.to_string()).part2(), 1)
    }
}
