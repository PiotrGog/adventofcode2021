use std::{
    fs::File,
    io::{BufRead, BufReader},
    collections::LinkedList,
    collections::HashMap,
};

type Line = Vec<char>;

#[derive(Debug, PartialEq, Clone)]
enum LineState {
    Correct,
    Incomplete(Line),
    Corrupted(char)
}


fn load_data(file_name: &str) -> Vec<Line> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    file.lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn classify_lines(data: &Vec<Line>) -> Vec<LineState> {
    let opening_to_clossing_chars = HashMap::from([('(', ')'), ('{', '}'), ('<', '>'), ('[', ']')]);
    data.iter().map(|line|{
        let mut state = LinkedList::new();
        for c in line.iter() {
            match c {
                closing_char if [')', '}', '>', ']'].into_iter().any(|c| c == *closing_char) => {
                    if let Some(opening_char) = state.pop_front() {
                        if opening_to_clossing_chars[opening_char] != *closing_char {
                            return LineState::Corrupted(*closing_char);
                        }
                    }
                },
                opening_char => {
                    state.push_front(opening_char);
                }
            }
        }

        if state.is_empty() { LineState::Correct } else { LineState::Incomplete(
            state.into_iter().map(|c| opening_to_clossing_chars[c]).collect()
        ) }
    }).collect()
}

fn score_symbol(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid corrupted symbol")
    }
}

fn score_line_closure(line: &Line) -> usize {
    line.iter().fold(0, |acc, c| {
        acc * 5 + match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Invalid corrupted symbol")
        }
    })
}

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 1. Result: {}",
        classify_lines(&data).iter().filter_map(|line_state| {
            if let LineState::Corrupted(c) = line_state {
                Some(score_symbol(*c))
            } else {
                None
            }
        }).sum::<usize>(),
    );
}


fn part_2_result(file_name: &str) {
    let data = load_data(file_name);
    let mut scored_incomplete_lines = classify_lines(&data).iter().filter_map(|line_state| {
        if let LineState::Incomplete(line) = line_state {
            Some(score_line_closure(line))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    scored_incomplete_lines.sort_by(|a, b| b.cmp(a));
    println!(
        "Part 2. Result: {}",
        scored_incomplete_lines[scored_incomplete_lines.len()/2]
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{load_data, classify_lines, LineState, score_symbol, score_line_closure};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let clasified_lines = classify_lines(&data);

        println!("{:?}", clasified_lines);
        assert_eq!(
            clasified_lines.iter().filter_map(|line_state| {
                if let LineState::Corrupted(c) = line_state {
                    Some(score_symbol(*c))
                } else {
                    None
                }
            }).sum::<usize>(),
            26397
        );
    }

    #[test]
    fn part_2_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        let mut scored_incomplete_lines = classify_lines(&data).iter().filter_map(|line_state| {
            if let LineState::Incomplete(line) = line_state {
                Some(score_line_closure(line))
            } else {
                None
            }
        }).collect::<Vec<_>>();
        scored_incomplete_lines.sort_by(|a, b| b.cmp(a));
        assert_eq!(scored_incomplete_lines[scored_incomplete_lines.len()/2], 288957);
    }
}
