use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("../../inputs/day8.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let map: Vec<Vec<char>> = contents.trim().split("\n")
        .map(|x| x.chars().collect())
        .collect();
    let mut answer1 = 0;
    let mut answer2 = 0;
    let mut antennas: Vec<(isize, isize, char)> = Vec::new();   
    let mut antinodes: Vec<(isize, isize)> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != '.' {
                antennas.push((x as isize, y as isize, map[y][x]));
            }
        }
    }

    for v in &antennas {
        for t in &antennas {
            if v.2 == t.2 && v.0 != t.0 && v.1 != t.1 {
                let y = v.1 + (v.1 - t.1);
                let x = v.0 + (v.0 - t.0);
                if x >= 0 && x < map[0].len() as isize
                    && y >= 0 && y < map.len() as isize 
                    && !antinodes.contains(&(x, y)) {
                        antinodes.push((x, y));
                        answer1 += 1;
                }
            }
        }
    }
    antinodes.clear();
    for v in &antennas {
        let mut i = 0;
        for t in &antennas {
            i += 1;
            if v.2 == t.2 && v.0 != t.0 && v.1 != t.1 {
                let mut y = v.1 + (v.1 - t.1);
                let mut x = v.0 + (v.0 - t.0);
                while x >= 0 && x < map[0].len() as isize
                    && y >= 0 && y < map.len() as isize {
                        if !antinodes.contains(&(x, y)) {
                            antinodes.push((x, y));
                        }
                        answer2 += 1;
                        y = y + (v.1 - t.1);
                        x = x + (v.0 - t.0);
                }
            }
        }
        if i > 0 && !antinodes.contains(&(v.0, v.1)) {
            antinodes.push((v.0, v.1));
            answer2 += 1;
        }

    }
    println!("{}\n{}", answer1, antinodes.len());
}
