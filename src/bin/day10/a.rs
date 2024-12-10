use advent_util::is_in_bound;
use std::collections::HashSet;
const DIRECTION: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

pub fn solve(input: &str) {
    let (map, start_positions) = parse_input(input);

    println!(
        "{}",
        start_positions.iter().fold(0, |acc, position| {
            let mut top_positions: HashSet<(i32, i32)> = HashSet::with_capacity(20);
            let mut explored_pos: HashSet<(i32, i32)> = HashSet::with_capacity(1000);
            explore(*position, &map, &mut explored_pos, &mut top_positions);
            acc + top_positions.len()
        })
    );
}
fn explore(
    position: (i32, i32),
    map: &Vec<Vec<i32>>,
    explored_pos: &mut HashSet<(i32, i32)>,
    top_set: &mut HashSet<(i32, i32)>,
) {
    explored_pos.insert(position);
    let mut pos_list: Vec<(i32, i32)> = Vec::with_capacity(4);
    for dir in DIRECTION {
        let new_pos = (position.0 + dir.0, position.1 + dir.1);
        if !explored_pos.contains(&new_pos) {
            if is_in_bound(new_pos, map.len() as i32) {
                let val = map[new_pos.0 as usize][new_pos.1 as usize];
                if val - map[position.0 as usize][position.1 as usize] == 1 {
                    if val == 9 {
                        top_set.insert(new_pos);
                        continue;
                    }
                    pos_list.push(new_pos);
                }
            }
        }
    }
    for pos in pos_list {
        explore(pos, map, explored_pos, top_set);
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
