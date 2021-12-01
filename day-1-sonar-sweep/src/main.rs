use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn load_data(file_name: &str) -> Vec<u32> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    file.lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

fn count_depth_increases(data: &Vec<u32>) -> u32 {
    data.windows(2).fold(
        0,
        |acc, value| if value[0] < value[1] { acc + 1 } else { acc },
    )
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!("Part 1. Result: {}", count_depth_increases(&data));
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{count_depth_increases, load_data};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(count_depth_increases(&data), 7);
    }
}
