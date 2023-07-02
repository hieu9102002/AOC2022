pub fn part_1(input: &String) -> String {
    input
        .lines()
        .map(|x| x.split(" ").collect())
        .map(|x: Vec<&str>| calculate_score(x[0], x[1]))
        .fold(0, |acc, next| acc + next)
        .to_string()
}

pub fn part_2(input: &String) -> String {
    input
        .lines()
        .map(|x| x.split(" ").collect())
        .map(|x: Vec<&str>| calculate_score_2(x[0], x[1]))
        .fold(0, |acc, next| acc + next)
        .to_string()
}

fn calculate_score(opp: &str, me: &str) -> i32 {
    let score_arr = vec![
        vec![4, 8, 3], 
        vec![1, 5, 9], 
        vec![7, 2, 6]];
        
    let opp = match opp {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("unexpected"),
    };
    let me = match me {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("unexpected"), 
    };

    return score_arr[opp][me];
}

fn calculate_score_2(opp: &str, outcome: &str) -> i32 {
    let score_arr = vec![
        vec![3, 4, 8], 
        vec![1, 5, 9], 
        vec![2, 6, 7]];

    let opp = match opp {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => panic!("unexpected"),
    };
    let outcome = match outcome {
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("unexpected"), 
    };

    return score_arr[opp][outcome]
}