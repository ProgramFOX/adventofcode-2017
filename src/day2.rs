use std::fs;
use std::io;
use std::io::BufRead;

pub fn part1(input: &str) -> String {
    let file = fs::File::open(input).expect("Failed to open file");
    let reader = io::BufReader::new(&file);

    let mut sum = 0;
    for l in reader.lines() {
        let line = l.expect("Failed to read from file");
        let integers: Vec<i32> = line.split("\t")
            .map(|x| x.parse().expect("Input file contained non-integers"))
            .collect();
        let min = integers.iter().min().expect("Failed to find min value");
        let max = integers.iter().max().expect("Failed to find max value");
        sum += max - min;
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    String::from("not yet implemented")
}
