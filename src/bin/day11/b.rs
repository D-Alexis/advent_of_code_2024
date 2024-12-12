use std::collections::HashMap;

pub fn solve(input: &str) {
    let mut value_to_nb: HashMap<String, u64> = HashMap::with_capacity(2000);
    input.split_whitespace().map(|value| value.to_string()).for_each(|val| {
        *value_to_nb.entry(val).or_default() += 1;
    });

    for _ in 0..75 {
        let mut next: HashMap<String, u64> = HashMap::with_capacity(2000);
        for (val, number) in value_to_nb.iter() {
            if val == "0" {
                *next.entry("1".to_string()).or_default() += number;
            } else if val.len() % 2 == 0 {
                let (left, right) = val.split_at(val.len() / 2);
                let right = format!("{}", right.parse::<u64>().unwrap());
                *next.entry(left.to_string()).or_default() += number;
                *next.entry(right).or_default() += number;
            } else {
                let key = format!("{}", val.parse::<u64>().unwrap() * 2024);
                *next.entry(key).or_default() += number;
            }
        }
        value_to_nb = next;
    }
    println!("{:?}", value_to_nb.iter().fold(0, |acc, (_, value)| acc + value));
    println!("{:?}", value_to_nb.len());
}
