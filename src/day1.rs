pub fn part1(input: &str) -> String {
    let mut sum = 0;

    for i in 0..input.len() {
        let next_index = (i + 1) % input.len();
        let current_char = input.chars().nth(i).unwrap();
        if current_char == input.chars().nth(next_index).unwrap() {
            sum += current_char.to_digit(10).unwrap();
        }
    }

    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let mut sum = 0;

    for i in 0..input.len() {
        let next_index = (i + input.len() / 2) % input.len();
        let current_char = input.chars().nth(i).unwrap();
        if current_char == input.chars().nth(next_index).unwrap() {
            sum += current_char.to_digit(10).unwrap();
        }
    }

    sum.to_string()
}
