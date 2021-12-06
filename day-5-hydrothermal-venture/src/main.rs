use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    string::ParseError,
};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(',').collect::<Vec<_>>();

        let x = s[0].parse().expect("Can't parse X");
        let y = s[1].parse().expect("Can't parse X");
        Ok(Point { x, y })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Line {
    point1: Point,
    point2: Point,
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" -> ").collect::<Vec<_>>();

        let point1 = Point::from_str(s[0]).expect("Can't parse the begining of the line");
        let point2 = Point::from_str(s[1]).expect("Can't parse the end of the line");
        Ok(Line { point1, point2 })
    }
}

fn load_data(file_name: &str) -> Vec<Line> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    file.lines()
        .map(|line| Line::from_str(line.unwrap().as_str()).unwrap())
        .collect()
}

fn is_horizontal_or_vertical(line: &Line) -> bool {
    let point1 = &line.point1;
    let point2 = &line.point2;
    point1.x == point2.x || point1.y == point2.y
}

fn get_dangerous_points_coords(data: &Vec<Line>) -> HashMap<(usize, usize), usize> {
    let mut points_counter = HashMap::new();

    for line in data {
        let Line { point1, point2 } = line;
        if point1.x == point2.x {
            for y in min(point1.y, point2.y)..=max(point1.y, point2.y) {
                let count = points_counter.entry((point1.x, y)).or_insert(0);
                *count += 1;
            }
        } else if point1.y == point2.y {
            for x in min(point1.x, point2.x)..=max(point1.x, point2.x) {
                let count = points_counter.entry((x, point1.y)).or_insert(0);
                *count += 1;
            }
        } else {
            let is_growing =
                (point1.x as i64 - point2.x as i64) * (point1.y as i64 - point2.y as i64) > 0;
            for (x, y) in (min(point1.x, point2.x)..=max(point1.x, point2.x)).zip(if is_growing {
                (min(point1.y, point2.y)..=max(point1.y, point2.y))
                    .collect::<Vec<_>>()
                    .into_iter()
            } else {
                (min(point1.y, point2.y)..=max(point1.y, point2.y))
                    .rev()
                    .collect::<Vec<_>>()
                    .into_iter()
            }) {
                let count = points_counter.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }

    points_counter
}

fn count_points_with_counter(counters: &HashMap<(usize, usize), usize>, threshold: usize) -> usize {
    counters
        .iter()
        .filter(|(_, &counter)| counter >= threshold)
        .count()
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let data = data
        .into_iter()
        .filter(is_horizontal_or_vertical)
        .collect::<Vec<_>>();
    let data = get_dangerous_points_coords(&data);
    println!("Part 1. Result: {}", count_points_with_counter(&data, 2));
}

fn part_2_result(file_name: &str) {
    let data = load_data(file_name);
    let data = get_dangerous_points_coords(&data);
    println!("Part 2. Result: {}", count_points_with_counter(&data, 2));
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{
        count_points_with_counter, get_dangerous_points_coords, is_horizontal_or_vertical,
        load_data,
    };

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let data = data
            .into_iter()
            .filter(is_horizontal_or_vertical)
            .collect::<Vec<_>>();
        let data = get_dangerous_points_coords(&data);
        assert_eq!(count_points_with_counter(&data, 2), 5);
    }

    #[test]
    fn part_2_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let data = get_dangerous_points_coords(&data);
        assert_eq!(count_points_with_counter(&data, 2), 12);
    }
}
