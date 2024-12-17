use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("../../inputs/day8.test").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let map: Vec<Vec<&str>> = contents.trim().split("\n")
        .map(|x| x.split("").collect())
        .collect();
    
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x]);
        }
        println!();
    }
}
