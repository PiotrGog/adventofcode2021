use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
    u32,
};

#[derive(Debug, PartialEq, Clone)]
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

impl From<BinaryNumber> for u32 {
    fn from(binary_number: BinaryNumber) -> Self {
        binary_number
            .0
            .iter()
            .fold(0, |acc, val| (acc << 1) | (*val as u32))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rates {
    gamma: u32,
    epsilon: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct RatesOxygenAndCO2 {
    oxygen: u32,
    co2: u32,
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

fn get_oxygen_and_co2_ratting(binary_numbers: Vec<BinaryNumber>) -> RatesOxygenAndCO2 {
    let mut binary_numbers_for_oxygen = binary_numbers.clone();
    let mut binary_numbers_for_co2 = binary_numbers.clone();
    for i in 0..binary_numbers[0].0.len() {
        if binary_numbers_for_oxygen.len() > 1 {
            let numbers_for_oxygen = binary_numbers_for_oxygen.len();
            let counted_ones = count_ones(&binary_numbers_for_oxygen)[i];
            binary_numbers_for_oxygen = binary_numbers_for_oxygen
                .into_iter()
                .filter(|val| {
                    if (counted_ones * 2) as usize >= numbers_for_oxygen {
                        val.0[i] == 1
                    } else {
                        val.0[i] == 0
                    }
                })
                .collect();
        }
        if binary_numbers_for_co2.len() > 1 {
            let numbers_of_co2 = binary_numbers_for_co2.len();
            let counted_ones = count_ones(&binary_numbers_for_co2)[i];
            binary_numbers_for_co2 = binary_numbers_for_co2
                .into_iter()
                .filter(|val| {
                    if (counted_ones * 2) as usize >= numbers_of_co2 {
                        val.0[i] == 0
                    } else {
                        val.0[i] == 1
                    }
                })
                .collect();
        }
    }

    RatesOxygenAndCO2 {
        oxygen: u32::from(binary_numbers_for_oxygen[0].clone()),
        co2: u32::from(binary_numbers_for_co2[0].clone()),
    }
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

fn part_2_result(file_name: &str) {
    let data = load_data(file_name);
    let rates = get_oxygen_and_co2_ratting(data);
    println!("Part 2. Result: {}", rates.oxygen * rates.co2);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{
        count_ones, get_oxygen_and_co2_ratting, get_rates, load_data, Rates, RatesOxygenAndCO2,
    };

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let counted_ones = count_ones(&data);
        assert_eq!(
            get_rates(&counted_ones, data.len()),
            Rates {
                gamma: 22,
                epsilon: 9
            }
        );
    }

    #[test]
    fn part_2_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(
            get_oxygen_and_co2_ratting(data),
            RatesOxygenAndCO2 {
                oxygen: 23,
                co2: 10
            }
        );
    }
}
