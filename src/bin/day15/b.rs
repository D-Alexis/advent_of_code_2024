use std::collections::{HashSet, VecDeque};

pub fn solve(input: &str) {
    let mut map_moves = parse_input(input);
    let (mut map, moves, mut position) = (map_moves.map, map_moves.moves, map_moves.position);

    moves.iter().for_each(|moves| {
        let (i, j) = (position.0 as i32 + moves.0, position.1 as i32 + moves.1);
        let (i, j) = (i as usize, j as usize);

        let next = map[i][j];
        if next != '#' {
            if next == '.' {
                // on bouge le joueur
                map[i][j] = '@';
                map[position.0][position.1] = '.';
                position = (i, j)
            } else {
                if moves.0 == 0 {
                    move_horizontal(&mut position, moves, &mut map);
                } else {
                    move_vertical(&mut position, moves, &mut map);
                }
            }
        }
    });

    advent_util::print_matrice(&map);
    let mut res = 0;
    map.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, charr)| {
            if *charr == '[' {
                res += i * 100 + j;
            }
        });
    });
    println!("{:?}", res)
}

fn move_horizontal(pos: &mut (usize, usize), moves: &(i32, i32), map: &mut Vec<Vec<char>>) {
    if let Some(next_empty) = find_next_empty(pos, moves, &map) {
        let mut char_to_move = map[pos.0][pos.1]; // should be the player
        assert!(char_to_move == '@', " the fuck is hapenning?");

        map[pos.0][pos.1] = '.';
        let next_robot = (pos.0 as i32 + moves.0, pos.1 as i32 + moves.1);
        let mut next = next_robot;

        while next.1 != next_empty.1 as i32 {
            let tmp = map[next.0 as usize][next.1 as usize];
            map[next.0 as usize][next.1 as usize] = char_to_move;
            char_to_move = tmp;
            next = (next.0 as i32 + moves.0, next.1 as i32 + moves.1);
        }
        map[next.0 as usize][next.1 as usize] = char_to_move;
        // on bouge le joueur à coté, et le premier mur aux prochain espace libre
        pos.0 = next_robot.0 as usize;
        pos.1 = next_robot.1 as usize;
    }
}

fn find_next_empty(pos: &mut (usize, usize), moves: &(i32, i32), map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut next = (pos.0 as i32 + moves.0, pos.1 as i32 + moves.1);
    while map[next.0 as usize][next.1 as usize] == '[' || map[next.0 as usize][next.1 as usize] == ']' {
        next = (next.0 + moves.0, next.1 + moves.1);
    }
    if map[next.0 as usize][next.1 as usize] == '.' {
        return Some((next.0 as usize, next.1 as usize));
    }
    // next is '#'
    None
}

fn move_vertical(pos: &mut (usize, usize), moves: &(i32, i32), map: &mut Vec<Vec<char>>) {
    // on ne va stocker que '[' , pas besoin des 2 coordonnées
    if let Some(boxes) = find_boxes_to_move(pos, moves, map) {
        // let's move all
        let mut vec_box = boxes.into_iter().collect::<Vec<(i32, i32)>>();
        vec_box.sort_by(|a, b| a.0.cmp(&b.0));
        if moves.0 == 1 {
            vec_box.reverse(); // on va les parcourir du plus loin au plus près du joueur
        }
        for ele in vec_box {
            let next = (ele.0 as i32 + moves.0, ele.1 as i32 + moves.1);
            map[next.0 as usize][next.1 as usize] = '[';
            map[next.0 as usize][next.1 as usize + 1] = ']';
            map[ele.0 as usize][ele.1 as usize] = '.';
            map[ele.0 as usize][ele.1 as usize + 1] = '.';
        }
        map[pos.0 as usize][pos.1 as usize] = '.';
        let next_robot = (pos.0 + moves.0 as usize, pos.1 + moves.1 as usize);
        map[pos.0 + moves.0 as usize][pos.1 + moves.1 as usize] = '@';
        pos.0 = next_robot.0;
        pos.1 = next_robot.1;
    }
}
fn find_boxes_to_move(
    pos: &mut (usize, usize),
    moves: &(i32, i32),
    map: &mut Vec<Vec<char>>,
) -> Option<HashSet<(i32, i32)>> {
    let mut valid_boxes: HashSet<(i32, i32)> = HashSet::with_capacity(20);
    let mut next = (pos.0 as i32 + moves.0, pos.1 as i32 + moves.1);
    let mut boxes_to_check: VecDeque<(i32, i32)> = VecDeque::with_capacity(30);
    if map[next.0 as usize][next.1 as usize] == ']' {
        next.1 -= 1;
    }
    boxes_to_check.push_back(next);

    while boxes_to_check.len() > 0 {
        let boxx = boxes_to_check.pop_front().unwrap();
        next = (boxx.0 as i32 + moves.0, boxx.1 as i32 + moves.1);
        if map[next.0 as usize][next.1 as usize] != '#' && map[next.0 as usize][next.1 as usize + 1] != '#' {
            // current boxx is valid
            valid_boxes.insert(boxx);
            check_a(map, next, &mut boxes_to_check);
            check_a(map, (next.0, next.1 + 1), &mut boxes_to_check);
        } else {
            return None;
        }
    }
    Some(valid_boxes)
}
fn check_a(map: &mut Vec<Vec<char>>, next: (i32, i32), boxes_to_check: &mut VecDeque<(i32, i32)>) {
    if map[next.0 as usize][next.1 as usize] != '.' {
        // pas un . ni un # donc [ ou ]
        if map[next.0 as usize][next.1 as usize] == '[' {
            boxes_to_check.push_back(next);
        } else {
            // ']'
            boxes_to_check.push_back((next.0, next.1 - 1));
        }
    }
}
fn parse_input(input: &str) -> MapMove {
    let (map_input, moves) = input.split_once("\n\n").unwrap();
    let max = map_input.split_once("\n").unwrap().0.len();
    let mut position = (0, 0);
    let mut map = vec![vec!['.'; max * 2]; max];
    map_input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, charr)| {
            if charr == '@' {
                position = (i, 2 * j)
            }
            map[i][2 * j] = charr;
            match charr {
                '@' => map[i][2 * j + 1] = '.',
                '.' => map[i][2 * j + 1] = '.',
                'O' => {
                    map[i][2 * j] = '[';
                    map[i][2 * j + 1] = ']'
                }
                '#' => map[i][2 * j + 1] = '#',
                _ => panic!("not a bvalid symbol"),
            }
        })
    });

    let moves = moves
        .lines()
        .map(|line| {
            line.chars()
                .map(|charr| match charr {
                    '<' => (0, -1),
                    '^' => (-1, 0),
                    '>' => (0, 1),
                    'v' => (1, 0),
                    _ => panic!("not a move"),
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect::<Vec<(i32, i32)>>();

    MapMove { map, position, moves }
}

#[derive(Debug)]
struct MapMove {
    pub map: Vec<Vec<char>>,
    pub position: (usize, usize),
    pub moves: Vec<(i32, i32)>,
}
