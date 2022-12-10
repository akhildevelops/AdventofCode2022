use crate::{Day, FromFile, FromStr};
#[derive(Debug, Clone)]
pub struct Day8 {
    stream: Vec<Vec<u32>>,
}

impl Day8 {
    fn rows(&self) -> usize {
        self.stream.len()
    }
    fn cols(&self) -> usize {
        self.stream[0].len()
    }
}

impl FromStr for Day8 {
    fn from_input(contents: String) -> Self {
        Self {
            stream: contents
                .lines()
                .into_iter()
                .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }
}

impl FromFile for Day8 {}
impl Day<u32, u32> for Day8 {
    fn part1(&mut self) -> u32 {
        let rows = self.rows();
        let cols = self.cols();
        (1..rows - 1)
            .into_iter()
            .flat_map(|row| {
                (1..cols - 1).into_iter().map({
                    let values = &self;
                    // all the values will be owned by closure (|col|{}) after move i.e, values, row.
                    // move is created for owning row by closure otherwise there's a possibility that |col|{} might outlive row value.
                    // For more check: https://stackoverflow.com/a/70286326/19483429
                    move |col| {
                        let point = values.stream[row][col];
                        let left = values.stream[row][..col].iter().all(|x| *x < point);
                        let right = values.stream[row][col + 1..].iter().all(|x| *x < point);
                        let mut vertical = values
                            .stream
                            .iter()
                            .enumerate()
                            .map(|(index, x)| (index, x[col]));
                        let top =
                            vertical.all(|(index, x)| if index < row { x < point } else { true });
                        let down =
                            vertical.all(|(index, x)| if index > row { x < point } else { true });
                        top | down | right | left
                    }
                })
            })
            .map(|x| x as u32)
            .sum::<u32>()
            + 2 * (self.rows() as u32 + self.cols() as u32)
            - 4
    }
    fn part2(&mut self) -> u32 {
        let rows = self.rows();
        let cols = self.cols();
        (1..rows - 1)
            .into_iter()
            .flat_map(|row| {
                (1..cols - 1).into_iter().map({
                    let values = &self; // only the pointer moves into closure.
                    move |col| {
                        let point = values.stream[row][col];
                        let mut left = values.stream[row][..col]
                            .iter()
                            .rev()
                            .take_while(|x| **x < point)
                            .count();
                        let mut right = values.stream[row][col + 1..]
                            .iter()
                            .take_while(|x| **x < point)
                            .count();
                        let vertical = values
                            .stream
                            .iter()
                            .enumerate()
                            .map(|(index, x)| (index, x[col]));
                        let mut top = vertical
                            .clone()
                            .filter(|(index, _)| *index < row)
                            .rev()
                            .take_while(|(_, x)| *x < point)
                            .count();
                        let mut down = vertical
                            .filter(|(index, _)| *index > row)
                            .take_while(|(_, x)| *x < point)
                            .count();
                        if row != 1 {
                            top = *[top + 1, row].iter().min().unwrap();
                        }
                        if row != (values.rows() - 1) {
                            down = *[down + 1, values.rows() - 1 - row].iter().min().unwrap();
                        }
                        if col != 1 {
                            left = *[left + 1, col].iter().min().unwrap();
                        }
                        if col != (values.cols() - 1) {
                            right = *[right + 1, values.cols() - 1 - col].iter().min().unwrap();
                        }
                        let val = top * left * right * down;

                        val
                    }
                })
            })
            .max()
            .unwrap() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_moves_stack() {
        Day8::from_input(INPUT.to_string());
    }
    #[test]
    fn test_part1() {
        assert_eq!(Day8::from_input(INPUT.to_string()).part1(), 21)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day8::from_input(INPUT.to_string()).part2(), 8)
    }
}
