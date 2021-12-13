use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Octopuses = Vec<Vec<u32>>;

const ALREADY_FLASHED_VALUE: u32 = std::u32::MAX;
const TO_FLASH_THRESHOLD_VALUE: u32 = 10;

fn load_data(file_name: &str) -> Octopuses {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);
    file.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|string_digit| string_digit.to_digit(10).unwrap() as u32)
                .collect()
        })
        .collect()
}

fn increase_energy_level_by_one(octopuses: Octopuses) -> Octopuses {
    octopuses
        .into_iter()
        .map(|octopuses_row| {
            octopuses_row
                .into_iter()
                .map(|octopus| octopus + 1)
                .collect()
        })
        .collect()
}

fn valid_coordinates(coordinates_to_check: (i32, i32), valid_size: (i32, i32)) -> bool {
    coordinates_to_check.0 >= 0
        && coordinates_to_check.1 >= 0
        && coordinates_to_check.0 < valid_size.0 as i32
        && coordinates_to_check.1 < valid_size.1 as i32
}

fn recurent_flash(current_coordinates: (usize, usize), octopuses: &mut Octopuses) {
    let (current_x, current_y) = current_coordinates;

    if octopuses[current_x][current_y] >= TO_FLASH_THRESHOLD_VALUE
        && octopuses[current_x][current_y] != ALREADY_FLASHED_VALUE
    {
        octopuses[current_x][current_y] = ALREADY_FLASHED_VALUE;
        for x in 0..=2 {
            for y in 0..=2 {
                let x_to_check = (current_x + x) as i32 - 1;
                let y_to_check = (current_y + y) as i32 - 1;

                if valid_coordinates(
                    (x_to_check, y_to_check),
                    (octopuses.len() as i32, octopuses.len() as i32),
                ) && (x_to_check != current_x as i32 || y_to_check != current_y as i32)
                {
                    let x_to_check = x_to_check as usize;
                    let y_to_check = y_to_check as usize;
                    if octopuses[x_to_check][y_to_check] != ALREADY_FLASHED_VALUE {
                        octopuses[x_to_check][y_to_check] += 1;
                        recurent_flash((x_to_check, y_to_check), octopuses);
                    }
                }
            }
        }
    }
}

fn iterative_flash(mut octopuses: Octopuses) -> Octopuses {
    for x in 0..octopuses.len() {
        for y in 0..octopuses.len() {
            recurent_flash((x, y), &mut octopuses)
        }
    }
    octopuses
}

fn iterate(octopuses: Octopuses) -> (Octopuses, usize) {
    let octopuses = increase_energy_level_by_one(octopuses);
    let mut octopuses = iterative_flash(octopuses);

    let mut flashed_counter = 0;
    for x in 0..octopuses.len() {
        for y in 0..octopuses.len() {
            if octopuses[x][y] == ALREADY_FLASHED_VALUE {
                flashed_counter += 1;
                octopuses[x][y] = 0;
            }
        }
    }

    (octopuses, flashed_counter)
}

fn iterate_n_times(octopuses: Octopuses, times: usize) -> (Octopuses, usize) {
    let mut octopuses = octopuses;
    let mut flashes = 0;
    for _ in 0..times {
        let iteration_result = iterate(octopuses);
        octopuses = iteration_result.0;
        flashes += iteration_result.1;
    }

    (octopuses, flashes)
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!("Part 1. Result: {}", iterate_n_times(data, 100).1,);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{iterate, iterate_n_times, load_data};

    #[test]
    fn part_1_test_data_1() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data_1.txt";
        let data = load_data(TEST_DATA_FILENAME);

        let iteration_result = iterate(data.clone());

        assert_eq!(
            iteration_result,
            (
                vec![
                    vec![3, 4, 5, 4, 3],
                    vec![4, 0, 0, 0, 4],
                    vec![5, 0, 0, 0, 5],
                    vec![4, 0, 0, 0, 4],
                    vec![3, 4, 5, 4, 3],
                ],
                9
            )
        );

        let iteration_result = iterate(iteration_result.0);
        assert_eq!(
            iteration_result,
            (
                vec![
                    vec![4, 5, 6, 5, 4],
                    vec![5, 1, 1, 1, 5],
                    vec![6, 1, 1, 1, 6],
                    vec![5, 1, 1, 1, 5],
                    vec![4, 5, 6, 5, 4],
                ],
                0
            )
        );
    }

    #[test]
    fn part_1_test_data_2() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data_2.txt";
        let data = load_data(TEST_DATA_FILENAME);

        let (data, flashes) = iterate_n_times(data, 100);

        assert_eq!(
            (data, flashes),
            (
                vec![
                    vec![0, 3, 9, 7, 6, 6, 6, 8, 6, 6],
                    vec![0, 7, 4, 9, 7, 6, 6, 9, 1, 8],
                    vec![0, 0, 5, 3, 9, 7, 6, 9, 3, 3],
                    vec![0, 0, 0, 4, 2, 9, 7, 8, 2, 2],
                    vec![0, 0, 0, 4, 2, 2, 9, 8, 9, 2],
                    vec![0, 0, 5, 3, 2, 2, 2, 8, 7, 7],
                    vec![0, 5, 3, 2, 2, 2, 2, 9, 6, 6],
                    vec![9, 3, 2, 2, 2, 2, 8, 9, 6, 6],
                    vec![7, 9, 2, 2, 2, 8, 6, 8, 6, 6],
                    vec![6, 7, 8, 9, 9, 9, 8, 7, 6, 6],
                ],
                1656
            )
        );
    }
}
