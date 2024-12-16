pub fn solve(input: &str) {
    let mut map_moves = parse_input(input);
    let (mut map, moves, mut position) = (map_moves.map, map_moves.moves, map_moves.position);

    moves.iter().for_each(|moves| {
        let (i, j) = (position.0 as i32 + moves.0, position.1 as i32 + moves.1);
        if !advent_util::is_in_bound((i, j), map.len() as i32) {
            panic!("out of map");
        }
        let (i, j) = (i as usize, j as usize);

        let next = map[i][j];
        if next != '#' {
            if next == '.' {
                // on bouge le joueur
                map[i][j] = '@';
                map[position.0][position.1] = '.';
                position = (i, j)
            } else {
                // on croise une box 'O'
                if let Some(next_empty) = find_next_empty((i, j), moves, &map) {
                    // on bouge le joueur à coté, et le premier mur aux prochain espace libre
                    map[i][j] = '@';
                    map[position.0][position.1] = '.';
                    map[next_empty.0][next_empty.1] = 'O';
                    position = (i, j)
                }
                // en cas de '#' on ne fait rien
            }
        }
    });

    advent_util::print_matrice(&map);
    let mut res = 0;
    map.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, charr)| {
            if *charr == 'O' {
                res += i * 100 + j;
            }
        });
    });
    println!("{:?}", res)
}
fn find_next_empty(pos: (usize, usize), moves: &(i32, i32), map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut next = (pos.0 as i32 + moves.0, pos.1 as i32 + moves.1);
    while map[next.0 as usize][next.1 as usize] == 'O' {
        next = (next.0 + moves.0, next.1 + moves.1);
    }
    if map[next.0 as usize][next.1 as usize] == '.' {
        return Some((next.0 as usize, next.1 as usize));
    }
    // next is '#'
    None
}
fn parse_input(input: &str) -> MapMove {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut position = (0, 0);
    let map = map
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, charr)| {
                    if charr == '@' {
                        position = (i, j)
                    }
                    charr
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

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
