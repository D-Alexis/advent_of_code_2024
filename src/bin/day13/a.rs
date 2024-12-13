
pub fn solve(input: &str) {
    let mut data = parse_input(input);
    
    println!("{:?}", data.iter().fold(0, |acc, claw| acc + find(claw)))
}

fn find(claw: &Claw) -> i64 {
    let max = claw.price.0/claw.button_b.0;
    // println!("{:?} {}", claw, max);
    let mut res = i64::MAX;

   let j = (claw.price.1 * claw.button_a.0 - claw.price.0 * claw.button_a.1) as f64 / (claw.button_a.0 * claw.button_b.1 - claw.button_b.0 * claw.button_a.1) as f64;
     if j.floor() - j == 0.0 {
         let i = (claw.price.0 as f64 - j * claw.button_b.0 as f64) / claw.button_a.0 as f64;
         if i.floor() - i == 0.0 {
             res = j as i64 + 3 * i as i64;
         }
     }
     if res == i64::MAX {
         res = 0;
     }
     //println!("{:?}", res);
     res
}

fn parse_input(input: &str) -> Vec<Claw>{
    input.split("\n\n").map(|claw| {
        let list = claw.lines().map(|line| {
            let (_, data) = line.split_once(": ").unwrap();
            let (mut x,mut y) = data.split_once(", ").unwrap();
            (x,y) = (&x[2..], &y[2..]);
            (x.parse::<i64>().unwrap(),y.parse::<i64>().unwrap())
        }).collect::<Vec<(i64, i64)>>();
        Claw {
            button_a : (list[0].0, list[0].1),
            button_b: (list[1].0, list[1].1),
            price: (list[2].0  , list[2].1 )
        }
    }).collect::<Vec<Claw>>()
}
#[derive(Debug, Clone)]
struct Claw {
    pub price: (i64,i64),
    pub button_a: (i64,i64),
    pub button_b: (i64,i64)
}
