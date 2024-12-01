use std::{collections::HashMap, time::Instant};

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
    let (left, right) = parse_input(input);
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }
    println!("sum is : {:?}", sum);
}

fn part2(input: &str) {
    let mut left_vec = Vec::with_capacity(1000);
    let mut right_vec = HashMap::with_capacity(1000);
    input
        .trim_end()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();
            left_vec.push(left);
            *right_vec.entry(right).or_insert(0) += 1;
        });
    let a = left_vec
        .iter()
        .map(|left_value| left_value * right_vec.get(left_value).or_else(|| Some(&0)).unwrap())
        .sum::<i32>();
    println!("sum is : {:?}", a);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_vec = Vec::with_capacity(1000);
    let mut right_vec = Vec::with_capacity(1000);
    input
        .trim_end()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            let left = left.parse::<i32>().unwrap();
            let right = right.parse::<i32>().unwrap();
            left_vec.push(left);
            right_vec.push(right);
        });
    left_vec.sort();
    right_vec.sort();
    (left_vec, right_vec)
}
