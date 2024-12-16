use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn solve(input: &str) {
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
    println!("{:?}", map[end.0 as usize][end.1 as usize])
}

fn parse_input(input: &str) -> (Vec<Vec<(char, u64)>>, (usize, usize), (usize, usize)) {
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
                    } else if charr == 'E' {
                        end = (i, j);
                    }
                    (charr, u64::MAX)
                })
                .collect::<Vec<(char, u64)>>()
        })
        .collect::<Vec<Vec<(char, u64)>>>();
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
