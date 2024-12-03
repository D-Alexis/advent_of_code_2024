use regex::Regex;
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
    let mut res = 0;
    let rex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    rex.captures_iter(input).for_each(|matc| {
        res += matc.get(1).unwrap().as_str().parse::<i32>().unwrap()
            * matc.get(2).unwrap().as_str().parse::<i32>().unwrap();
    });
    println!("{:?}", res);
}

fn part2(input: &str) {
    let mut res = 0;
    let mut donut = true;
    let rex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\)|don\'t\(\))").unwrap();
    rex.captures_iter(input).for_each(|matc| {
        // matc est sous la forme [matchTotal, group1, group2, group3] eg: [do(), None, None,do()] ou [mul(3,4), 3, 4, None]
        let first = matc.get(0).unwrap().as_str();
        if first == "do()" || first == "don't()" {
            donut = first == "do()"
        } else if donut {
            res += matc.get(1).unwrap().as_str().parse::<i32>().unwrap()
                * matc.get(2).unwrap().as_str().parse::<i32>().unwrap();
        }
    });
    println!("{:?}", res);
}
