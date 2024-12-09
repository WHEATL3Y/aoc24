use std::collections::HashSet;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let answer1;
    let mut answer2 = 0;
    let mut file = File::open("../../inputs/day6.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let dirs: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut d = 0;
    let mut pos: (usize, usize) = (0, 0);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let map: Vec<Vec<char>> = buf.split("\n").map(
            |s| s.chars().collect()
        ).collect();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            assert!(map[y].len() > 0);
            if map[y][x] == '^' {
                pos = (y, x);
            }
        }
    }
    assert!(pos.0 != 0 && pos.1 != 0); 
    let mut ny = pos.0;
    let mut nx = pos.1;
    loop {
        println!("{}, {}", nx, ny);
        seen.insert((ny, nx));
        ny = (ny as isize + dirs[d].0) as usize;
        nx = (nx as isize + dirs[d].1) as usize;
        if 0 > ny as isize
            || ny >= map.len()
            || 0 > nx as isize
            || nx >= map[0].len() {
                break;
        }
        if map[ny][nx] == '#' {
            d = (d + 1) % 4;
        }
    }
    answer1 = seen.len();
    println!("{}", answer1); 
}
