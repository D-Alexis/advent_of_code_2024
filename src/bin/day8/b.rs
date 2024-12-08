use std::{collections::HashMap, collections::HashSet};
use advent_util;

pub fn solve(input: &str) {
    let (char_to_pos, max) = parse_input(input);
    let mut positions: HashSet<(i32,i32)> = HashSet::with_capacity(2000);
    char_to_pos.values().for_each(|vecc| count_antinode(vecc, &mut positions, max));
    
    println!("{:?}", positions.len());
}

fn count_antinode(antenas: &Vec<(i32,i32)>, positions: &mut HashSet<(i32,i32)>, max: i32) {
    for i in 0..antenas.len() - 1 {
        for j in i+1..antenas.len() {
            let (first, second) = (antenas[i], antenas[j]);
            positions.insert(first);
            positions.insert(second);
            let diff = (second.0 - first.0, second.1 - first.1);
            let mut antinode = (second.0 + diff.0, second.1 + diff.1);
            while advent_util::is_in_bound(antinode, max) {
                positions.insert(antinode);
                antinode = (antinode.0 + diff.0, antinode.1 + diff.1);
            }
            
            let mut antinode = (first.0 - diff.0, first.1 - diff.1);
            while advent_util::is_in_bound(antinode, max) {
                positions.insert(antinode);
                antinode = (antinode.0 - diff.0, antinode.1 - diff.1);
            }
        }
    }
}

fn parse_input(input:&str) -> (HashMap<char, Vec<(i32,i32)>>, i32) {
    let mut res : HashMap<char, Vec<(i32,i32)>> = HashMap::with_capacity(50);
    
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, charr)| {
            if charr != '.' {
                res.entry(charr).or_default().push((i as i32, j as i32));
            }
        })
    });
    
    (res, input.lines().count() as i32)
}