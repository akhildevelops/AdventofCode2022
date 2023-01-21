use crate::{Day, FromFile, FromStr as CrateFromStr};
const S: usize = 83;
const E: usize = 69;
pub struct Day12 {
    grid: Vec<Vec<usize>>,
}

impl Day12 {
    fn get_SE_coords(&self) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        for row in self.grid.iter().enumerate() {
            for col in row.1.iter().enumerate() {
                if S == *col.1 {
                    start = Some((row.0, col.0));
                }
                if E == *col.1 {
                    end = Some((row.0, col.0));
                }
            }
        }
        (start, end)
    }
    fn paths(&self, root: (usize, usize), mut path: Vec<usize>) -> Vec<Vec<usize>> {
        let mut current_level = self.grid[root.0][root.1];
        path.push(current_level);
        let mut all_paths: Vec<Vec<usize>> = vec![];
        for row in [-1i32, 0, 1] {
            for col in [-1i32, 0, 1] {
                if row.abs() == col.abs() {
                    continue;
                }
                let new_coords = ((root.0 as i32 + row), (root.1 as i32 + col));
                if new_coords.0 < 0
                    || new_coords.0 >= self.grid.len() as i32
                    || new_coords.1 < 0
                    || new_coords.1 >= self.grid[0].len() as i32
                {
                    continue;
                }
                println!("{:?}", &new_coords);
                let mut nested_paths: Vec<Vec<usize>> = vec![];
                let new_coords = (new_coords.0 as usize, new_coords.1 as usize);
                let new_level = self.grid[new_coords.0][new_coords.1];
                if current_level == E {
                    current_level = 122;
                }
                if current_level == S {
                    nested_paths = vec![path.clone()];
                } else if current_level as i32 - new_level as i32 == 1
                    || current_level as i32 - new_level as i32 == 0
                {
                    nested_paths = self.paths(new_coords, path.clone());
                }
                all_paths.extend(nested_paths);
            }
        }
        all_paths
    }
}

impl CrateFromStr for Day12 {
    fn from_input(input: String) -> Self {
        Self {
            grid: input
                .lines()
                .map(|x| x.chars().map(|x| x as usize).collect())
                .collect(),
        }
    }
}

impl FromFile for Day12 {}

impl Day<usize, usize> for Day12 {
    fn part1(&mut self) -> usize {
        let (start, end) = self.get_SE_coords();

        let (start, end) = (start.unwrap(), end.unwrap());

        self.grid[end.0][end.1]
    }
    fn part2(&mut self) -> usize {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_path() {
        let grid = Day12::from_input(INPUT.to_string());
        let (_, end) = grid.get_SE_coords();
        // println!("{}", 34);
        println!("{:?}", grid.paths(end.unwrap(), vec![]));
        assert_eq!(23, 80);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day12::from_input(INPUT.to_string()).part1(), 10);
    }
    #[test]
    fn test_part2() {
        assert_eq!(Day12::from_input(INPUT.to_string()).part2(), 10);
    }
}
