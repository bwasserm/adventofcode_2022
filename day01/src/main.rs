use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // let cols = &args[2].parse::<i32>().unwrap().clone();
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut elves = Vec::<u32>::new();
    elves.push(0);

    for line in lines {
        if let Ok(num) = line {
            let val = num.parse::<u32>();
            match val {
                Ok(intval) => {
                    let tmp = elves.pop().unwrap();
                    elves.push(tmp + intval);
                },
                Err(_) => elves.push(0),
            }
        }
    }

    println!("Max: {:?}", elves.iter().max().unwrap());

    elves.sort();
    println!("Max of top three: {:?}", elves.iter().rev().take(3).sum::<u32>());
}
