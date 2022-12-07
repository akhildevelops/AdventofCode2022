use crate::{Day, FromFile, FromStr};
use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct Day6 {
    stack: Vec<Vec<char>>,
    moves: Vec<(u32, u32, u32)>,
}

// From https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn stack_parser(x: &str) -> Vec<Vec<char>> {
    let mut lines = x.lines();
    lines.next_back();
    let lines = lines
        .map(|x| {
            let n = x.len();
            (0..n)
                .step_by(4)
                .map(|y| {
                    if &x[y..y + 3] != "   " {
                        Some(*&x.chars().nth(y + 1).unwrap())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    let lines = transpose(lines);
    lines
        .iter()
        .map(|x| {
            x.iter()
                .filter(|y| y.map_or(false, |_| true))
                .map(|y| y.unwrap())
                .collect()
        })
        .collect()
}

fn move_parser(x: &str) -> Vec<(u32, u32, u32)> {
    x.lines()
        .map(|x| {
            let mut moves = x
                .split(" ")
                .map(|x| x.parse::<u32>().ok())
                .filter(|x| x.map_or(false, |_| true));
            (
                moves.next().unwrap().unwrap(),
                moves.next().unwrap().unwrap(),
                moves.next().unwrap().unwrap(),
            )
        })
        .collect()
}

impl FromStr for Day6 {
    fn from_input(contents: String) -> Self {
        let mut s_m = contents.split("\n\n");
        let stack = s_m.next().unwrap();
        let moves = s_m.next().unwrap();

        Self {
            stack: stack_parser(stack),
            moves: move_parser(moves),
        }
    }
}

impl FromFile for Day6 {}
impl Day<String, String> for Day6 {
    fn part1(&mut self) -> String {
        let mut s = self.clone();
        for &move_crate in s.moves.iter() {
            let from_c = s.stack.get_mut(move_crate.1 as usize - 1).unwrap();
            let movable_crates = from_c[..move_crate.0 as usize].to_vec();
            from_c.drain(..move_crate.0 as usize);
            let to_c = s.stack.get_mut(move_crate.2 as usize - 1).unwrap();

            for each_crate in movable_crates.iter() {
                to_c.insert(0, *each_crate)
            }
        }
        s.stack
            .iter()
            .map(|x| x[0].to_string())
            .reduce(|x, y| x + &y)
            .unwrap()
    }
    fn part2(&mut self) -> String {
        for &move_crate in self.moves.iter() {
            let from_c = self.stack.get_mut(move_crate.1 as usize - 1).unwrap();
            let movable_crates = from_c[..move_crate.0 as usize].to_vec();
            from_c.drain(..move_crate.0 as usize);
            let to_c = self.stack.get_mut(move_crate.2 as usize - 1).unwrap();

            for each_crate in movable_crates.iter().rev() {
                to_c.insert(0, *each_crate)
            }
        }
        self.stack
            .iter()
            .map(|x| x[0].to_string())
            .reduce(|x, y| x + &y)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_moves_stack() {
        Day6::from_input(input.to_string());
    }
    #[test]
    fn test_part1() {
        assert_eq!(Day6::from_input(input.to_string()).part1(), "CMZ")
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day6::from_input(input.to_string()).part2(), "MCD")
    }
}
