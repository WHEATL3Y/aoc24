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
    let mut id = 0;
    let mut in_file = true;
    let mut fs_master: Vec<isize> = Vec::new();
    for v in data {
        let value_to_push: isize;
        if !in_file {
            value_to_push = -1;
        }
        else {
           value_to_push = id;
           id += 1;
        }
        let mut i = 0;
        while i < v {
            fs_master.push(value_to_push);
            i += 1;
        }
        in_file = !in_file;
    }

    // Copy and compact file system
    let mut fs = fs_master.to_vec();
    let mut last_digit = fs.len() - 1;
    let mut pos = 0;
    loop {
        while fs[last_digit] == -1 {
            last_digit -= 1;
        }
        while !(fs[pos] == -1) {
            pos += 1;
        }
        if last_digit <= pos {
            break;
        }
        fs[pos] = fs[last_digit];
        fs[last_digit] = -1;
    }

    // Calculate checksum
    let mut i = 0;
    while fs[i] != -1 {
        answer1 += i * fs[i] as usize;
        i += 1;
    }

    // Copy and defrag
    fs = fs_master.to_vec();
    let mut last_block = [fs.len() - 2, fs.len() - 2];
    let mut next_empty = [0, 1];
    let mut last_digit = fs.len() -1;
    let mut pos = 0;
    loop {
        // Find bounds of file, seaching from the end of fs
        while fs[last_digit] == -1 {
            last_digit -= 1;
        }
        last_block[1] = last_digit;
        while fs[last_digit] == fs[last_block[1]] && last_digit > 0 {
            last_digit -= 1
        }
        last_block[0] = last_digit;
        if last_digit == 0 {
            break;
        }
    }
    
    // Print answers
    println!("{}", answer1);
}
