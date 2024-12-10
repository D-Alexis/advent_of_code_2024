pub fn solve(input: &str) {
    let mut all_numbers = parse_input(input);
    let mut output: Vec<usize> = Vec::with_capacity(all_numbers.len() * 2);
    let mut max_j = all_numbers.len() - 1;
    let mut free_space_index: Vec<(usize, usize)> = Vec::with_capacity(all_numbers.len() / 2);
    let mut number_index: Vec<(usize, usize)> = Vec::with_capacity(all_numbers.len() / 2);
    ///////////
    //
    // Init
    //
    ///////////
    for i in 0..all_numbers.len() {
        if i % 2 == 0 {
            number_index.push((i / 2, output.len()));
            for _ in 0..all_numbers[i] {
                output.push(i / 2);
            }
        } else {
            if all_numbers[i] != 0 {
                free_space_index.push((all_numbers[i], output.len()));
                for _ in 0..all_numbers[i] {
                    output.push(0);
                }
            }
        }
    }
    for (j, num_index) in number_index.iter().rev() {
        let space_needed = all_numbers[(*j) * 2];
        if let Some((space, index)) = free_space_index.iter_mut().find(|(space, _)| *space >= space_needed) {
            if *index < *num_index {
                for n in 0..space_needed {
                    output[*index + n] = *j;
                    output[*num_index + n] = 0;
                }
                *space -= space_needed;
                *index += space_needed;
            }
        }
    }

    // println!("{:?}", output);
    println!(
        "{:?}",
        output
            .iter()
            .enumerate()
            .fold(0, |acc, (i, num)| acc + i * *num as usize)
    );
}
fn parse_input(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|charr| charr.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
}
