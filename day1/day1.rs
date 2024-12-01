use std::fs::File;
use std::io::prelude::*;

fn get_input(file: &str) -> String {
    let mut file = File::open(file).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read file");
    contents
}

fn count(list: &Vec<usize>, value: usize) -> usize {
    // Returns the number of times values appears in list
    let mut count = 0;
    let mut i = 0;
    while i < list.len() {
        if list[i] == value {
            count += 1;
        }
        i += 1;
    }
    count
}

fn main() {
    // Split input into lines, loop through lines and split into 
    // individual numbers, add numbers to l1, and l2.

    // let filename = "../inputs/day1.sample.txt";
    let filename = "../inputs/day1.txt";
    let mut l1: Vec<usize> = Vec::<usize>::new();
    let mut l2: Vec<usize> = Vec::<usize>::new();
    for line in get_input(filename).split("\n") {
        let num_pairs: Option<(&str, &str)> = line.split_once("   ");
        if num_pairs == None {
            break;
        }
        else {
            let nums = num_pairs.unwrap();
            l1.push(usize::from_str_radix(nums.0, 10).unwrap());
            l2.push(usize::from_str_radix(nums.1, 10).unwrap());

        }
    }

    let mut i = 0;
    let mut answer1: usize = 0;
    let mut answer2: usize = 0;
    l1.sort();
    l2.sort();
    while i < l1.len() {
        answer1 += l1[i].abs_diff(l2[i]);
        answer2 += count(&l2, l1[i]) * l1[i];
        i += 1;
    }
    println!("{}\n{}", answer1, answer2);
}
