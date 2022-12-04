#![feature(iter_array_chunks)]

use std::fs;
use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let lines = fs::read_to_string(path).unwrap();

    let mut priorities: u64 = 0;
    for line in lines.split_terminator("\n") {
        // if let Ok(line) = line {
            let num_items = line.len();
            let comp1_bytes = line[0..num_items / 2].as_bytes();
            let comp2_bytes = line[num_items / 2..num_items].as_bytes();
            let comp1: HashSet<&u8> = HashSet::from_iter(comp1_bytes.iter());
            let comp2: HashSet<&u8> = HashSet::from_iter(comp2_bytes.iter());
            for i in comp1.intersection(&comp2){
                priorities += atoi(**i);
            }
        // }
    }
    println!("Part 1: {priorities}");

    priorities = 0;
    
    for group in lines.split_terminator("\n").array_chunks::<3>() {
        let sack0 = group.get(0).unwrap().as_bytes().iter();
        // println!("{:?}", sack0);
        let mut sack1 = group.get(1).unwrap().as_bytes().iter();
        // println!("{:?}", sack1);
        let mut sack2 = group.get(2).unwrap().as_bytes().iter();
        // println!("{:?}", sack2);
        let mut badge: u8 = 0;
        for char0 in sack0 {
            for char1 in sack1.by_ref().clone() {
                // println!("{char0} {char1}");
                if char0 == char1 {
                    for char2 in sack2.by_ref().clone() {
                        if char0 == char2 {
                            badge = *char0;
                            break;
                        }
                    }
                }
                if badge > 0 {
                    break;
                }
            }
            if badge > 0 {
                break;
            }
        }
        priorities += atoi(badge);
    }
    println!("Part 2: {priorities}");
}

fn atoi(b: u8) -> u64 {
    // println!("{}", b as char);
    let a: u64 = b.into();
    if 97 <= a && a <= 122 {  // a-z
        return a - 96;
    } else if 65 <= a && a <= 90 {
        return a - 65 + 27;
    } else {
        return 0;
    }
}