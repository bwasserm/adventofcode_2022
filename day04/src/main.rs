#![feature(iter_array_chunks)]

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let lines = fs::read_to_string(path).unwrap();

    let mut contains: u64 = 0;
    let mut overlaps: u64 = 0;
    for line in lines.split_terminator("\r\n") {
        let mut elves = line.split(",");
        let elf1 = elves.next().unwrap();
        let mut elf1_sects = elf1.split("-");
        // println!("{:?}", elf1_sects);
        let elf1_min: u64 = elf1_sects.next().unwrap().parse().unwrap();
        let elf1_max: u64 = elf1_sects.next().unwrap().parse().unwrap();
        
        let elf2 = elves.next().unwrap();
        // println!("{elf2}");
        let mut elf2_sects = elf2.split("-");
        let elf2_min: u64 = elf2_sects.next().unwrap().parse().unwrap();
        let elf2_max: u64 = elf2_sects.next().unwrap().parse().unwrap();

        if elf1_min <= elf2_min && elf2_max <= elf1_max || 
            elf2_min <= elf1_min && elf1_max <= elf2_max {
                contains += 1;
        }

        if elf1_min <= elf2_max && elf2_min <= elf1_max ||
            elf2_min <= elf1_max && elf1_min <= elf2_max {
                overlaps += 1;
            }

    }
    println!("Part 1: {contains}");
    println!("Part 2: {overlaps}");
}
