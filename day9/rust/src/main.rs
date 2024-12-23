use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("../../inputs/day9.test")
        .expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file to string");
    let data: Vec<usize> = contents.trim().chars().map(
        |x| x.to_digit(10).expect("Error converting char to digit") as usize)
        .collect();
    let mut answer1 = 0;
    // let mut answer2 = 0;
    //
    // Build FS from input data
    let mut id: isize = 0;
    let mut fs_master: Vec<(isize, usize)> = Vec::new();
    let mut in_file = true;
    for v in data {
        fs_master.push((if in_file {id} else {-1}, v));
        if in_file {
            id += 1;
        }
        in_file = !in_file;
    }
    
    for i in 0..fs_master.len() {
        println!("({}, {})", fs_master[i].0, fs_master[i].1); 
    }

    for i in (0..fs_master.len()).rev() {
        for i in 0..fs_master.len() {
            print!("({}, {})", fs_master[i].0, fs_master[i].1); 
        }
        println!();
        if fs_master[i].0 != -1 {
            for j in 0..fs_master.len() {
                if i <= j {
                    break;
                }
                if fs_master[j].0 == -1 && fs_master[i].1 <= fs_master[j].1 {
                    // (value, size)
                    // Value and size of blocks being moved
                    let v = fs_master[i].0.clone();
                    let s = fs_master[i].1.clone();
                    // Size of empty block
                    let es = fs_master[j].1.clone();
                    // Resize empty block
                    fs_master[j].1 = es - s;
                    // Move file to empty block
                    fs_master.insert(j, (v, s));
                    // Set old block to empty
                    fs_master[i].0 = -1;
                    break;
                }
            }
        }
    }
    // Print answers
    println!("{}", answer1);
}
