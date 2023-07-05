pub fn part_1(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, x| {
            let split: Vec<&str> = x.split(',').collect();
            (acc + if contains(split[0], split[1]) {1} else {0})
        })
        .to_string()
}

pub fn part_2(input: &str) -> String {
    input
        .lines()
        .fold(0, |acc, x| {
            let split: Vec<&str> = x.split(',').collect();
            acc + if overlap(split[0], split[1]) {1} else {0}
        })
        .to_string()
}

fn contains(first: &str, second: &str) -> bool {
    let (first_low, first_high) = split(first);
    let (second_low, second_high) = split(second) ;

    (first_low <= second_low && first_high >= second_high) 
        || (second_low <= first_low && second_high >= first_high)

}

fn overlap(first: &str, second: &str) -> bool {
    let (first_low, first_high) = split(first);
    let (second_low, second_high) = split(second) ;

    first_low <= second_high && second_low <= first_high
}

fn split(input: &str) -> (usize, usize) {
    let split:Vec<usize> = input.split('-').map(|x| x.parse::<usize>().unwrap()).collect();

    (split[0], split[1])
}