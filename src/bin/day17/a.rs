use std::ops::BitXor;

pub fn solve(input: &str) {
    let (mut registers, instructions) = parse_input(input);
    println!("{:?} and {:?}", registers, instructions);
    
    let mut pointer = 0;
    let mut output = String::new();
    while pointer < instructions.len() {
        if let Some(out) = exec_inst(&mut pointer, &mut registers, &instructions) {
            if output.len() == 0 {
                output.push_str(&format!("{}", out));
            } else {
                output.push_str(&format!(",{}", out));
            }
        }
        //println!("registers : {:?} ", registers);
    }
    println!("out put : {:?} ", output);
 //   println!("registers : {:?} ", registers);
}
fn exec_inst(pointer: &mut usize, registers : &mut Vec<u64>, instructions: &Vec<u64>) -> Option<u64> {
    let (opcod, operand) = (instructions[*pointer], instructions[*pointer +1 ]);
    //println!("{:?} and {:?}", opcod, operand);
    
    match opcod {
        0 => {
            let denom = 2u64.pow(get_combo(operand, registers)as u32);
            let val = registers[0] / denom;
            registers[0] = val;
        },
        1 => {
            registers[1] = registers[1] ^ operand;
            
        },
        2 => {
            registers[1] = get_combo(operand, registers) % 8;
        },
        3 => {
            if registers[0] != 0 {
                *pointer = 0;
                return None;
            }
        },
        4 => {
            registers[1] = registers[1] ^ registers[2];
        },
        5 => {
            *pointer += 2;
            return Some(get_combo(operand, registers) % 8);
        }
        6 => {
            let denom = 2u64.pow(get_combo(operand, registers) as u32);
            let val = registers[0] / denom;
            registers[1] = val;
        },
        7 => {
            let denom = 2u64.pow(get_combo(operand, registers) as u32);
            let val = registers[0] / denom;
            registers[2] = val;
        }
        _ => panic!("invalid instructions")
    }
    
    *pointer += 2;
    None
}

fn get_combo(operand: u64, registers : &mut Vec<u64>) -> u64 {
    let res = match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("invalid operand")
    };
    res
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>){
    let (register, program) = input.split_once("\n\n").unwrap();
    let registers = register.lines().enumerate().map(|(i, line)| {
        line.split_once(": ").unwrap().1.parse::<u64>().unwrap()
    }).collect::<Vec<u64>>();
    
    let instructions = program.split_once(": ").unwrap().1.split(",").map(|charr| charr.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    (registers, instructions)
}