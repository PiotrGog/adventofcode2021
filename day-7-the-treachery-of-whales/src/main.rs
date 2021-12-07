use std::fs;

fn load_data(file_name: &str) -> Vec<usize> {
    let contents = fs::read_to_string(file_name).expect(&format!("Can't read file {}", file_name));
    contents
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn find_position_to_align_crabs<CostFunction>(
    init_crabs_positions: &Vec<usize>,
    cost_function: CostFunction,
) -> Option<(usize, usize)>
where
    CostFunction: Fn(usize, usize) -> usize,
{
    let max_position = *init_crabs_positions.iter().max().unwrap();
    let min_position = *init_crabs_positions.iter().min().unwrap();
    let mut min_cost = usize::MAX;
    let mut best_position = None;
    for position in min_position..=max_position {
        let cost = init_crabs_positions
            .iter()
            .fold(0usize, |acc, crab_position| {
                acc + cost_function(*crab_position, position)
            });
        if cost < min_cost {
            min_cost = cost;
            best_position = Some(position);
        }
    }

    if let Some(position) = best_position {
        Some((position, min_cost))
    } else {
        None
    }
}

fn one_move_one_fuel_cost_function(crab_postiion: usize, desire_postion: usize) -> usize {
    (crab_postiion as i64 - desire_postion as i64).abs() as usize
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let result = find_position_to_align_crabs(&data, one_move_one_fuel_cost_function).unwrap();
    println!("Part 1. Result position: {}, fuel: {}", result.0, result.1);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{find_position_to_align_crabs, load_data, one_move_one_fuel_cost_function};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(
            find_position_to_align_crabs(&data, one_move_one_fuel_cost_function),
            Some((2, 37))
        );
    }
}
