pub fn is_in_bound((i, j): (i32, i32), max: i32) -> bool {
    i >= 0 && i < max && j >= 0 && j < max
}
