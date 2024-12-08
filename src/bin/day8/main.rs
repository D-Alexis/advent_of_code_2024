use std::time::Instant;
mod a;
mod b;
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
    a::solve(input);
}
fn part2(input: &str) {
    b::solve(input);
}