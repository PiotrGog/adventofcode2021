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

fn find_path_with_lowest_risk(data: &RisksMap) -> (Path, usize) {
    let mut cache = vec![vec![std::usize::MAX / 2; data.len()]; data[0].len()];
    let mut previous = vec![vec![None; data.len()]; data[0].len()];

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
            if x + 1 >= data.len() {
                None
            } else {
                Some((x + 1, y))
            },
            if y + 1 >= data[0].len() {
                None
            } else {
                Some((x, y + 1))
            },
        ]
        .into_iter()
        .filter_map(|v| v)
        .collect::<Path>()
    };

    cache[0][0] = 0;
    for x in 0..data.len() {
        for y in 0..data[0].len() {
            for (next_x, next_y) in adjacent(x, y) {
                if cache[next_x][next_y] as i64 > cache[x][y] as i64 + data[next_x][next_y] as i64 {
                    cache[next_x][next_y] = cache[x][y] + data[next_x][next_y];
                    previous[next_x][next_y] = Some((x, y));
                }
            }
        }
    }

    let mut result = LinkedList::new();
    let mut current_position = (data.len() - 1, data[0].len() - 1);
    let risk = cache[current_position.0][current_position.1];
    result.push_front(current_position);
    while let Some(position) = previous[current_position.0][current_position.1] {
        result.push_front(position);
        current_position = position;
    }

    (result.into_iter().collect(), risk)
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let (_, risk) = find_path_with_lowest_risk(&data);
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
        let (_, risk) = find_path_with_lowest_risk(&data);
        assert_eq!(risk, 40);
    }
}
