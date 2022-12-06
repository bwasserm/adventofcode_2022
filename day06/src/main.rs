#![feature(iter_array_chunks)]

use std::fs;
use std::env;

// use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = fs::read_to_string(path).unwrap();
    
    let len_input = input.len();

    for idx in 14..len_input {
        let slice: &Vec<char> = &input[(idx-14)..idx].chars().collect();
        let mut slice = slice.clone();
        slice.sort();
        let mut deduped = slice.clone();
        deduped.dedup();
        if slice == deduped {
            println!("Part 2: {idx}: {slice:?} {deduped:?}");
            break;
        }
    }
}