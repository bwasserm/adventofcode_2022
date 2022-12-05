#![feature(iter_array_chunks)]

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let lines = fs::read_to_string(path).unwrap();
    let num_stacks: usize = args[2].parse().unwrap();
    
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0_usize..num_stacks {
        stacks.push(Vec::new());
    }

    let mut initialized = false;

    for line in lines.split_terminator("\r\n") {
        // End of initialization
        if line == "" {
            println!("Initial crates stacked!");
            for (_stacknum, stack) in stacks.iter_mut().enumerate() {
                stack.pop();
                stack.reverse();
                // println!("{}: {:?}", _stacknum + 1, stack);
            }
            print_stacks(&stacks);
            initialized = true;
            continue;
        }
        if !initialized {
            for (stacknum, col) in (1_usize..(4 * num_stacks + 1)).step_by(4).enumerate() {
                let krate = line.chars().nth(col).unwrap();
                if krate != ' ' {
                    stacks[stacknum].push(krate);
                }
            }
        } else {
            let cmd_args: Vec<&str> = line.split(" ").collect();
            // println!("{cmd_args:?}");
            let num_to_move: u32 = cmd_args[1].parse().unwrap();
            let src_stack: usize = cmd_args[3].parse::<usize>().unwrap() - 1_usize;
            let dst_stack: usize = cmd_args[5].parse::<usize>().unwrap() - 1_usize;


            let mut crane = vec![];
            for _ in 0..num_to_move {
                let krate = stacks[src_stack].pop().unwrap();
                crane.push(krate);
            }
            for _ in 0..num_to_move {
                let krate = crane.pop().unwrap();
                stacks[dst_stack].push(krate);
            }
        }
    }
    println!("Final configuration:");
    print_stacks(&stacks);
    print!("Part 2: ");
    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
}

fn print_stacks(stacks: &Vec<Vec<char>>) {
    for (stacknum, stack) in stacks.iter().enumerate() {
        println!("{}: {:?}", stacknum + 1, stack);
    }
}