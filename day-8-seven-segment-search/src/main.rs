use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
};

type Signal = String;
type Digit = String;

#[derive(Clone, Debug, PartialEq)]
struct SignalsToDigits {
    signals: Vec<Signal>,
    digits: Vec<Digit>,
}

impl FromStr for SignalsToDigits {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut signals_and_digits = s.split(" | ");
        let signals = signals_and_digits
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let digits = signals_and_digits
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        Ok(SignalsToDigits { signals, digits })
    }
}

fn load_data(file_name: &str) -> Vec<SignalsToDigits> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);
    file.lines()
        .map(|line| SignalsToDigits::from_str(line.unwrap().as_str()).unwrap())
        .collect()
}

fn count_numbers_with_n_segments(data: &Vec<SignalsToDigits>, segments_n: &[usize]) -> usize {
    data.iter().fold(0usize, |acc_all, signals_to_digit| {
        acc_all
            + signals_to_digit
                .digits
                .iter()
                .filter(|digit| segments_n.iter().any(|n| *n == digit.len()))
                .count()
    })
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 1. Result: {}",
        count_numbers_with_n_segments(&data, &[2, 3, 4, 7])
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{count_numbers_with_n_segments, load_data};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(count_numbers_with_n_segments(&data, &[2, 3, 4, 7]), 26);
    }
}
