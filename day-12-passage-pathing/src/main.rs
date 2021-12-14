use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
};

fn load_data(file_name: &str) -> HashMap<String, HashSet<String>> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    for line in file.lines() {
        let line = line.unwrap();
        let points = line.split('-').collect::<Vec<_>>();
        if let Some(val) = result.get_mut(points[0]) {
            val.insert(points[1].to_string());
        } else {
            result.insert(
                points[0].to_string(),
                HashSet::from_iter([points[1].to_string()]),
            );
        }

        if let Some(val) = result.get_mut(points[1]) {
            val.insert(points[0].to_string());
        } else {
            result.insert(
                points[1].to_string(),
                HashSet::from_iter([points[0].to_string()]),
            );
        }
    }

    result
}

fn find_paths(data: HashMap<String, HashSet<String>>) -> Vec<Vec<String>> {
    fn rec_find_path(
        stop: String,
        data: HashMap<String, HashSet<String>>,
        current_paths: Vec<Vec<String>>,
    ) -> Vec<Vec<String>> {
        let mut paths = vec![];

        for path in current_paths {
            let last_point = path.last().unwrap();
            if *last_point == stop {
                paths.push(path)
            } else {
                let mut data_copy = data.clone();
                if last_point.to_lowercase() == *last_point {
                    data_copy.remove(last_point);
                }
                if let Some(points) = data.get(last_point) {
                    for point in points {
                        let mut new_path = path.clone();
                        new_path.push(point.clone());
                        paths.append(&mut rec_find_path(
                            stop.clone(),
                            data_copy.clone(),
                            vec![new_path],
                        ));
                    }
                }
            }
        }

        paths
    }

    rec_find_path("end".to_string(), data, vec![vec!["start".to_string()]])
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    let paths = find_paths(data);
    println!("Part 1. Result: {}", paths.len());
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{find_paths, load_data};

    #[test]
    fn part_1_a_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let mut expected = vec![
            vec![
                "start".to_string(),
                "A".to_string(),
                "b".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "b".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "b".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "b".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "b".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec!["start".to_string(), "A".to_string(), "end".to_string()],
            vec![
                "start".to_string(),
                "b".to_string(),
                "A".to_string(),
                "c".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "b".to_string(),
                "A".to_string(),
                "end".to_string(),
            ],
            vec!["start".to_string(), "b".to_string(), "end".to_string()],
        ];
        expected.sort();
        let mut actual = find_paths(data);

        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn part_1_b_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data_2.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let mut expected = [
            vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec!["start".to_string(), "HN".to_string(), "end".to_string()],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "dc".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec!["start".to_string(), "dc".to_string(), "end".to_string()],
            vec![
                "start".to_string(),
                "dc".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "dc".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "kj".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "kj".to_string(),
                "dc".to_string(),
                "HN".to_string(),
                "end".to_string(),
            ],
            vec![
                "start".to_string(),
                "kj".to_string(),
                "dc".to_string(),
                "end".to_string(),
            ],
        ];
        expected.sort();
        let mut actual = find_paths(data);

        actual.sort();
        assert_eq!(actual, expected);
    }
}
