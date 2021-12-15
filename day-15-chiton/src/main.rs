use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

type RisksMap = Vec<Vec<usize>>;

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

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    risk: usize,
    coordinates: (usize, usize),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_adjacent(x: usize, y: usize, max_xy: usize) -> Vec<(usize, usize)> {
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
        if x + 1 >= max_xy {
            None
        } else {
            Some((x + 1, y))
        },
        if y + 1 >= max_xy {
            None
        } else {
            Some((x, y + 1))
        },
    ]
    .into_iter()
    .filter_map(|v| v)
    .collect::<Vec<(usize, usize)>>()
}

fn find_path_with_lowest_risk(data: &RisksMap) -> Option<usize> {
    let mut dist = vec![vec![std::usize::MAX; data.len()]; data[0].len()];

    let mut remaining_nodes = BinaryHeap::new();
    remaining_nodes.push(Node {
        risk: 0,
        coordinates: (0, 0),
    });

    while let Some(Node { risk, coordinates }) = remaining_nodes.pop() {
        if coordinates == (data.len() - 1, data.len() - 1) {
            return Some(risk);
        }

        if risk > dist[coordinates.0][coordinates.1] {
            continue;
        }

        for adjecent in get_adjacent(coordinates.0, coordinates.1, data.len()) {
            let new_node = Node {
                risk: risk + data[adjecent.0][adjecent.1],
                coordinates: adjecent,
            };

            if new_node.risk < dist[adjecent.0][adjecent.1] {
                remaining_nodes.push(new_node);

                dist[adjecent.0][adjecent.1] = new_node.risk;
            }
        }
    }

    None
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let risk = find_path_with_lowest_risk(&data);
    println!("Part 1. Result: {}", risk.unwrap());
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let risk = find_path_with_lowest_risk(&data).unwrap();
        assert_eq!(risk, 40);
    }
}
