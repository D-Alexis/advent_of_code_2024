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
    let input = parse_input(input);
    let a = input
        .iter()
        .fold(0, |acc, report| acc + if is_safe(report) { 1 } else { 0 });
    println!("{a}");
}

fn part2(input: &str) {
    let input = parse_input(input);
    let a = input.iter().fold(0, |acc, report| {
        acc + if is_safe_with_permutation(report) { 1 } else { 0 }
    });
    println!("{a}");
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim_end()
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let foward = if (report[1] - report[0]) > 0 {
        true
    } else if (report[1] - report[0]) < 0 {
        false
    } else {
        return false; // unsafe
    };
    for i in 1..report.len() {
        let diff = if foward {
            report[i] - report[i - 1]
        } else {
            report[i - 1] - report[i]
        };
        if !(diff >= 1 && diff <= 3) {
            return false; // unsafe
        }
    }
    true
}

fn is_safe_with_permutation(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut permutated = report.clone();
        permutated.remove(i);
        if is_safe(&permutated) {
            return true;
        }
    }
    false
}
