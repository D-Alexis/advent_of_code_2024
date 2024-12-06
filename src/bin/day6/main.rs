use std::{collections::HashSet, thread::current, time::Instant};

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
   let (map, initial_position) = parse_input(input);
   let mut visited = HashSet::with_capacity(1000);
   visited.insert(initial_position);
   let mut current_pos = initial_position;
   let mut moves_helper = Move::new();
   let mut next_move = moves_helper.next();
   loop {
       let next_pos = (current_pos.0 + next_move.0, current_pos.1 + next_move.1);
       if is_in_bound(next_pos, map.len() as i32) {
           let char = map[next_pos.0 as usize][next_pos.1 as usize];
           if char != '#' {
               current_pos = next_pos;
               visited.insert(current_pos);
           } else {
               next_move = moves_helper.next();
           }
       } else {
           println!("{:?}", next_move);
           println!("{:?}", current_pos);
           break;
       }
   }
   println!("{:?}", visited.len());
}

fn part2(input:&str) {
    let (mut map, initial_position) = parse_input(input);
    let mut sum = 0;
    let mut blocked = HashSet::with_capacity(2000);
    let mut current_pos = initial_position;
    let mut moves_helper = Move::new();
    let mut next_move = moves_helper.next();
    
    for i in 0..map.len() {
        for j in 0..map.len() {
            if map[i][j] == '.' {
                map[i][j] = '#';
                if check(&map, initial_position) {blocked.insert((i,j));};
                map[i][j] = '.';
            }
        }
    }
    println!("{:?}", blocked.len());
}

fn check(map: &Vec<Vec<char>>, initial_position:(i32,i32)) -> bool {
    
    let mut  step = 0;
    let mut current_pos = initial_position;
    let mut moves_helper = Move::new();
    let mut next_move = moves_helper.next();
    loop {
        let next_pos = (current_pos.0 + next_move.0, current_pos.1 + next_move.1);
        if is_in_bound(next_pos, map.len() as i32) {
            let char = map[next_pos.0 as usize][next_pos.1 as usize];
            if char != '#' {
                current_pos = next_pos;
                step+=1;
                if step > 6000 {
                    return true;
                }
            } else {
                next_move = moves_helper.next();
            }
        } else {
            break;
        }
    }
    false
} 
/* 
fn part2(input: &str) {
    let (mut map, initial_position) = parse_input(input);
    let mut blocked = HashSet::new();
    let mut current_pos = initial_position;
    let mut moves_helper = Move::new();
    let mut next_move = moves_helper.next();
    loop {
        let next_pos = (current_pos.0 + next_move.0, current_pos.1 + next_move.1);
        if is_in_bound(next_pos, map.len() as i32) {
            let char = map[next_pos.0 as usize][next_pos.1 as usize];
            if char != '#' {
                if char != '^' {
                     if check_loop(&mut map, current_pos.clone(), next_pos, moves_helper.clone()) {
                         blocked.insert(next_pos);
                     };
                }
                current_pos = next_pos;
            } else {
                next_move = moves_helper.next();
            }
        } else {
            println!("{:?}", next_move);
            println!("{:?}", current_pos);
            break;
        }
    }
    println!("{:?}", blocked.len());
}
fn check_loop(map: &mut Vec<Vec<char>>, mut current_pos: (i32,i32), obstacle: (i32,i32), mut moves_helper: Move) -> bool {
    let val = map[obstacle.0 as usize][obstacle.1 as usize];
    map[obstacle.0 as usize][obstacle.1 as usize] = '#';
    let mut next_move = moves_helper.next();
    let mut step = 0;
    loop {
        let next_pos = (current_pos.0 + next_move.0, current_pos.1 + next_move.1);
        if is_in_bound(next_pos, map.len() as i32) {
            let char = map[next_pos.0 as usize][next_pos.1 as usize];
            if char != '#' {
                current_pos = next_pos;
                step+=1;
                if step >=100000 {  // technique de merde mais si ca marche
                   map[obstacle.0 as usize][obstacle.1 as usize] = val;
                    return true;
                }
            } else {
                next_move = moves_helper.next();
            }
        } else {
            break;
        }
    }
    map[obstacle.0 as usize][obstacle.1 as usize] = val;
    false
}
*/
fn parse_input(input: &str) -> (Vec<Vec<char>>, (i32, i32)) {
    let mut initial_position = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, char)| {
                    if char == '^' {
                        initial_position = (i as i32, j as i32);
                    }
                    return char;
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    (map, initial_position)
}

#[derive(Clone,Debug)]
struct Move {
    directions: Vec<(i32, i32)>,
    current: usize,
    started: bool
}

impl Move {
    pub fn new() -> Self {
        Self {
            directions: vec![(-1, 0), (0, 1), (1, 0), (0, -1)],
            current: 0,
            started: false
        }
    }
    
    pub fn  next(&mut self) -> (i32, i32) {
        if !self.started {
            self.started = true;
            return self.directions[0]
        }
        self.current = (self.current + 1) % 4;
        self.directions[self.current]
    }
}

fn is_in_bound((i, j): (i32, i32), max: i32) -> bool {
    i >= 0 && i < max && j >= 0 && j < max
}