pub fn part_1(input:&str) -> String {
    let (initial_state, instructions) = input
        .split_once("\n\n")
        .unwrap();

    let mut stacks = parse_initial_state(initial_state);

    instructions
        .lines()
        .for_each(|x| 
            execute_command(&mut stacks, x));

    stacks
        .iter()
        .fold(String::new(), |mut acc, x| {
            acc.push(*x.last().unwrap_or(&' '));
            acc
        })
}

pub fn part_2(input:&str) -> String {
    let (initial_state, instructions) = input
        .split_once("\n\n")
        .unwrap();

    let mut stacks = parse_initial_state(initial_state);

    instructions
        .lines()
        .for_each(|x| 
            execute_command_2(&mut stacks, x));

    stacks
        .iter()
        .fold(String::new(), |mut acc, x| {
            acc.push(*x.last().unwrap_or(&' '));
            acc
        })
}

fn execute_command_2(stacks: &mut [Vec<char>; 9], command: &str) {
    let command_split: Vec<&str> = command.split(' ').collect();
    
    let amount:usize = command_split[1].parse().expect("amount");

    let mut split = {
        let from = &mut stacks[command_split[3].parse::<usize>().expect("from")-1];
        from.split_off(from.len() - amount)
    };

    let to = &mut stacks[command_split[5].parse::<usize>().expect("to")-1]; 

    to.append(&mut split);

}
fn execute_command(stacks: &mut [Vec<char>; 9], command: &str) {
    let command_split: Vec<&str> = command.split(' ').collect();
    
    let amount:usize = command_split[1].parse().expect("amount");

    let mut split = {
        let from = &mut stacks[command_split[3].parse::<usize>().expect("from")-1];
        let mut ret: Vec<char> = vec![];
        for _ in 0..amount {
            ret.push(from.pop().unwrap())
        }
        ret
    };

    let to = &mut stacks[command_split[5].parse::<usize>().expect("to")-1]; 

    to.append(&mut split);

}

fn parse_initial_state(initial_state: &str) -> [Vec<char>; 9] {
    initial_state
        .lines()
        .rev()
        .skip(1)
        .fold(Default::default(),|mut acc:[Vec<char>; 9], x| {
            (0..=8).for_each(|i| {
                let char = x.chars().nth(i*4+1).unwrap_or(' ');
                if char == ' '{
                    return;
                }
                acc[i].push(char)
            });
            acc
        })
}