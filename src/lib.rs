pub trait Day<T, U> {
    fn part1(&mut self) -> T;
    fn part2(&mut self) -> U;
}

pub trait FromFile {
    fn from_input_file(path: &str) -> Self;
}

pub const INPUT_FILE_NAME: &str = "input.txt";

mod day1;
mod day2;
pub use day1::Day1;
pub use day2::Day2;
