mod cuboid;
mod cuboids_range;
mod initialization_procedure;
mod parse_reboot_step_error;
mod reboot_step;

use initialization_procedure::InitializationProcedure;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use cuboids_range::CuboidsRange;

fn load_data(file_name: &str) -> InitializationProcedure {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    let steps = file
        .lines()
        .map(|r_line| r_line.unwrap().parse().unwrap())
        .collect();
    InitializationProcedure { steps }
}

fn part_1_result(file_name: &str) {
    let initialization_procedure = load_data(file_name);
    let result_cuboids =
        initialization_procedure.run(Some(CuboidsRange::new((-50, 50), (-50, 50), (-50, 50))));
    let result = result_cuboids
        .iter()
        .fold(0, |sum, cuboid| sum + cuboid.count_cubes_on());

    println!("Part 1. Result: {}", result);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{cuboid::Cuboid, cuboids_range::CuboidsRange, reboot_step::RebootStep};

    use super::*;

    #[test]
    fn test_load_data() {
        const TEST_FILE_PATH: &str = "./resources/test_data.txt";
        let loaded_data = load_data(TEST_FILE_PATH);

        let expected_data = InitializationProcedure::new(vec![
            RebootStep::new_on(Cuboid::new((-20, 26), (-36, 17), (-47, 7))),
            RebootStep::new_on(Cuboid::new((-20, 33), (-21, 23), (-26, 28))),
            RebootStep::new_on(Cuboid::new((-22, 28), (-29, 23), (-38, 16))),
            RebootStep::new_on(Cuboid::new((-46, 7), (-6, 46), (-50, -1))),
            RebootStep::new_on(Cuboid::new((-49, 1), (-3, 46), (-24, 28))),
            RebootStep::new_on(Cuboid::new((2, 47), (-22, 22), (-23, 27))),
            RebootStep::new_on(Cuboid::new((-27, 23), (-28, 26), (-21, 29))),
            RebootStep::new_on(Cuboid::new((-39, 5), (-6, 47), (-3, 44))),
            RebootStep::new_on(Cuboid::new((-30, 21), (-8, 43), (-13, 34))),
            RebootStep::new_on(Cuboid::new((-22, 26), (-27, 20), (-29, 19))),
            RebootStep::new_off(Cuboid::new((-48, -32), (26, 41), (-47, -37))),
            RebootStep::new_on(Cuboid::new((-12, 35), (6, 50), (-50, -2))),
            RebootStep::new_off(Cuboid::new((-48, -32), (-32, -16), (-15, -5))),
            RebootStep::new_on(Cuboid::new((-18, 26), (-33, 15), (-7, 46))),
            RebootStep::new_off(Cuboid::new((-40, -22), (-38, -28), (23, 41))),
            RebootStep::new_on(Cuboid::new((-16, 35), (-41, 10), (-47, 6))),
            RebootStep::new_off(Cuboid::new((-32, -23), (11, 30), (-14, 3))),
            RebootStep::new_on(Cuboid::new((-49, -5), (-3, 45), (-29, 18))),
            RebootStep::new_off(Cuboid::new((18, 30), (-20, -8), (-3, 13))),
            RebootStep::new_on(Cuboid::new((-41, 9), (-7, 43), (-33, 15))),
            RebootStep::new_on(Cuboid::new(
                (-54112, -39298),
                (-85059, -49293),
                (-27449, 7877),
            )),
            RebootStep::new_on(Cuboid::new((967, 23432), (45373, 81175), (27513, 53682))),
        ]);

        assert_eq!(loaded_data, expected_data);
    }

    #[test]
    fn test_part_1() {
        const TEST_FILE_PATH: &str = "./resources/test_data.txt";
        let initialization_procedure = load_data(TEST_FILE_PATH);
        let result_cuboids =
            initialization_procedure.run(Some(CuboidsRange::new((-50, 50), (-50, 50), (-50, 50))));

        let result = result_cuboids
            .iter()
            .fold(0, |sum, cuboid| sum + cuboid.count_cubes_on());
        assert_eq!(result, 590784);
    }
}
