use crate::{Day, FromFile, FromStr as CrateFromStr};

use std::str::FromStr;

pub struct Day10(Vec<OP>);

enum OP {
    NOOP,
    ADD(i32),
}

impl FromStr for OP {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut op_nu = s.split(' ');
        let op = op_nu.next().unwrap();
        match op {
            "addx" => Ok(Self::ADD(op_nu.next().unwrap().parse().unwrap())),
            "noop" => Ok(Self::NOOP),
            _ => Err(()),
        }
    }
}

impl CrateFromStr for Day10 {
    fn from_input(input: String) -> Self {
        Self(input.lines().map(|x| OP::from_str(x).unwrap()).collect())
    }
}

impl FromFile for Day10 {}
impl Day<i32, String> for Day10 {
    fn part1(&mut self) -> i32 {
        let mut cycle = 1;
        let mut value = 1;
        let mut ad_val = 0;
        let mut final_value = 0;
        let mut inprogress = false;
        let mut actions = self.0.iter();
        while cycle <= 220 {
            if (cycle - 20) % 40 == 0 {
                final_value += cycle * value;
            }
            if !inprogress {
                match actions.next().unwrap() {
                    OP::ADD(x) => {
                        ad_val = *x;
                        inprogress = !inprogress
                    }
                    _ => {}
                }
            } else {
                value += ad_val;
                inprogress = !inprogress
            }
            cycle += 1;
        }
        final_value
    }
    fn part2(&mut self) -> String {
        let mut cycle = 1;
        let mut inprogress = false;
        let mut screen = vec![];
        let mut ad_val = 0;
        let mut sprite_loc = 1;
        let mut actions = self.0.iter();
        while cycle <= 240 {
            let n_sprite = 40 * (cycle / 40) + sprite_loc;
            if [n_sprite, n_sprite + 1, n_sprite + 2]
                .iter()
                .any(|x| cycle == *x)
            {
                screen.push('#');
            } else {
                screen.push('.')
            }
            if cycle % 40 == 0 {
                screen.push('\n')
            }

            if !inprogress {
                match actions.next().unwrap() {
                    OP::ADD(x) => {
                        ad_val = *x;
                        inprogress = !inprogress
                    }
                    _ => {}
                }
            } else {
                sprite_loc += ad_val;
                inprogress = !inprogress
            }
            cycle += 1;
        }
        let screen_str = screen.into_iter().collect();
        // println!("{}", screen_str);
        screen_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_moves_stack() {
        Day10::from_input(INPUT.to_string());
    }
    #[test]
    fn test_part1() {
        assert_eq!(Day10::from_input(INPUT.to_string()).part1(), 13140)
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day10::from_input(INPUT.to_string()).part2(),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......###.
#######.......#######.......#######.....
"
        )
    }
}
