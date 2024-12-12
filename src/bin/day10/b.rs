use advent_util::is_in_bound;
use std::collections::HashSet;
const DIRECTION: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub fn solve(input: &str) {
    let (map, start_positions) = parse_input(input);

    println!(
        "{}",
        start_positions.iter().fold(0, |acc, position| {
            let mut trail = 0;
            explore(*position, &map, &mut trail);
            acc + trail
        })
    );
}
fn explore(position: (i32, i32), map: &Vec<Vec<i32>>, trail: &mut usize) {
    let mut pos_list: Vec<(i32, i32)> = Vec::with_capacity(4);
    for dir in DIRECTION {
        let new_pos = (position.0 + dir.0, position.1 + dir.1);
        if is_in_bound(new_pos, map.len() as i32) {
            let val = map[new_pos.0 as usize][new_pos.1 as usize];
            if val - map[position.0 as usize][position.1 as usize] == 1 {
                if val == 9 {
                    *trail += 1;
                    continue;
                }
                pos_list.push(new_pos);
            }
        }
    }
    for pos in pos_list {
        explore(pos, map, trail);
    }
}

fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<(i32, i32)>) {
    let mut start_positions: Vec<(i32, i32)> = Vec::with_capacity(50);
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, value)| {
                    let val = value.to_digit(10).unwrap() as i32;
                    if val == 0 {
                        start_positions.push((i as i32, j as i32));
                    }
                    val
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    (map, start_positions)
}
