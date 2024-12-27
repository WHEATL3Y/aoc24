use std::fs::File;
use std::io::prelude::*;

fn checksum(fs: Vec<(isize, usize)>) -> usize {
    let mut i = 0;
    let mut answer = 0;
    for v in &fs {
        for _ in 0..v.1 {
            if v.0 != -1 {
                answer += v.0 * i;
            }
            i += 1;
        }
    }
    return answer as usize;
}

fn build_fs(input: &Vec<usize>) -> Vec<(isize, usize)> {
    let mut id: isize = 0;
    let mut fs: Vec<(isize, usize)> = Vec::new();
    let mut in_file = true;
    for v in input {
        fs.push((if in_file {id} else {-1}, *v));
        if in_file {
            id += 1;
        }
        in_file = !in_file;
    }
    return fs
}

fn main() {
    let mut file = File::open("../../inputs/day9.txt")
        .expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file to string");
    let data: Vec<usize> = contents.trim().chars().map(
        |x| x.to_digit(10).expect("Error converting char to digit") as usize)
        .collect();
    let answer1: usize;
    let answer2: usize;

    // Answer 1
    let mut fs = build_fs(&data);
    let mut last = fs.len() - 1;
    let mut first = 0;
    while last > first {
        // Seek, from beginning to next free space
        while fs[first].0 != -1 || fs[first].1 == 0 {
            first += 1;
        }
        // Seek, from end to next available file
        while fs[last].0 == -1 || fs[last].1 == 0 {
            last -= 1;
        }
        fs[first].1 = fs[first].1.checked_sub(1).unwrap();
        fs[last].1 = fs[last].1.checked_sub(1).unwrap();
        fs.insert(first, (fs[last].0, 1));
        last += 1;      // Accounting for insert
    }
    answer1 = checksum(fs);

    // Answer 2
    fs = build_fs(&data);
    for i in (0..fs.len()).rev() {
        if fs[i].0 != -1 {
            for j in 0..fs.len() {
                if i <= j {
                    break;
                }
                if fs[j].0 == -1 && fs[i].1 <= fs[j].1 {
                    // (value, size)
                    let v = fs[i].0.clone();    // Block value to be moved
                    let s = fs[i].1.clone();    // Size of block to be moved
                    let es = fs[j].1.clone();   // Size of empty block
                    fs[j].1 = es - s;           // Resize empty block
                    fs.insert(j, (v, s));       // Move file to empty block
                    fs[i + 1].0 = -1;           // Set old block to empty
                                                // i + 1 accounts for insert
                    break;
                }
            }
        }
    }
    answer2 = checksum(fs);

    println!("{}\n{}", answer1, answer2);
}
