use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
};

type RisksMap = Vec<Vec<usize>>;
type Path = Vec<(usize, usize)>;

fn load_data(file_name: &str) -> RisksMap {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    file.lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn find_path_with_lowest_risk(data: &RisksMap, repeat_map: usize) -> (Path, usize) {
    let mut cache =
        vec![vec![std::usize::MAX / 2; data.len() * repeat_map]; data[0].len() * repeat_map];
    let mut previous = vec![vec![None; data.len() * repeat_map]; data[0].len() * repeat_map];

    let adjacent = |x, y| {
        vec![
            if x as i64 - 1 < 0 {
                None
            } else {
                Some((x - 1, y))
            },
            if y as i64 - 1 < 0 {
                None
            } else {
                Some((x, y - 1))
            },
            if x + 1 >= data.len() * repeat_map {
                None
            } else {
                Some((x + 1, y))
            },
            if y + 1 >= data[0].len() * repeat_map {
                None
            } else {
                Some((x, y + 1))
            },
        ]
        .into_iter()
        .filter_map(|v| v)
        .collect::<Path>()
    };

    let normalize_adjacent = |x, y| (x % data.len(), y % data[0].len());

    cache[0][0] = 0;
    for x in 0..data.len() * repeat_map {
        for y in 0..data[0].len() * repeat_map {
            for (next_x, next_y) in adjacent(x, y) {
                let (normalized_x, normalized_y) = normalize_adjacent(next_x, next_y);
                let risk_value = (data[normalized_x][normalized_y]
                    + ((next_x / data.len()) + (next_y / data[0].len())))
                    % 10;
                // let risk_value = if risk_value > 9 { 1 } else { risk_value };
                let risk_value = if risk_value == 0 { 1 } else { risk_value };
                if next_x == 23 && next_y == 22 {
                    println!("{} {} {}", next_x, next_y, risk_value);
                }
                if cache[next_x][next_y] as i64 > cache[x][y] as i64 + risk_value as i64 {
                    cache[next_x][next_y] = cache[x][y] + risk_value;
                    previous[next_x][next_y] = Some((x, y));
                }
            }
        }
    }

    let mut result = LinkedList::new();
    let mut current_position = (
        (data.len() * repeat_map) - 1,
        (data[0].len() * repeat_map) - 1,
    );
    let risk = cache[current_position.0][current_position.1];
    result.push_front(current_position);
    while let Some(position) = previous[current_position.0][current_position.1] {
        result.push_front(position);
        current_position = position;
    }
    println!("{:?}", cache);
    (result.into_iter().collect(), risk)
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let (_, risk) = find_path_with_lowest_risk(&data, 1);
    println!("Part 1. Result: {}", risk);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{find_path_with_lowest_risk, load_data};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let (_, risk) = find_path_with_lowest_risk(&data, 1);
        assert_eq!(risk, 40);
    }

    #[test]
    fn part_2_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let (_, risk) = find_path_with_lowest_risk(&data, 5);
        assert_eq!(risk, 315);
    }
}
