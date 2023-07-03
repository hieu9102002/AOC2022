pub mod question;

use question::question_4::part_1;
use question::question_4::part_2;
use std::fs;

fn main() {
    let input = "inputs/input4";

    let contents = fs::read_to_string(input)
        .expect("Input file does not exist");

    let result = part_1(&contents);
    let result2 = part_2(&contents);

    println!("Part 1: {}", result);
    println!("Part 2: {}", result2);
}

