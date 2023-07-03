pub mod question;

use question::question_3::part_1;
use question::question_3::part_2;
use std::fs;

fn main() {
    let input = "inputs/input3";

    let contents = fs::read_to_string(input)
        .expect("Input file does not exist");

    let result = part_1(&contents);
    let result2 = part_2(&contents);

    println!("Part 1: {}\nPart 2: {}", result, result2);
}

