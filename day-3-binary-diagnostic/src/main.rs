use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
};

struct BinaryNumber(Vec<u8>);

impl FromStr for BinaryNumber {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BinaryNumber(
            s.chars()
                .map(|ch| if ch.eq(&'0') { 0 } else { 1 })
                .collect(),
        ))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rates {
    gamma: u32,
    epsilon: u32,
}

fn count_ones(data: &Vec<BinaryNumber>) -> Vec<u32> {
    data.iter()
        .fold(vec![0u32; data[0].0.len()], |acc, binary_num| {
            acc.into_iter()
                .zip(binary_num.0.iter())
                .map(|(a, b)| a + (*b as u32))
                .collect()
        })
}

fn get_rates(data: &Vec<u32>, binary_numbers: usize) -> Rates {
    let (gamma, epsilon) = data.iter().fold((0u32, 0u32), |acc, count| {
        (
            (acc.0 << 1) | {
                if (count * 2) as usize > binary_numbers {
                    1
                } else {
                    0
                }
            },
            (acc.1 << 1) | {
                if (count * 2) as usize > binary_numbers {
                    0
                } else {
                    1
                }
            },
        )
    });
    Rates { gamma, epsilon }
}

fn load_data(file_name: &str) -> Vec<BinaryNumber> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    file.lines()
        .map(|line| BinaryNumber::from_str(line.unwrap().as_str()).unwrap())
        .collect()
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let counted_ones = count_ones(&data);
    let rates = get_rates(&counted_ones, data.len());
    println!("Part 1. Result: {}", rates.gamma * rates.epsilon);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{count_ones, get_rates, load_data, Rates};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let counted_ones = count_ones(&data);
        println!("{:?}", counted_ones);
        assert_eq!(
            get_rates(&counted_ones, data.len()),
            Rates {
                gamma: 22,
                epsilon: 9
            }
        );
    }
}
