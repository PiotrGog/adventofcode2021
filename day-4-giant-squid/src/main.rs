use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

const BOARD_SIZE: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Board {
    id: u32,
    board: [[u32; BOARD_SIZE]; BOARD_SIZE],
    value_drawn: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn get_value_idx(&self, val: u32) -> Option<(usize, usize)> {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == val {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn set_as_drawn(&mut self, idx: (usize, usize)) {
        self.value_drawn[idx.0][idx.1] = true;
    }

    fn check_column(&self, column: usize) -> bool {
        for i in 0..BOARD_SIZE {
            if !self.value_drawn[i][column] {
                return false;
            }
        }
        true
    }

    fn check_row(&self, row: usize) -> bool {
        for i in 0..BOARD_SIZE {
            if !self.value_drawn[row][i] {
                return false;
            }
        }
        true
    }

    fn calculate_score(&self) -> (u32, u32) {
        let mut sum_unmarked = 0;
        let mut sum_marked = 0;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.value_drawn[i][j] {
                    sum_marked += self.board[i][j];
                } else {
                    sum_unmarked += self.board[i][j];
                }
            }
        }
        (sum_unmarked, sum_marked)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Data {
    pub numbers: Vec<u32>,
    pub boards: Vec<Board>,
}

fn load_data(file_name: &str) -> Data {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    let mut lines = file.lines().peekable();
    let numbers = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|val| val.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];
    let mut id = 0;
    while let Some(line) = lines.peek() {
        match line.as_ref().unwrap().as_str() {
            "" => {
                lines.next();
            }
            _ => {
                let mut board = Board {
                    id,
                    board: [[0; BOARD_SIZE]; BOARD_SIZE],
                    value_drawn: [[false; BOARD_SIZE]; BOARD_SIZE],
                };
                for i in 0..BOARD_SIZE {
                    for (j, value) in lines
                        .next()
                        .unwrap()
                        .unwrap()
                        .split_whitespace()
                        .filter_map(|val| val.parse().ok())
                        .enumerate()
                    {
                        board.board[i][j] = value;
                    }
                }
                id += 1;
                boards.push(board);
            }
        }
    }

    Data { numbers, boards }
}

fn play(data: &mut Data) -> Option<(usize, u32)> {
    for number_to_check in &data.numbers {
        for (id, board) in data.boards.iter_mut().enumerate() {
            if let Some(idx) = board.get_value_idx(*number_to_check) {
                board.set_as_drawn(idx);
                if board.check_row(idx.0) || board.check_column(idx.1) {
                    return Some((id, *number_to_check));
                }
            }
        }
    }
    None
}

fn part_1_result(file_name: &str) {
    let mut data = load_data(file_name);
    let (board_id, last_number) = play(&mut data).unwrap();
    println!(
        "Part 1. Result: {}",
        last_number * data.boards[board_id].calculate_score().0
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{load_data, play};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let mut data = load_data(TEST_DATA_FILENAME);
        assert_eq!(play(&mut data), Some((2, 24)));
    }
}
