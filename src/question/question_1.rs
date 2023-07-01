pub fn part_1(contents: &String) -> String {
    return split_string_to_sum_seperated_by(contents, "")
        .iter()
        .max()
        .unwrap_or_else(|| &0)
        .to_string() 
}

pub fn part_2(contents: &String) -> String {
    let mut vec = split_string_to_sum_seperated_by(contents, "");
    vec.sort();

    return match &vec[..] {
        [.., a, b, c] => (a + b + c).to_string(),
        _ => "0".to_string()
    };
}

fn split_string_to_sum_seperated_by(contents: &String, separator: &str) -> Vec<i32> {
    return contents
        .lines()
        .fold(vec![0], |mut acc, x| {
            if x == separator {
                acc.push(0)
            } else {
                *acc.last_mut().unwrap() += x.parse::<i32>().unwrap_or_else(|_|0)
            }
            return acc;
        })
}