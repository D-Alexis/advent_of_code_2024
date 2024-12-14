use std::collections::HashMap;


//const WIDTH : i32 = 11;
//const HEIGHT : i32 = 7;
const WIDTH : i32 = 101;
const HEIGHT : i32 = 103;
pub fn solve(input: &str) {
    let robots = parse_input(input);
    let mut max = 0;
    let mut max_n = 0;
    for n in 0..10000 {
        let mut quandrant_nb : Vec<i32> = vec![0;4];
        robots.iter().for_each(|robot| {
            compute_position(robot,n, &mut quandrant_nb);
        });
        let max_iter = quandrant_nb.iter().max().unwrap();
        if *max_iter > max { // On devrait avoir une plus grand concentration de robot à un endroit lorsqu'ils font l'image. Sauf si c'est parfait centrer pour nous emmerder
            max = *max_iter;
            max_n = n;
        }
    }
    println!("{} {}", max, max_n)
}
fn compute_position(robot: &((i32,i32),(i32,i32)), n: i32, pos_to_nb : &mut Vec<i32>) {
    let (pos, vel) = robot;
    let pos_x = (pos.0 + vel.0 * n).rem_euclid(WIDTH);
    let pos_y = (pos.1 + vel.1 * n).rem_euclid(HEIGHT);
    if let Some(quadrant) = get_quadrant((pos_x, pos_y)) {
         pos_to_nb[quadrant] += 1;
    }
}

fn get_quadrant(pos : (i32,i32)) -> Option<usize> {
    // Quadrant will be numberred like this
    //  0 1
    //  2 3
    //
    if pos.0 < WIDTH/2 { // quandrat 0 and 2
        if pos.1 < HEIGHT/2 { //quandrant 0
            return Some(0)
        } else if pos.1 > HEIGHT/2{ //quandrant 2
            return Some(2)
        }
    } else if pos.0 > WIDTH/2{ // quandrat 1 and 3
        if pos.1 < HEIGHT/2 {
            return Some(1)
        } else if pos.1 > HEIGHT/2 {
            return Some(3)
        }
    }
    None
}
fn parse_input(input: &str) -> Vec<((i32,i32),(i32,i32))> {
    input.lines().map(|line| {
        let (mut pos, mut vel) = line.split_once(" ").unwrap();
        (pos, vel) = (&pos[2..], &vel[2..]);
        let (pos_x, pos_y) = pos.split_once(",").map(|(pos_x, pos_y)| (pos_x.parse::<i32>().unwrap(), pos_y.parse::<i32>().unwrap())).unwrap();
        let (vel_x, vel_y) = vel.split_once(",").map(|(vel_x, vel_y)| (vel_x.parse::<i32>().unwrap(), vel_y.parse::<i32>().unwrap())).unwrap();
        ((pos_x, pos_y), (vel_x, vel_y))
    }).collect::<Vec<((i32,i32),(i32,i32))>>()
}