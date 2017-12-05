use std::io;
use std::fs;
use std::io::BufRead;

pub fn part1(input: &str) -> String {
    let file = fs::File::open(input).expect("Failed to open file");
    let reader = io::BufReader::new(&file);

    let mut jumps: Vec<isize> = vec![];
    for l in reader.lines() {
        let line = l.expect("Failed to read from file");
        let line = line.trim();
        jumps.push(line.parse().expect("Failed to convert line to integer"));
    }

    let mut index: isize = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let jump = jumps[index as usize];
        jumps[index as usize] += 1;
        index += jump;
        if index < 0 || index >= jumps.len() as isize {
            return steps.to_string();
        }
    }
}

pub fn part2(input: &str) -> String {
    let file = fs::File::open(input).expect("Failed to open file");
    let reader = io::BufReader::new(&file);

    let mut jumps: Vec<isize> = vec![];
    for l in reader.lines() {
        let line = l.expect("Failed to read from file");
        let line = line.trim();
        jumps.push(line.parse().expect("Failed to convert line to integer"));
    }

    let mut index: isize = 0;
    let mut steps = 0;
    loop {
        steps += 1;
        let jump = jumps[index as usize];
        jumps[index as usize] += if jump < 3 { 1 } else { -1 };
        index += jump;
        if index < 0 || index >= jumps.len() as isize {
            return steps.to_string();
        }
    }
}
