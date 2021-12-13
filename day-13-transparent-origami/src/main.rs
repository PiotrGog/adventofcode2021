use regex::Regex;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
enum Fold {
    X(usize),
    Y(usize),
}

fn load_data(file_name: &str) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);
    let mut folds = vec![];
    let mut dots = HashSet::new();

    let dot_regex = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let fold_regex = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();
    file.lines().for_each(|line| {
        let line = line.unwrap();
        if let Some(m) = dot_regex.captures(&line) {
            dots.insert((
                m.get(1).unwrap().as_str().parse().unwrap(),
                m.get(2).unwrap().as_str().parse().unwrap(),
            ));
        } else if let Some(m) = fold_regex.captures(&line) {
            folds.push(match m.get(1).unwrap().as_str() {
                "y" => Fold::Y(m.get(2).unwrap().as_str().parse().unwrap()),
                "x" => Fold::X(m.get(2).unwrap().as_str().parse().unwrap()),
                _ => panic!("Incorrect fold value"),
            });
        }
    });
    (dots, folds)
}

fn fold(dots: HashSet<(usize, usize)>, fold: Fold) -> HashSet<(usize, usize)> {
    match fold {
        Fold::X(x) => dots
            .into_iter()
            .filter_map(|dot| {
                if dot.0 > x * 2 {
                    None
                } else if dot.0 < x {
                    Some(dot)
                } else {
                    Some((2 * x - dot.0, dot.1))
                }
            })
            .collect(),
        Fold::Y(y) => dots
            .into_iter()
            .filter_map(|dot| {
                if dot.1 > y * 2 {
                    None
                } else if dot.1 < y {
                    Some(dot)
                } else {
                    Some((dot.0, 2 * y - dot.1))
                }
            })
            .collect(),
    }
}

fn print_origami_result(dots: &HashSet<(usize, usize)>) {
    let (mut max_x, mut max_y) = (0, 0);

    for (x, y) in dots {
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    let mut result_code = vec![vec![' '; max_x + 1]; max_y + 1];
    for (x, y) in dots {
        result_code[*y][*x] = '#';
    }

    result_code.into_iter().for_each(|x| println!("{:?}", x));
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 1. Result: {}",
        fold(data.0, data.1[0]).into_iter().count()
    );
}

fn part_2_result(file_name: &str) {
    let (dots, folds) = load_data(file_name);
    let dots = folds
        .into_iter()
        .fold(dots, |dots, fold_instruction| fold(dots, fold_instruction));

    println!("Part 2. Result:");
    print_origami_result(&dots);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, iter::FromIterator};

    use crate::{fold, load_data};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(
            fold(data.0.clone(), data.1[0]),
            HashSet::from_iter([
                (0, 0),
                (2, 0),
                (3, 0),
                (6, 0),
                (9, 0),
                (0, 1),
                (4, 1),
                (6, 2),
                (10, 2),
                (0, 3),
                (4, 3),
                (1, 4),
                (3, 4),
                (6, 4),
                (8, 4),
                (9, 4),
                (10, 4),
            ])
        );
    }
}
