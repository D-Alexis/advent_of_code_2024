pub fn solve(input: &str) {
    let mut all_numbers = parse_input(input);
    let mut output: Vec<i32> = Vec::with_capacity(all_numbers.len() * 2);
    let mut j = all_numbers.len() - 1;
    for i in 0..all_numbers.len() {
        if i % 2 == 0 {
            for _ in 0..all_numbers[i] {
                output.push(i as i32 / 2);
            }
            all_numbers[i] = 0;
        } else {
            for _ in 0..all_numbers[i] {
                output.push(j as i32 / 2);
                all_numbers[j] -= 1;
                if all_numbers[j] == 0 {
                    j -= 2;
                    if i >= j {
                        break;
                    }
                }
            }
        }
        if i >= j {
            break;
        }
    }
    println!(
        "{:?}",
        output
            .iter()
            .enumerate()
            .fold(0, |acc, (i, num)| acc + i * *num as usize)
    );
}
fn parse_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|charr| charr.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}
