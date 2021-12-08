use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
    vec,
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

#[rustfmt::skip]
fn decode_value_from_signal(signals_to_digit: &SignalsToDigits) -> usize{
    let mut segments_val = HashMap::new();
    segments_val.insert('a', 1);
    segments_val.insert('b', 2);
    segments_val.insert('c', 4);
    segments_val.insert('d', 8);
    segments_val.insert('e', 16);
    segments_val.insert('f', 32);
    segments_val.insert('g', 64);
    let segments_val = segments_val;

    let convert_signals_to_binary_code = |signals: &[String]| {
        signals
            .iter()
            .map(|signal| {
                signal.chars().map(|s| segments_val.get(&s).unwrap()).sum::<u8>()
            })
            .collect::<Vec<_>>()
    };

    let mut signals = convert_signals_to_binary_code(&signals_to_digit.signals);
    signals.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()));

    let mut signal_codes = vec![0; 10];
    let mut segment_codes = HashMap::new();

    signal_codes[1] = signals[0];
    signal_codes[7] = signals[1];
    signal_codes[4] = signals[2];
    signal_codes[8] = signals[9];
    segment_codes.insert('a', signal_codes[7] - signal_codes[1]);
    segment_codes.insert('f', signals[6] & signals[7] & signals[8] & signal_codes[1]);
    segment_codes.insert('c', signal_codes[1] ^ segment_codes[&'f']);
    signal_codes[6] = signal_codes[8] - segment_codes[&'c'];
    segment_codes.insert('g', (signal_codes[8] ^ (signal_codes[4] | segment_codes[&'a'])) & (signals[3] & signals[4] & signals[5]));
    segment_codes.insert('e', (signal_codes[8] ^ (signal_codes[4] | segment_codes[&'a'])) - segment_codes[&'g']);
    signal_codes[5] = signal_codes[6] - segment_codes[&'e'];
    signal_codes[9] = signal_codes[8] - segment_codes[&'e'];
    segment_codes.insert('d', (signals[3] & signals[4] & signals[5]) - segment_codes[&'g'] - segment_codes[&'a']);
    signal_codes[0] = signal_codes[8] - segment_codes[&'d'];
    signal_codes[2] = segment_codes[&'a'] | segment_codes[&'c'] | segment_codes[&'d'] | segment_codes[&'e'] | segment_codes[&'g'];
    signal_codes[3] = segment_codes[&'a'] | segment_codes[&'c'] | segment_codes[&'d'] | segment_codes[&'f'] | segment_codes[&'g'];
    segment_codes.insert('b', signal_codes[8] - (signal_codes[2] | segment_codes[&'f']));

    let digits = convert_signals_to_binary_code(&signals_to_digit.digits);
    digits.iter().fold(0, |acc, code| {
        acc * 10 + signal_codes.iter().position(|c| c == code).unwrap()
    })
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 1. Result: {}",
        count_numbers_with_n_segments(&data, &[2, 3, 4, 7])
    );
}

fn part_2_result(file_name: &str) {
    let data = load_data(file_name);
    let result = data.into_iter().fold(0, |acc, signals_to_digits| {
        acc + decode_value_from_signal(&signals_to_digits)
    });
    println!("Part 2. Result: {}", result);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{
        count_numbers_with_n_segments, decode_value_from_signal, load_data, SignalsToDigits,
    };

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(count_numbers_with_n_segments(&data, &[2, 3, 4, 7]), 26);
    }

    #[test]
    fn part_2_test_data() {
        let signals_to_digits = SignalsToDigits {
            signals: vec![
                "acedgfb".to_string(),
                "cdfbe".to_string(),
                "gcdfa".to_string(),
                "fbcad".to_string(),
                "dab".to_string(),
                "cefabd".to_string(),
                "cdfgeb".to_string(),
                "eafb".to_string(),
                "cagedb".to_string(),
                "ab".to_string(),
            ],
            digits: vec![
                "cdfeb".to_string(),
                "fcadb".to_string(),
                "cdfeb".to_string(),
                "cdbaf".to_string(),
            ],
        };
        assert_eq!(decode_value_from_signal(&signals_to_digits), 5353);
    }
}
