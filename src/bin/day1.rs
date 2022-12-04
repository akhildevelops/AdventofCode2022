use adventofcode_2022::*;
fn main() {
    // Day1 Solution
    let mut day1 = Day1::from_input_file(&format!("./src/day1/{}", INPUT_FILE_NAME));
    println!("part-1:{},part-2:{}", day1.part1(), day1.part2());
}
