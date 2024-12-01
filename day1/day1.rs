use std::fs::File;
use std::io::prelude::*;

fn get_input(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<()> {
    
    // Split input into lines, loop through lines and split into 
    // individual numbers, add numbers to l1, and l2.
    let filename = "../inputs/day1.sample.txt";
    // let filename = "../inputs/day1.txt";
    let mut l1: Vec<usize> = Vec::<usize>::new();
    let mut l2: Vec<usize> = Vec::<usize>::new();
    for line in get_input(filename)?.split("\n") {
        println!("{}", line);
        let mut numbers = line.split_whitespace();
        l1.push(usize::from_str_radix(numbers.next().unwrap(), 10)?);
    }
    Ok(())
}
