use std::fs;
pub trait Day<T, U> {
    fn part1(&mut self) -> T;
    fn part2(&mut self) -> U;
}

pub trait FromStr {
    fn from_input(input: String) -> Self;
}

pub trait FromFile: FromStr
where
    Self: Sized,
{
    fn from_input_file(path: &str) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        Self::from_input(contents)
    }
}

pub const INPUT_FILE_NAME: &str = "input.txt";

mod day1;
mod day2;
mod day3;
mod day4;
pub use day1::Day1;
pub use day2::Day2;
pub use day3::Day3;
pub use day4::Day4;
