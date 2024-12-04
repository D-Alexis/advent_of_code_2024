use std::time::Instant;

const DIRECTION: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
const CROSS_LINES: [((i32, i32), (i32, i32)); 2] = [((-1, -1), (1, 1)), ((1, -1), (-1, 1))];
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
    let char_matrice: Vec<Vec<char>> = parse_input(input);
    let mut sum = 0;
    for i in 0..char_matrice.len() {
        for j in 0..char_matrice.len() {
            if char_matrice[i][j] == 'X' {
                sum += search_for_MAS((i as i32, j as i32), &char_matrice);
            }
        }
    }
    println!("{:?}", sum);
}

fn part2(input: &str) {
    let char_matrice: Vec<Vec<char>> = parse_input(input);
    let mut sum = 0;
    for i in 0..char_matrice.len() {
        for j in 0..char_matrice.len() {
            if char_matrice[i][j] == 'A' {
                sum += if is_X_MAS((i as i32, j as i32), &char_matrice) {
                    1
                } else {
                    0
                };
            }
        }
    }
    println!("{:?}", sum);
}
fn search_for_MAS((i, j): (i32, i32), char_matrice: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for dir in DIRECTION {
        let mut is_ok = true;
        let (mut next_i, mut next_j) = (i, j);
        let mut next_char_iter = ['M', 'A', 'S'].iter();
        while let Some(next_char) = next_char_iter.next() {
            (next_i, next_j) = (next_i + dir.0, next_j + dir.1);
            if is_in_bound((next_i, next_j), char_matrice.len() as i32)
                && *next_char == char_matrice[next_i as usize][next_j as usize]
            {
                is_ok = true;
            } else {
                is_ok = false;
                break;
            }
        }
        if is_ok && next_char_iter.next() == None {
            sum += 1;
        }
    }
    sum
}
fn is_X_MAS((i, j): (i32, i32), char_matrice: &Vec<Vec<char>>) -> bool {
    if i == 0 || j == 0 || i == char_matrice.len() as i32 - 1 || j == char_matrice.len() as i32 - 1 {
        // A can't be on any side of the square
        return false;
    }
    for (first_point, second_point) in CROSS_LINES {
        let (first, second) = (
            char_matrice[(i + first_point.0) as usize][(j + first_point.1) as usize],
            char_matrice[(i + second_point.0) as usize][(j + second_point.1) as usize],
        );
        if !((first == 'M' && second == 'S') || (first == 'S' && second == 'M')) {
            return false;
        }
    }
    true
}

fn is_in_bound((i, j): (i32, i32), max: i32) -> bool {
    i >= 0 && i < max && j >= 0 && j < max
}
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
