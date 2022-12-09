#![feature(is_sorted)]

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = fs::read_to_string(path).unwrap();

    let size = input.split_whitespace().peekable().next().unwrap().len();
    let mut trees: Vec<Vec<u32>> = vec![];
    // Build matrix of forest
    for line in input.lines() {
        let line_trees: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        trees.push(line_trees);
    }

    let mut visible = 0;
    let mut max_scenic_score = 0;
    // println!("{:?}", trees);
    for tree_row in 0..size {
        for tree_col in 0..size {
            let mut is_visible = false;
            let scenic_score;
            if tree_row == 0 || tree_row == size - 1 {
                visible += 1;
                continue;
            }
            if tree_col == 0 || tree_col == size - 1 {
                visible += 1;
                continue;
            }
            let tree = trees[tree_row][tree_col];
            // print!("Checking {tree_row},{tree_col}: {tree} ");
            // Check left
            let mut los: Vec<u32> = vec![];
            let mut sleft = 0;
            for (dst, col) in (0..tree_col).rev().enumerate() {
                let tmp_tree = trees[tree_row][col];
                los.push(tmp_tree);
                if tmp_tree >= tree && sleft == 0 {
                    sleft = dst + 1;
                }
            }
            if &tree > los.iter().max().unwrap() && !is_visible {
                visible += 1;
                is_visible = true;
                // println!("visible left");
                // continue;
            }
            if sleft == 0 {
                sleft = tree_col;
            }
            // Check right
            let mut los: Vec<u32> = vec![];
            let mut sright = 0;
            for (dst, col) in ((tree_col + 1)..size).enumerate() {
                let tmp_tree = trees[tree_row][col];
                los.push(tmp_tree);
                if tmp_tree >= tree && sright == 0 {
                    sright = dst + 1;
                }
            }
            if &tree > los.iter().max().unwrap() && !is_visible {
                visible += 1;
                is_visible = true;
                // println!("visible right");
                // continue;
            }
            if sright == 0 {
                sright = size - tree_col - 1;
            }
            // Check up
            let mut los: Vec<u32> = vec![];
            let mut sup = 0;
            for (dst, row) in (0..tree_row).rev().enumerate() {
                let tmp_tree = trees[row][tree_col];
                los.push(tmp_tree);
                if tmp_tree >= tree && sup == 0 {
                    sup = dst + 1;
                }
            }
            if &tree > los.iter().max().unwrap() && !is_visible {
                visible += 1;
                is_visible = true;
                // println!("visible up");
                // continue;
            }
            if sup == 0 {
                sup = tree_row;
            }
            // Check down
            let mut los: Vec<u32> = vec![];
            let mut sdown = 0;
            for (dst, row) in ((tree_row + 1)..size).enumerate() {
                let tmp_tree = trees[row][tree_col];
                los.push(tmp_tree);
                if tmp_tree >= tree && sdown == 0 {
                    sdown = dst + 1;
                }
            }
            if &tree > los.iter().max().unwrap() && !is_visible {
                visible += 1;
                // println!("visible down");
                // continue;
            }
            if sdown == 0 {
                sdown = size - tree_row - 1;
            }
            // println!("not visible!");

            // println!("{sleft} {sright} {sup} {sdown}");
            scenic_score = sleft * sright * sup * sdown;
            max_scenic_score = max_scenic_score.max(scenic_score);
        }
    }
    println!("Part 1: {visible}");
    println!("Part 2: {max_scenic_score}");
}
