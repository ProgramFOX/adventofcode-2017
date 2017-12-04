use std::fs;
use std::io;
use std::io::BufRead;

fn is_valid_passphrase(pp: &str) -> bool {
    let mut words: Vec<&str> = pp.split(" ").collect();
    words.sort();
    let length1 = words.len();
    words.dedup();
    length1 == words.len()
}

pub fn part1(input: &str) -> String {
    let file = fs::File::open(input).expect("Failed to open file");
    let reader = io::BufReader::new(&file);

    let mut sum = 0;
    for l in reader.lines() {
        let line = l.expect("Failed to read from file");
        let line = line.trim();
        if is_valid_passphrase(line) {
            sum += 1;
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    String::from("not yet implemented")
}
