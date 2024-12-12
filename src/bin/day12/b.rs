use std::collections::HashSet;
const ANGLES: [((i32, i32), (i32, i32)); 4] = [((0, -1), (-1, 0)), ((-1, 0), (0, 1)), ((0, 1), (1, 0)), ((1, 0), (0, -1))];
pub fn solve(input: &str) {
    let mut data = parse_input(input);
    let mut sum = 0;
    for i in 0..data.len() {
        for j in 0..data.len() {
            if !data[i][j].1 {
                // pas encore parcouru
                sum += explore_area((i as i32, j as i32), &mut data);
                data[i][j].1 = true
            }
        }
    }

    println!("{:?}", sum);
}

fn explore_area(position: (i32, i32), map: &mut Vec<Vec<(char, bool)>>) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::with_capacity(100);
    let mut next: Vec<(i32, i32)> = Vec::with_capacity(100);
    next.push(position);
    let mut resultat = (0, 0);
    explore(next.pop().unwrap(), map, &mut next, &mut visited, &mut resultat);
   // println!("{:?}", resultat);
    resultat.0 * resultat.1
}

fn explore(
    position: (i32, i32),
    map: &mut Vec<Vec<(char, bool)>>,
    next: &mut Vec<(i32, i32)>,
    visited: &mut HashSet<(i32, i32)>,
    resultat: &mut (i32, i32),
) {
    resultat.0 += 1; //area
    for dir in advent_util::DIRECTION {
        let next_pos = (position.0 + dir.0, position.1 + dir.1);
        if advent_util::is_in_bound(next_pos, map.len() as i32) {
            if map[next_pos.0 as usize][next_pos.1 as usize].0 == map[position.0 as usize][position.1 as usize].0 {
                if !visited.contains(&next_pos)
                    && !next.contains(&next_pos)
                    && map[position.0 as usize][position.1 as usize].1 == false
                {
                    next.push(next_pos);
                }
            }
        }
    }
    //
    resultat.1 += check_angle(position, map);
    visited.insert(position);
    map[position.0 as usize][position.1 as usize].1 = true;
    while next.len() > 0 {
        explore(next.pop().unwrap(), map, next, visited, resultat);
    }
}

fn check_angle(position: (i32, i32),map: &mut Vec<Vec<(char, bool)>>,) -> i32 {
    let mut res = 0;
    let val = map[position.0 as usize][position.1 as usize].0;
    for (dir_first, dir_second) in ANGLES {
        let (first_point, second_point) = ((position.0 + dir_first.0, position.1 + dir_first.1), (position.0 + dir_second.0, position.1+ dir_second.1));
        let (is_first_in, is_second_in) = (advent_util::is_in_bound(first_point, map.len() as i32), advent_util::is_in_bound(second_point, map.len() as i32));
        if is_first_in && is_second_in {
            if map[first_point.0 as usize][first_point.1 as usize].0 == val 
                && map[second_point.0 as usize][second_point.1 as usize].0 == val {
                    // check for 'interior' angle
                    let dir = (dir_first.0 + dir_second.0, dir_first.1 + dir_second.1);
                    if val != map[(position.0 + dir.0) as usize][(position.1 + dir.1) as usize].0 {
                        res += 1;
                    }
            } else if map[first_point.0 as usize][first_point.1 as usize].0 != val 
                && map[second_point.0 as usize][second_point.1 as usize].0 != val {
                // is an 'external' angle
                res += 1;
            }
        } else if !is_first_in && !is_second_in {
            res += 1;
        }  else if is_first_in && !is_second_in && val != map[first_point.0 as usize][first_point.1 as usize].0 {
            res += 1;
        }  else if !is_first_in && is_second_in && val != map[second_point.0 as usize][second_point.1 as usize].0 {
            res += 1;
        }
    }
    
    res
}

fn parse_input(input: &str) -> Vec<Vec<(char, bool)>> {
    input
        .lines()
        .map(|line| line.chars().map(|charr| (charr, false)).collect::<Vec<(char, bool)>>())
        .collect::<Vec<Vec<(char, bool)>>>()
}
