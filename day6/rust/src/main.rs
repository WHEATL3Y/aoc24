use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;

fn simulate_path(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    seen: &mut HashSet<(usize, usize)>) -> Option<usize> {

    let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut d = 0;
    let mut x = start.1;
    let mut y = start.0;
    let mut path_len = 2;
    loop {
        seen.insert((y, x));
        y = (y as isize + dirs[d].0) as usize;
        x = (x as isize + dirs[d].1) as usize;
        if 0 > y as isize
            || y >= map.len() - 1
            || 0 > x as isize
            || x >= map[0].len() {
                break;
        }
        if map[y][x] == '#' {
            y = (y as isize - dirs[d].0) as usize;
            x = (x as isize - dirs[d].1) as usize;
            d = (d + 1) % 4;
        }
        if seen.contains(&(y, x)) {
            path_len -= 1;
        }
        else {
            path_len += 1;
        }
        if path_len == 0 {
            return None;
        }
    }
    Some(seen.len())
}

fn main() {
    let answer1;
    let mut answer2 = 0;
    let mut file = File::open("../../inputs/day6.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let mut pos: (usize, usize) = (0, 0);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let map: Vec<Vec<char>> = buf.split("\n").map(
            |s| s.chars().collect()
        ).collect();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                pos = (y, x);
            }
        }
    }
    
    match simulate_path(&map, pos, &mut seen) {
        Some(result) => answer1 = result,
        None => answer1 = 0,
    }
    for p in seen {
        let mut new_seen: HashSet<(usize, usize)> = HashSet::new();
        let mut new_map = map.clone();
        new_map[p.0][p.1] = '#';
        match simulate_path(&new_map, pos, &mut new_seen) {
            None => answer2 += 1, 
            _ => continue,
        }
    }
    println!("{}\n{}", answer1, answer2); 
}
