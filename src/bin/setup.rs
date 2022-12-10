use std::fs;
use std::io::Write;
const BINDIR: &str = "./src/bin";
const LIBRS: &str = "./src/lib.rs";
const SRC: &str = "./src";
fn main() {
    let day = "day11";
    let day_captialize = day[..1].to_uppercase() + &day[1..];
    // Create binary file
    let bin_content = format!(
        "use adventofcode_2022::*;
    fn main() {{
        // {day} Solution
        let mut day = {}::from_input_file(&format!(\"./src/{day}/{{}}\", INPUT_FILE_NAME));
        println!(\"part-1:{{}},part-2:{{}}\", day.part1(), day.part2());
    }}
    ",
        &day_captialize
    );
    let mut file = fs::File::create(format!("{BINDIR}/{day}.rs")).unwrap();

    file.write_all(bin_content.as_bytes()).unwrap();

    // Add module in lib.rs
    let lib_rs_content = format!(
        "
mod {day};
pub use {day}::{};
",
        &day_captialize
    );

    let mut file = fs::OpenOptions::new().append(true).open(LIBRS).unwrap();
    file.write(lib_rs_content.as_bytes()).unwrap();

    // Create day's directory
    let day_folder = format!("{SRC}/{day}");
    fs::create_dir(&day_folder).unwrap();

    // mod.rs
    let contents_modrs = format!(
        "use crate::{{Day, FromFile, FromStr as CrateFromStr}};
pub struct {day_captialize};

impl CrateFromStr for {day_captialize} {{
    fn from_input(_input: String) -> Self {{
        Self
    }}
}}

impl FromFile for {day_captialize} {{}}

impl Day<u32, u32> for {day_captialize} {{
    fn part1(&mut self) -> u32 {{
        10
    }}
    fn part2(&mut self) -> u32 {{
        10
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const INPUT: &str = \"\";

    #[test]
    fn test_part1() {{
        assert_eq!({day_captialize}::from_input(INPUT.to_string()).part1(), 10);
    }}
    #[test]
    fn test_part2() {{
        assert_eq!({day_captialize}::from_input(INPUT.to_string()).part2(), 10);
    }}
}}"
    );
    let mut file = fs::File::create(format!("{day_folder}/mod.rs")).unwrap();
    file.write_all(contents_modrs.as_bytes()).unwrap();

    // create input.txt and Readme.md
    fs::File::create(format!("{day_folder}/input.txt")).unwrap();
    fs::File::create(format!("{day_folder}/Readme.md")).unwrap();
}
