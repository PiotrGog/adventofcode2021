use std::{
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, PartialEq)]
enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(' ').collect::<Vec<_>>();

        let value = s[1].parse().expect("error");
        match s[0] {
            "forward" => Ok(Movement::Forward(value)),
            "down" => Ok(Movement::Down(value)),
            "up" => Ok(Movement::Up(value)),
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: u32,
    depth: u32,
}

fn get_final_position_following_movements(data: &Vec<Movement>) -> Position {
    data.iter().fold(
        Position {
            horizontal: 0,
            depth: 0,
        },
        |position, movement| match movement {
            Movement::Forward(val) => Position {
                horizontal: position.horizontal + val,
                ..position
            },
            Movement::Down(val) => Position {
                depth: position.depth + val,
                ..position
            },
            Movement::Up(val) => Position {
                depth: position.depth - val,
                ..position
            },
        },
    )
}

fn load_data(file_name: &str) -> Vec<Movement> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    file.lines()
        .map(|line| Movement::from_str(line.unwrap().as_str()).unwrap())
        .collect()
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let final_position = get_final_position_following_movements(&data);
    println!(
        "Part 1. Result: {}",
        final_position.horizontal * final_position.depth
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{get_final_position_following_movements, load_data, Position};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(
            get_final_position_following_movements(&data),
            Position {
                horizontal: 15,
                depth: 10
            }
        );
    }
}
