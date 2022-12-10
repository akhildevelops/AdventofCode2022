# AdventofCode2022
Solutions for Advent of Code 2022

[![Advent Of Code](https://miro.medium.com/max/1200/1*XtCMwEXZe2VcH-jfcHwCBQ.jpeg)](https://adventofcode.com/)

This is a repository containing solutions for Advent of Code Challenge. 

It is held once in every year for 25 days from 1st to 25th December 

We need to solve two problems each day. For solving we are awarded with stars.

The challenge is unique compared to other competitive programming challenges.

The solutions are written in [Rust](https://www.rust-lang.org/) a high performant language.

To get each day's solution, clone this repo: https://github.com/akhildevelops/AdventofCode2022 and run respective command as shown below

| Day | Story | Solution | Part1 | Part2 | command to run | challenge level | What I learnt
| --- | --- | --- | --- | --- | --- | --- | --- |
| Dec-1 | [Calorie Counting](https://adventofcode.com/2022/day/1) | [Day1](./src/day1/mod.rs) 🚢 | ⭐ | ⭐  | `cargo solve day1` | 🧗| Rust's Result methods
| Dec-2 | [Rock Paper Scissors](https://adventofcode.com/2022/day/2) | [Day2](./src/day2/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day2`|🧗 🧗| Mental Map scenarios/states to integers
| Dec-3 | [Rucksack Reorganization](https://adventofcode.com/2022/day/3) | [Day3](./src/day3/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day3`| 🧗🧗| Sets to find common elements
| Dec-4 | [Camp Cleanup](https://adventofcode.com/2022/day/4) | [Day4](./src/day4/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day4`|🧗🧗| Check if an element is present in the boundaries.
| Dec-5 | [Supply Stacks](https://adventofcode.com/2022/day/5) | [Day5](./src/day5/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day5`| 🧗🧗🧗 | Parse text into columns and borrow checker
| Dec-6 | [Tuning Trouble](https://adventofcode.com/2022/day/6) | [Day6](./src/day6/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day6`|🧗| Find non repeating chars in a string.
| Dec-7 | [No Space Left On Device](https://adventofcode.com/2022/day/7) |  |  |  | |  |
| Dec-8 | [Treetop Tree House](https://adventofcode.com/2022/day/8) | [Day8](./src/day8/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day8`|🧗🧗| Grid type data structures + create references, not to be owned by closures
| Dec-9 | [Rope Bridge](https://adventofcode.com/2022/day/9) | [Day9](./src/day9/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day9`|🧗🧗| Hashets and iterators
| Dec-10 | [Cathode-Ray Tube](https://adventofcode.com/2022/day/10) | [Day10](./src/day10/mod.rs) 🚢 | ⭐ | ⭐ | `cargo solve day10`|🧗*1.5| Code comprehension >>> Code optimization

## For Contributors / Forkers:

To create a new project for a particular day run `cargo setup <name>`

Ex: To setup day15 run: `cargo setup day15` It creates below structure:
```
src/
├── bin/
│   ├── ...
│   ├── day15.rs      // day15.rs binary file is created that prints part1 and part2 solutions.  
│   └── ...
├── lib.rs           // Get's updated with day15 module for importing into binary.
├── ...
├── day15           // day15 module is created with input.txt for puzzle input, Readme.md for any Notes and mod.rs containing logic./
│   ├── input.txt
│   ├── mod.rs
│   └── Readme.md
└── ...
```