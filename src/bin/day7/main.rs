use std::time::Instant;
fn main() {
    let input = include_str!("input.txt");
    let start = Instant::now();
    part1(input);
    println!("day1 took : {:?}", start.elapsed());

    let start = Instant::now();
    part2(input);
    println!("day2 took : {:?}", start.elapsed());
}

fn part1(input: &str) {
    let operations = vec![0,1];
    let input_datas = parse(input);
    println!("{}",
        input_datas.iter().fold(0,|acc, (target, numbers)| acc + if is_valid(*target, numbers, &operations){*target}else{0}));
}

fn part2(input: &str) {
    let operations = vec![0,1,2];
    let input_datas = parse(input);
    println!("{}",
        input_datas.iter().fold(0,|acc, (target, numbers)| acc + if is_valid(*target, numbers, &operations){*target}else{0}));
}
fn is_valid(target: u64, numbers: &Vec<u64>, operations: &Vec<u8>) -> bool {
    let acc = numbers[0];
    let mut is_valid = false;
    for ope in operations {
        is_valid |= check(target, acc, *ope, operations,&numbers[1..]);
    }
    is_valid
}

fn check(target: u64, mut acc: u64, current_ope: u8, operations: &Vec<u8>, numbers :&[u64]) -> bool {
    let current = numbers[0];
    match current_ope {
        0 => acc += current,
        1 =>  acc *= current,
        2 => acc = concat(acc, current),
        _ => panic!("non existing operations")
    }
    if numbers.len() > 1 {
        let mut is_valid = false;
        for ope in operations {
            if is_valid { return is_valid}
            is_valid |= check(target, acc, *ope, operations,&numbers[1..]);
        }
        return is_valid
    } else {
        return target == acc
    }
}

fn concat( acc: u64, value: u64) -> u64 {
    let mut val = acc.to_string();
    val.push_str(&value.to_string());
    val.parse::<u64>().unwrap()
}
fn parse(input: &str) ->Vec<(u64, Vec<u64>)> {
    input.lines().map(|line| {
        let (value, numbers) = line.split_once(": ").unwrap();
        let vec_numbers = numbers.split_whitespace().map(|number_c| number_c.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        (value.parse::<u64>().unwrap(), vec_numbers)
    })
    .collect::<Vec<(u64,Vec<u64>)>>()
}