use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn solve(input: &str) {
    
    
    let max = 3450;//3450 ou 25
    
    for i in 1024 .. max {
        if get_min(input, i) == u64::MAX {
            println!("{i}");
            break;
        }
    }
}
fn get_min(input: &str, iter: usize) -> u64 {
    let mut map = parse_input(input, iter);
    let mut heap: BinaryHeap<ToProcess> = BinaryHeap::new();
    heap.push(ToProcess {
        i: 0 as i32,
        j: 0 as i32,
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
                let poids = node.poids + 1;
                if next_val.1 > poids {
                    map[next.0 as usize][next.1 as usize].1 = poids;
                        heap.push(ToProcess {
                            i: next.0 as i32,
                            j: next.1 as i32,
                            poids,
                            direction: dir,
                        });
                }
        }
    }
    map[70][70].1
}

fn parse_input(input: &str, length: usize) -> Vec<Vec<(char, u64)>> {
    let mut map = vec![vec![('.', u64::MAX);71];71];
    input.lines().take(length).for_each(|line| {
        let (i,j) = line.split_once(",").map(|(i,j)| (i.parse::<usize>().unwrap(), j.parse::<usize>().unwrap())).unwrap();
        map[j][i] = ('#', u64::MAX); // inversé dans l'ennoncé
    });
    let a = map.len()-1;
    map[a][a].0 = 'E';
    map
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
