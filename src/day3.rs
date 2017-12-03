#[derive(PartialEq)]
struct Point(i32, i32);

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Point {
    fn perform_move(self, dir: Direction) -> Point {
        match dir {
            Direction::Left => Point(self.0 - 1, self.1),
            Direction::Right => Point(self.0 + 1, self.1),
            Direction::Up => Point(self.0, self.1 + 1),
            Direction::Down => Point(self.0, self.1 - 1),
        }
    }
}


pub fn part1(input: &str) -> String {
    let target: i32 = input.parse().expect("Input is not an integer");

    let mut bottom_right_value = 0;
    let mut i = 1i32;
    let mut bottom_right = Point(0, 0);
    loop {
        bottom_right_value = i.pow(2);
        bottom_right = Point((i - 1) / 2, -(i - 1) / 2);
        if target <= bottom_right_value {
            break;
        }
        i += 2;
    }

    let mut current = bottom_right_value;

    let mut current_point = Point(bottom_right.0, bottom_right.1);

    let bottom_left = Point(-bottom_right.0, bottom_right.1);
    let top_left = Point(-bottom_right.0, -bottom_right.1);
    let top_right = Point(bottom_right.0, -bottom_right.1);

    let mut dir = Direction::Left;
    loop {
        if current == target {
            break;
        }
        dir = if current_point == bottom_left {
            Direction::Up
        } else if current_point == top_left {
            Direction::Right
        } else if current_point == top_right {
            Direction::Down
        } else {
            dir
        };
        current -= 1;
        current_point = current_point.perform_move(dir);
    }

    (current_point.0.abs() + current_point.1.abs()).to_string()
}

pub fn part2(input: &str) -> String {
    String::from("not yet implemented")
}
