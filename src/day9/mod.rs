use crate::{Day, FromFile, FromStr};

pub struct Day9 {
    stream: Vec<Dirs>,
}
#[derive(Debug)]
enum Dirs {
    H(i32),
    V(i32),
}

impl FromStr for Dirs {
    fn from_input(input: String) -> Self {
        let mut dir_steps = input.split(' ');
        let dir = dir_steps.next().unwrap();
        let steps = dir_steps.next().unwrap().parse().unwrap();
        match dir {
            "R" => Self::H(steps),
            "L" => Self::H(-1 * steps),
            "U" => Self::V(steps),
            "D" => Self::V(-1 * steps),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Day9 {
    fn from_input(contents: String) -> Self {
        Self {
            stream: contents
                .lines()
                .into_iter()
                .map(|x| Dirs::from_input(x.to_string()))
                .collect(),
        }
    }
}

impl FromFile for Day9 {}
impl Day<u32, u32> for Day9 {
    fn part1(&mut self) -> u32 {
        self.stream
            .iter()
            .fold(((0i32, 0i32), 0), |x, y| {
                let z = match y {
                    Dirs::H(z) => {
                        if z.abs() - x.0 .0.abs() <= 1 {
                            ((z + x.0 .0, x.0 .1), x.1)
                        } else {
                            let dir = z / z.abs();
                            ((dir, 0), z.abs() - x.0 .0.abs() - 1 + x.1)
                        }
                    }
                    Dirs::V(z) => {
                        if z.abs() - x.0 .1.abs() <= 1 {
                            ((x.0 .0, z + x.0 .1), x.1)
                        } else {
                            let dir = z / z.abs();
                            ((0, dir), x.1 + z.abs() - x.0 .1.abs() - 1)
                        }
                    }
                };
                z
            })
            .1 as u32
    }
    fn part2(&mut self) -> u32 {
        10
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
        assert_eq!(Day9::from_input(INPUT.to_string()).part1(), 12)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day9::from_input(INPUT.to_string()).part2(), 8)
    }
}
