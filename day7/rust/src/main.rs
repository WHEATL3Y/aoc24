use std::fs::File;
use std::io::prelude::*;
fn concat(a: usize, b: usize) -> usize {
    let mut usize_str = usize::to_string(&a);
    usize_str.push_str(usize::to_string(&b).as_str());
    usize::from_str_radix(&usize_str, 10).unwrap()
}

fn check_answer(input: &Vec<usize>, answer: usize) -> bool  {
    if input.len() == 2 {
        let a = input[0];
        let b = input[1];
        if a * b == answer || a + b == answer {
            return true;
        }
        else {
            return false;
        }
    }
    
    let mut sum: Vec<usize> = Vec::from([input[0] + input[1]]);
    sum.extend_from_slice(&input[2..]);
    let mut prod: Vec<usize> = Vec::from([input[0] * input[1]]);
    prod.extend_from_slice(&input[2..]);
    
    return check_answer(&prod, answer) || check_answer(&sum, answer)
}

fn check_answer_with_concat(input: &Vec<usize>, answer: usize) -> bool  {
    if input.len() == 2 {
        let a = input[0];
        let b = input[1];
        if a * b == answer || a + b == answer || concat(a, b) == answer{
            return true;
        }
        else {
            return false;
        }
    }
    
    let mut sum: Vec<usize> = Vec::from([input[0] + input[1]]);
    sum.extend_from_slice(&input[2..]);
    let mut prod: Vec<usize> = Vec::from([input[0] * input[1]]);
    prod.extend_from_slice(&input[2..]);
    let mut concat: Vec<usize> = Vec::from([concat(input[0], input[1])]);
    concat.extend_from_slice(&input[2..]);
    
    return check_answer_with_concat(&prod, answer)
        || check_answer_with_concat(&sum, answer)
        || check_answer_with_concat(&concat, answer);
}

fn main() {
    let mut file = File::open("../../inputs/day7.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let mut answer1 = 0;
    let mut answer2 = 0;
   
    // Convert each line into a vec of usize, first value is result
    for line in contents.trim().split("\n") {
        let problem: Vec<usize> = line.split_whitespace()
            .map(
                |x| usize::from_str_radix(x.trim_end_matches(":"), 10)
                .unwrap()
            )
            .collect();
        let answer = problem[0];
        let terms: Vec<usize> = Vec::from(&problem[1..]);
        if check_answer(&terms, answer) {
            answer1 += answer;
        }
        if check_answer_with_concat(&terms, answer) {
            answer2 += answer;
        }
    }

    println!("{}\n{}", answer1, answer2);
}
