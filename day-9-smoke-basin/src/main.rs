use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

type Area = Vec<Vec<u8>>;

fn load_data(file_name: &str) -> Area {
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

fn sum_of_risk_levels<RiskCostFun>(area: &Area, risk_fun: RiskCostFun) -> usize
where
    RiskCostFun: Fn(u8) -> usize,
{
    let x_max = area.len();
    let y_max = area[0].len();
    let mask = |x: usize, y: usize| {
        [
            if x as i64 - 1 < 0 { 10 } else { area[x - 1][y] },
            if y as i64 - 1 < 0 { 10 } else { area[x][y - 1] },
            if x + 1 >= x_max { 10 } else { area[x + 1][y] },
            if y + 1 >= y_max { 10 } else { area[x][y + 1] },
        ]
        .iter()
        .all(|adjacent| *adjacent > area[x][y])
    };
    let mut sum = 0;
    for x in 0..x_max {
        for y in 0..y_max {
            sum += if mask(x, y) { risk_fun(area[x][y]) } else { 0 };
        }
    }

    return sum;
}

fn find_n_largest(area: &Area, n: usize) -> Vec<HashSet<(i32, i32)>> {
    let mut basins_points = HashSet::new();
    for x in 0..area.len() {
        for y in 0..area[0].len() {
            if area[x][y] < 9 {
                basins_points.insert((x as i32, y as i32));
            };
        }
    }

    let mut basins = vec![];
    let adjacent = |x, y| VecDeque::from(vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]);

    while let Some(cord) = basins_points.iter().next() {
        let mut cords_to_check = VecDeque::from(vec![*cord]);
        let mut new_basin = HashSet::new();
        while let Some(front) = cords_to_check.pop_front() {
            if basins_points.remove(&front) {
                cords_to_check.append(&mut adjacent(front.0, front.1));
                new_basin.insert(front);
            }
        }
        basins.push(new_basin);
    }

    basins.sort_by(|basin1, basin2| basin2.len().cmp(&basin1.len()));
    basins.into_iter().take(n).collect()
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 1. Result: {}",
        sum_of_risk_levels(&data, |val| { val as usize + 1 })
    );
}

fn part_2_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 2. Result: {}",
        find_n_largest(&data, 3)
            .into_iter()
            .fold(1, |acc, elem| acc * elem.len())
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{find_n_largest, load_data, sum_of_risk_levels};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(sum_of_risk_levels(&data, |val| { val as usize + 1 }), 15);
    }

    #[test]
    fn part_2_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(
            find_n_largest(&data, 3)
                .into_iter()
                .fold(1, |acc, elem| acc * elem.len()),
            1134
        );
    }
}
