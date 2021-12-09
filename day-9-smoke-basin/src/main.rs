use std::{
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

fn part_1_result(file_name: &str) {
    let data = load_data(file_name);
    println!(
        "Part 1. Result: {}",
        sum_of_risk_levels(&data, |val| { val as usize + 1 })
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{load_data, sum_of_risk_levels};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let data = load_data(TEST_DATA_FILENAME);
        assert_eq!(sum_of_risk_levels(&data, |val| { val as usize + 1 }), 15);
    }
}
