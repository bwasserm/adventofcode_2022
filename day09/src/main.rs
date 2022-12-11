use std::fs;
use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let input = fs::read_to_string(path).unwrap();
    
    let mut visited1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited9: HashSet<(i32, i32)> = HashSet::new();
    visited1.insert((0, 0));
    visited9.insert((0, 0));
    
    let mut rope = vec![(0, 0); 10];
    
    for line in input.split("\r\n") {
        let mut parts = line.split_whitespace();
        let dir = parts.next().unwrap();
        let steps: usize = parts.next().unwrap().parse().unwrap();
        
        for _step in 0..steps {
            match dir {
                "L" => {  // -X
                    rope[0].0 -= 1;
                },
                "R" => {  // +X
                    rope[0].0 += 1;
                },
                "D" => {  // -Y
                    rope[0].1 -= 1;
                },
                "U" => {  // +Y
                    rope[0].1 += 1;
                },
                _ => {},
            }
            for knot in 1..rope.len() {
                let Hx: i32 = (&rope[knot - 1].0).clone();
                let Hy: i32 = (&rope[knot - 1].1).clone();
                let Tx: i32 = (&rope[knot].0).clone();
                let Ty: i32 = (&rope[knot].1).clone();
                rope[knot] = update_follower(Hx, Hy, Tx, Ty);
                // print!("{knot}");
                // println!("{dir} {steps}  H: {Hx} {Hy}\tT{knot}: {Tx} {Ty}");
            }
            
            visited1.insert(rope[1]);
            visited9.insert(rope[9]);
        }
    }
    println!("Part 1: {}", visited1.len());
    println!("Part 2: {}", visited9.len());
}

fn update_follower(Hx: i32, Hy: i32, Tx: i32, Ty: i32) -> (i32, i32) {
    // let Hx = Hx.clone();
    // let Hy = Hy.clone();
    let mut Tx = Tx.clone();
    let mut Ty = Ty.clone();
    if Hx == Tx {
        if Ty + 1 < Hy {
            Ty += 1;
        }
        if Hy + 1 < Ty {
            Ty -= 1;
        }
    } else if Hy == Ty {
        if Tx + 1 < Hx {
            Tx += 1;
        }
        if Hx + 1 < Tx {
            Tx -= 1;
        }
    } else if Hx != Tx && Hy != Ty {
        if Ty + 1 < Hy {
            Ty += 1;
            if Hx > Tx {
                Tx += 1;
            } else {
                Tx -= 1;
            }
        } else if Hy + 1 < Ty {
            Ty -= 1;
            if Hx > Tx {
                Tx += 1;
            } else {
                Tx -= 1;
            }
        } else if Hx + 1 < Tx {
            Tx -= 1;
            if Hy > Ty {
                Ty += 1;
            } else {
                Ty -= 1;
            }
        } else if Tx + 1 < Hx {
            Tx += 1;
            if Hy > Ty {
                Ty += 1;
            } else {
                Ty -= 1;
            }
        }
    }
    (Tx, Ty)
}