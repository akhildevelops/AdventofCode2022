use adventofcode_2022::*;
fn main() {
    // template Solution
    let mut day = Template::from_input_file(&format!("./src/template/{}", INPUT_FILE_NAME));
    println!("part-1:{},part-2:{}", day.part1(), day.part2());
}
