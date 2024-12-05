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
    let (rules, updates) = parse_input(input);
    println!(
        "{}",
        updates.iter().map(|update| {
            let is_ok = check_if_valid(&rules, update);
        if is_ok {
            update[update.len()/2] // (n/2)+1-1, oui toute les lignes sont impair
        } else {
            0
        }
    }).sum::<i32>());
}

fn part2(input: &str) {
    let (rules, mut updates) = parse_input(input);
    println!(
        "{}",
        updates.iter_mut().map(|update| {
            let is_ok = check_if_valid(&rules, update);
            if is_ok {
                0
            } else {
                let mut has_changed = true;
                while has_changed {
                    has_changed = false;
                    let length = update.len();
                    for i in 0..length-1 {
                        if let Some(list) = rules.get(&(update[i])) {
                            if !list.contains(&(update[i] , update[i+1])) {
                                let val = update[i];
                                update[i] = update[i+1];
                                update[i+1] = val;
                                has_changed = true;
                            }
                        } else { // if it has no rules where it is first element (which is only in test and not in real data, fuck you btw)
                            let val = update[i];
                            update[i] = update[length-1];
                            update[length-1] = val;
                            has_changed = true;
                        }
                    }
                }

                update[update.len()/2] // (n/2)+1-1, oui toute les lignes sont impair
            }
        }).sum::<i32>());
}


fn check_if_valid(rules: &HashMap<i32, Vec<(i32, i32)>>, update: &Vec<i32>) -> bool {
    let mut is_ok = true;
    for i in 0..update.len() - 1 {
        for j in i + 1..update.len() {
            if let Some(list) = rules.get(&(update[i])) {
                if !list.contains(&(update[i] , update[j])) {
                    is_ok = false;
                    break;
                }
            } else {
                is_ok = false;
                break;
            }
        }
    }
    is_ok
}

fn parse_input(input: &str) -> (HashMap<i32, Vec<(i32, i32)>>, Vec<Vec<i32>>) {
    let mut rules_map: HashMap<i32, Vec<(i32, i32)>> = HashMap::with_capacity(1200);
    let (first, second) = input.split_once("\n\n").unwrap();
    first.lines().for_each(|line| {
        let (left, right) = line.split_once('|').unwrap();
        let (left, right) = (left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap());
        rules_map
            .entry(left)
            .or_insert(Vec::with_capacity(15))
            .push((left, right));
    });
    let updates = second
        .lines()
        .map(|line| {
            line.split(',')
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    (rules_map, updates)
}
