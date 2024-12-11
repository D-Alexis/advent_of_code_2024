use std::collections::HashMap;

pub fn solve(input: &str) {
    let mut value_to_nb : HashMap<String, u64> = HashMap::with_capacity(2000);
   input.split_whitespace().map(|value| value.to_string()).for_each(|val| {
       *value_to_nb.entry(val).or_default() += 1;
   }); 
   
   for _ in 0..25 {
       let mut next : HashMap<String, u64> = HashMap::with_capacity(2000);
       for (val, number) in value_to_nb.iter() {
           if val == "0" {
               *next.entry("1".to_string()).or_default() += number;
           } else if val.len() % 2 == 0 {
               let (left, right) = val.split_at(val.len()/2);
               let right = format!("{}",right.parse::<u64>().unwrap());
               *next.entry(left.to_string()).or_default() += number;
               *next.entry(right).or_default() += number;
           } else {
               let key = format!("{}",val.parse::<u64>().unwrap() * 2024);
               *next.entry(key).or_default() += number;
           }
       }
       value_to_nb = next;
   }
   println!("{:?}",value_to_nb.iter().fold(0, |acc, (_,value)| acc + value));
}


/////////////////////////////////
////
////
//// First naive implementation
//// The idea was : "Well we don't know what part2 will be, we might need to keep the ordering, it seems to be important in the instructions"
////  => It was, in fact, a bait, the order is pointless
////
////
////////////////////////////////
/* POUBELLE
pub fn solve(input: &str) {
   let mut list = input.split_whitespace().map(|value| value.to_string()).collect::<Vec<String>>(); 
   
   
   for _ in 0 .. 25 {
       let mut treated = 0;
       let len = list.len();
       let mut i = 0;
       while treated < len {
           let val = &list[i];
           if val == "0" {
               list[i] = String::from("1");
               i += 1;
           } else if val.len() % 2 == 0 {
               let (left, right) = val.split_at(val.len()/2);
               let to_insert = format!("{}",right.parse::<u64>().unwrap());
               list[i] = left.to_string();
               list.insert(i + 1 , to_insert);
               i += 2;
           } else {
               list[i] = format!("{}",val.parse::<u64>().unwrap() * 2024);
               i += 1;
           }
           treated += 1;
       }
   }
   println!("{:?}", list.len());
}
*/