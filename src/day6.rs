use std::collections::HashSet;

fn stringify_memory(memory: &Vec<u32>) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        memory[0],
        memory[1],
        memory[2],
        memory[3],
        memory[4],
        memory[5],
        memory[6],
        memory[7],
        memory[8],
        memory[9],
        memory[10],
        memory[11],
        memory[12],
        memory[13],
        memory[14],
        memory[15]
    )
}

pub fn part1(input: &str) -> String {
    let mut memory: Vec<u32> = input.split(" ").map(|s| s.parse().unwrap()).collect();

    let mut previous = HashSet::new();
    previous.insert(stringify_memory(&memory));
    let mut steps = 1;
    loop {
        let mem_clone = memory.clone();
        let max = mem_clone.iter().max().unwrap();
        let highest_index = memory.iter().position(|a| a == max).unwrap();
        let value = memory[highest_index];
        memory[highest_index] = 0;

        let mut index = (highest_index + 1) % 16;
        for x in 0..value {
            memory[index] += 1;
            index = (index + 1) % 16;
        }

        let s = stringify_memory(&memory);
        if previous.contains(&s) {
            return steps.to_string();
        }
        previous.insert(s);
        steps += 1;
    }
}

pub fn part2(input: &str) -> String {
    let mut memory: Vec<u32> = input.split(" ").map(|s| s.parse().unwrap()).collect();

    let mut previous = vec![];
    previous.push(stringify_memory(&memory));
    let mut steps = 1;
    loop {
        let mem_clone = memory.clone();
        let max = mem_clone.iter().max().unwrap();
        let highest_index = memory.iter().position(|a| a == max).unwrap();
        let value = memory[highest_index];
        memory[highest_index] = 0;

        let mut index = (highest_index + 1) % 16;
        for x in 0..value {
            memory[index] += 1;
            index = (index + 1) % 16;
        }

        let s = stringify_memory(&memory);
        if previous.contains(&s) {
            return (steps - previous.iter().position(|t| t == &s).unwrap() as i32).to_string();
        }
        previous.push(s);
        steps += 1;
    }
}
