use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeBounds,
};

type Octopuses = Vec<Vec<u8>>;

fn load_data(file_name: &str) -> Octopuses {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);
    file.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|string_digit| string_digit.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn iterate(octopuses: Octopuses) -> Octopuses {
    let mut octopuses: Octopuses = octopuses
        .into_iter()
        .map(|octopuses_row| {
            octopuses_row
                .into_iter()
                .map(|octopus| octopus + 1)
                .collect()
        })
        .collect();

    let mut to_flash = VecDeque::new();
    for (x, octopuses_row) in octopuses.iter().enumerate() {
        for (y, octopus) in octopuses_row.iter().enumerate() {
            if *octopus > 9 {
                to_flash.push_back((x, y));
            }
        }
    }

    let mut flashed = HashSet::new();
    while let Some(cords) = to_flash.pop_front() {
        flashed.insert(cords);
        for x in 0..=2 {
            for y in 0..=2 {
                let x_to_check = (cords.0 + x) as i32 - 1;
                let y_to_check = (cords.1 + y) as i32 - 1;
                if x_to_check < 0
                    || y_to_check < 0
                    || x_to_check >= octopuses.len() as i32
                    || y_to_check >= octopuses.len() as i32
                    || x_to_check == cords.0 as i32
                    || y_to_check == cords.1 as i32
                {
                    continue;
                } else {
                    let x_to_check = x_to_check as usize;
                    let y_to_check = y_to_check as usize;
                    octopuses[x_to_check][y_to_check] += 1;
                    if octopuses[x_to_check][y_to_check] > 9
                        && !flashed.contains(&(x_to_check, y_to_check))
                    {
                        to_flash.push_back((x_to_check, y_to_check));
                    }
                }
            }
        }
    }

    for (x, octopuses_row) in octopuses.iter_mut().enumerate() {
        for (y, octopus) in octopuses_row.iter_mut().enumerate() {
            if *octopus > 9 {
                *octopus = 0;
            }
        }
    }

    octopuses
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{iterate, load_data};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);

        let iteration_result = iterate(data.clone());
        println!("{:?}", iteration_result);
        assert_eq!(
            iteration_result,
            vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3],
            ]
        );

        let iteration_result = iterate(iteration_result);
        assert_eq!(
            iteration_result,
            vec![
                vec![4, 5, 6, 5, 4],
                vec![5, 1, 1, 1, 5],
                vec![6, 1, 1, 1, 6],
                vec![5, 1, 1, 1, 5],
                vec![4, 5, 6, 5, 4],
            ]
        );
    }
}
