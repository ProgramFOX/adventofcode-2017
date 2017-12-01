extern crate aoclib;

use std::io;

fn main() {
    println!("Day:");
    let mut day_input = String::new();
    io::stdin()
        .read_line(&mut day_input)
        .expect("Failed to read from stdin");
    let day: usize = day_input.trim().parse().expect("Please input an integer.");

    println!("Part one? (y/n)");
    let mut part_input = String::new();
    io::stdin()
        .read_line(&mut part_input)
        .expect("Failed to read from stdin");
    let part_one = part_input.trim() == "y";

    println!("Input:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    println!("{}", aoclib::call_function(day, part_one, input));
}
