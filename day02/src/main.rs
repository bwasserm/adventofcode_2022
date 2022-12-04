use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    // let cols = &args[2].parse::<i32>().unwrap().clone();
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut score1 = 0;
    let mut score2 = 0;
    for line in lines {
        if let Ok(linestr) = line {
            let actions: Vec<&str> = linestr.split(" ").collect();
            let rps = (actions[0], actions[1]);
            score1 += match rps {
                ("A", "X") => 3 + 1,  // rock rock
                ("A", "Y") => 6 + 2,  // rock paper
                ("A", "Z") => 0 + 3,  // rock scissors
                ("B", "X") => 0 + 1,  // paper rock
                ("B", "Y") => 3 + 2,  // paper paper
                ("B", "Z") => 6 + 3,  // paper scissors
                ("C", "X") => 6 + 1,  // scissors rock
                ("C", "Y") => 0 + 2,  // scissors paper
                ("C", "Z") => 3 + 3,  // scissors scissors
                _ => 0,
            };
            score2 += match rps {
                ("A", "Y") => 3 + 1,  // rock rock
                ("A", "Z") => 6 + 2,  // rock paper
                ("A", "X") => 0 + 3,  // rock scissors
                ("B", "X") => 0 + 1,  // paper rock
                ("B", "Y") => 3 + 2,  // paper paper
                ("B", "Z") => 6 + 3,  // paper scissors
                ("C", "Z") => 6 + 1,  // scissors rock
                ("C", "X") => 0 + 2,  // scissors paper
                ("C", "Y") => 3 + 3,  // scissors scissors
                _ => 0,
            };
        }
    }

    println!("Score 1: {:?}", score1);
    println!("Score 2: {:?}", score2);

}
