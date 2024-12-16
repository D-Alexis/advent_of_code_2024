use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

pub fn solve(input: &str) {
    let (mut map, start, end) = solve_p1(input);

    let mut in_path: HashSet<(i32, i32)> = HashSet::with_capacity(2000);
    explore(&mut in_path, &mut map, (start.0 as i32, start.1 as i32), end);

    println!("{:?}", in_path.len());
    // println!("{:?}", map[1][139]);
    // println!("{:?}", map[1][138]);
    // println!("{:?}", map[2][139]);
}
fn explore(
    in_path: &mut HashSet<(i32, i32)>,
    map: &mut Vec<Vec<(char, u64, (i32, i32))>>,
    actual: (i32, i32),
    end: (usize, usize),
) -> bool {
    let mut to_process: Vec<(i32, i32)> = Vec::with_capacity(4);
    let mut is_ok = false;
    let node_poid = map[actual.0 as usize][actual.1 as usize].1;
    for dir in advent_util::DIRECTION {
        let next = (actual.0 + dir.0, actual.1 + dir.1);
        let next_poids = map[next.0 as usize][next.1 as usize].1;
        if map[next.0 as usize][next.1 as usize].0 == 'E' {
            if next_poids == node_poid + 1 || next_poids == node_poid + 1001 {
                is_ok = true;
                in_path.insert(next);
                continue;
            }
        }

        if ((next_poids == node_poid + 1) && dir == map[next.0 as usize][next.1 as usize].2)
            || ((next_poids == node_poid + 1001) && dir == map[next.0 as usize][next.1 as usize].2) {
           
            is_ok |= explore(in_path, map, next, end);
        }
        if next_poids + 1000 == node_poid + 1 {
            if map[(next.0 + dir.0) as usize][(next.1 + dir.1) as usize].1 == node_poid + 2 {
                if dir != map[next.0 as usize][next.1 as usize].2 {
                    if map[(next.0 + dir.0) as usize][(next.1 + dir.1) as usize].1 == node_poid + 2 {
                    is_ok |= explore(in_path, map, next, end);
                    }
                    
                }
                
            }
        }
    }

    if is_ok {
        in_path.insert((actual.0 as i32, actual.1 as i32));
    }

    is_ok
}

pub fn solve_p1(input: &str) -> (Vec<Vec<(char, u64, (i32, i32))>>, (usize, usize), (usize, usize)) {
    let (mut map, start, end) = parse_input(input);
    //  advent_util::print_matrice(&map);
    let mut heap: BinaryHeap<ToProcess> = BinaryHeap::new();
    heap.push(ToProcess {
        i: start.0 as i32,
        j: start.1 as i32,
        poids: 0,
        direction: (0, 1),
    });

    while heap.len() != 0 {
        let node = heap.pop().unwrap();
        //    println!("{:?}", node);
        //check around
        for dir in advent_util::DIRECTION {
            let next = (node.i + dir.0, node.j + dir.1);
            if !advent_util::is_in_bound(next, map.len() as i32) || map[next.0 as usize][next.1 as usize].0 == '#' {
                continue;
            }
            let next_val = map[next.0 as usize][next.1 as usize];
            if dir == node.direction {
                let poids = node.poids + 1;
                if next_val.1 > poids {
                    map[next.0 as usize][next.1 as usize].1 = poids;
                    map[next.0 as usize][next.1 as usize].2 = dir;
                    if next_val.0 != 'E' {
                        heap.push(ToProcess {
                            i: next.0 as i32,
                            j: next.1 as i32,
                            poids,
                            direction: dir,
                        });
                    }
                }
            } else if dir.0 != node.direction.0 && dir.1 != node.direction.1 {
                // on tourne à 90°
                let poids = node.poids + 1001;
                if next_val.1 > poids {
                    map[next.0 as usize][next.1 as usize].1 = poids;
                    map[next.0 as usize][next.1 as usize].2 = dir;
                    if next_val.0 != 'E' {
                        heap.push(ToProcess {
                            i: next.0 as i32,
                            j: next.1 as i32,
                            poids,
                            direction: dir,
                        });
                    }
                }
            } // else rien, on fait pas demi tour
        }
    }
    (map, start, end)
}

fn parse_input(input: &str) -> (Vec<Vec<(char, u64, (i32, i32))>>, (usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, charr)| {
                    if charr == 'S' {
                        start = (i, j);
                        return (charr, 0, (0, 0));
                    } else if charr == 'E' {
                        end = (i, j);
                    }
                    (charr, u64::MAX - 1, (0, 0))
                })
                .collect::<Vec<(char, u64, (i32, i32))>>()
        })
        .collect::<Vec<Vec<(char, u64, (i32, i32))>>>();
    (map, start, end)
}

#[derive(Debug)]
struct ToProcess {
    i: i32,
    j: i32,
    poids: u64,
    direction: (i32, i32),
}

impl Eq for ToProcess {}

impl PartialEq<Self> for ToProcess {
    fn eq(&self, other: &Self) -> bool {
        self.poids == other.poids
    }
}

impl PartialOrd<Self> for ToProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ToProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        other.poids.cmp(&self.poids)
    }
}
