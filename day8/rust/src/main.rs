use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("../../inputs/day8.test").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let map: Vec<&str> = contents.split("\n").collect();

}
