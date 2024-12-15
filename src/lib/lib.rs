pub fn is_in_bound((i, j): (i32, i32), max: i32) -> bool {
    i >= 0 && i < max && j >= 0 && j < max
}

pub const DIRECTION: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

use std::fmt::Display;
pub fn print_matrice<T>(input: &Vec<Vec<T>>) where T: Display{
    input.iter().for_each(|line| {
        line.iter().for_each(|elem| {
            print!("{}", elem)
        });
        println!("");
    });
}