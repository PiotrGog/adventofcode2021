use std::{cmp, fs};

fn load_data(file_name: &str) -> Vec<usize> {
    let contents = fs::read_to_string(file_name).expect(&format!("Can't read file {}", file_name));
    contents
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn lanternfish_after_n_days(mut lanternfishes: Vec<usize>, mut days: usize) -> Vec<usize> {
    const NEW_CYCLE_INIT_VALUE: usize = 6;
    const FIRST_CYCLE_VALUE: usize = 8;
    while days > 0 {
        let min_days_to_new_lanternfishes =
            cmp::max(cmp::min(days, *lanternfishes.iter().min().unwrap()), 1);
        let lanternfishes_with_end_of_cycle = lanternfishes
            .iter()
            .filter(|lanternfish| **lanternfish == 0)
            .count();

        lanternfishes = lanternfishes
            .into_iter()
            .map(|lanternfish| {
                if lanternfish == 0 {
                    NEW_CYCLE_INIT_VALUE
                } else {
                    lanternfish - min_days_to_new_lanternfishes
                }
            })
            .collect();

        lanternfishes.append(&mut vec![
            FIRST_CYCLE_VALUE;
            lanternfishes_with_end_of_cycle
        ]);

        days -= min_days_to_new_lanternfishes;
    }
    lanternfishes
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 1. Result: {}",
        lanternfish_after_n_days(data.clone(), 80).len()
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{lanternfish_after_n_days, load_data};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(lanternfish_after_n_days(data.clone(), 18).len(), 26);
        assert_eq!(lanternfish_after_n_days(data.clone(), 80).len(), 5934);
    }
}
