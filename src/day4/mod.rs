use crate::{Day, FromFile, FromStr};
pub struct Day4 {
    sections: Vec<(i32, i32, i32, i32)>,
}

impl FromStr for Day4 {
    fn from_input(contents: String) -> Self {
        Day4 {
            sections: contents
                .strip_suffix('\n')
                .unwrap_or(&contents)
                .split('\n')
                .map(|x| {
                    let sections = x
                        .split(',')
                        .flat_map(|x| x.split('-').map(|x| x.parse::<i32>().unwrap()))
                        .collect::<Vec<i32>>();
                    match &sections[..] {
                        &[first, second, third, fourth] => (first, second, third, fourth),
                        _ => unreachable!(),
                    }
                })
                .collect(),
        }
    }
}

impl FromFile for Day4 {}
impl Day<u32, u32> for Day4 {
    fn part1(&mut self) -> u32 {
        self.sections
            .iter()
            .map(|x| u32::from((x.0 <= x.2 && x.3 <= x.1) | (x.0 >= x.2 && x.3 >= x.1)))
            .sum()
    }
    fn part2(&mut self) -> u32 {
        self.sections
            .iter()
            .map(|x| u32::from((x.0 <= x.2 && x.2 <= x.1) | (x.0 >= x.2 && x.0 <= x.3)))
            .sum()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        assert_eq!(Day4::from_input(INPUT.to_string()).part1(), 2)
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day4::from_input(INPUT.to_string()).part2(), 4)
    }
}
