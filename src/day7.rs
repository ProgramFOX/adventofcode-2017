use std::collections::HashSet;
use std::io;
use std::fs;
use std::io::BufRead;

pub fn part1(input: &str) -> String {
    let mut names: HashSet<String> = HashSet::new();
    let mut not_bottom: HashSet<String> = HashSet::new();

    let file = fs::File::open(input).expect("Failed to open file");
    let reader = io::BufReader::new(&file);

    for l in reader.lines() {
        let line = l.expect("Failed to read from file");
        let line = line.trim();

        let name_and_carrying: Vec<&str> = line.split(" -> ").collect();

        if name_and_carrying.len() == 2 {
            let carrying: Vec<&str> = name_and_carrying[1].split(", ").collect();
            for c in carrying {
                not_bottom.insert(String::from(c));
            }
        }

        names.insert(String::from(
            name_and_carrying[0].split(" ").nth(0).unwrap(),
        ));
    }

    (*names.difference(&not_bottom).nth(0).unwrap()).clone()
}

pub fn part2(input: &str) -> String {
    String::from("not yet implemented")
}
