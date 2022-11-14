use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn load_data(file_name: &str) -> ImageWithEnhanceAlgorithm {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    fn convert_sign_into_num(c: char) -> u8 {
        match c {
            '.' => 0,
            '#' => 1,
            _ => panic!(),
        }
    }

    let mut lines = file.lines();
    let algorithm_data = lines.next().unwrap().unwrap();
    let enhance_algorithm = EnhanceAlgorithm {
        algorithm: algorithm_data
            .chars()
            .into_iter()
            .map(convert_sign_into_num)
            .collect(),
    };

    let consumed_empty_line = lines.next();
    if !consumed_empty_line.unwrap().unwrap().is_empty() {
        panic!()
    }

    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;

    let mut pixels = HashSet::new();
    for (x, line) in lines.enumerate() {
        for (y, value) in line.unwrap().chars().enumerate() {
            if convert_sign_into_num(value) == 1 {
                let x = x as isize;
                let y = y as isize;
                pixels.insert((x, y));
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    let image = Image {
        pixels,
        min_x,
        max_x,
        min_y,
        max_y,
        count_lit: true,
    };

    ImageWithEnhanceAlgorithm {
        image,
        enhance_algorithm,
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Image {
    pixels: HashSet<(isize, isize)>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    count_lit: bool,
}

impl Image {
    pub fn process_with_algorithm(&self, algorithm: &EnhanceAlgorithm) -> Self {
        let mut result_image = Image {
            pixels: HashSet::new(),
            min_x: isize::MAX,
            max_x: isize::MIN,
            min_y: isize::MAX,
            max_y: isize::MIN,
            count_lit: if algorithm.algorithm[0] == 1 {
                !self.count_lit
            } else {
                true
            },
        };
        for x in (self.min_x - 1)..=(self.max_x + 1) {
            for y in (self.min_y - 1)..=(self.max_y + 1) {
                let value_from_subimage = self.get_value_from_subimage(x, y);
                if (result_image.count_lit && algorithm.algorithm[value_from_subimage] == 1)
                    || (!result_image.count_lit && algorithm.algorithm[value_from_subimage] == 0)
                {
                    result_image.pixels.insert((x, y));
                    result_image.min_x = result_image.min_x.min(x);
                    result_image.max_x = result_image.max_x.max(x);
                    result_image.min_y = result_image.min_y.min(y);
                    result_image.max_y = result_image.max_y.max(y);
                }
            }
        }
        result_image
    }

    pub fn count_lit_pixels(&self) -> Option<usize> {
        if self.count_lit {
            Some(self.pixels.len())
        } else {
            None
        }
    }

    pub fn process_with_algorithm_n_times(&self, algorithm: &EnhanceAlgorithm, n: usize) -> Self {
        let mut image = self.clone();
        for _ in 0..n {
            image = image.process_with_algorithm(algorithm);
        }

        image
    }

    fn get_value_from_subimage(&self, x: isize, y: isize) -> usize {
        let mut result = 0usize;

        for current_x in (x - 1)..=(x + 1) {
            for current_y in (y - 1)..=(y + 1) {
                result = (result << 1)
                    | if (self.count_lit && self.pixels.contains(&(current_x, current_y)))
                        || (!self.count_lit && !self.pixels.contains(&(current_x, current_y)))
                    {
                        1
                    } else {
                        0
                    };
            }
        }

        result
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct EnhanceAlgorithm {
    algorithm: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq)]
struct ImageWithEnhanceAlgorithm {
    image: Image,
    enhance_algorithm: EnhanceAlgorithm,
}

fn part_1_result(file_name: &str) {
    let ImageWithEnhanceAlgorithm {
        image,
        enhance_algorithm: algorithm,
    } = load_data(file_name);
    let lit_pixels = image
        .process_with_algorithm_n_times(&algorithm, 2)
        .count_lit_pixels()
        .unwrap();

    println!("Part 1. Result: {}", lit_pixels);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let ImageWithEnhanceAlgorithm {
            image,
            enhance_algorithm: algorithm,
        } = load_data("resources/test_data.txt");
        let result = image
            .process_with_algorithm_n_times(&algorithm, 2)
            .count_lit_pixels()
            .unwrap();
        assert_eq!(result, 35);
    }
}
