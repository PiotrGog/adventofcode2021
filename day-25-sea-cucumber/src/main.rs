use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
};

fn load_data(file_name: &str) -> (HashSet<SeaCucumber>, AreaSize) {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    let mut area_size = AreaSize { x: 0, y: 0 };
    (
        file.lines()
            .enumerate()
            .map(|(y, r_line)| {
                r_line
                    .unwrap()
                    .chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        area_size.x = area_size.x.max(x + 1);
                        area_size.y = area_size.y.max(y + 1);
                        match c {
                            '>' => Some(SeaCucumber::new_east_facing(x, y)),
                            'v' => Some(SeaCucumber::new_south_facing(x, y)),
                            '.' => None,
                            _ => panic!(""),
                        }
                    })
                    .collect::<HashSet<_>>()
            })
            .fold(HashSet::new(), |mut acc, partial_result| {
                acc.extend(partial_result);
                acc
            }),
        area_size,
    )
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AreaSize {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialOrd, Ord)]
pub enum SeaCucumber {
    EastFacing { x: usize, y: usize },
    SouthFacing { x: usize, y: usize },
}

impl PartialEq for SeaCucumber {
    fn eq(&self, other: &Self) -> bool {
        let self_unpack = match self {
            SeaCucumber::EastFacing { x, y } => (x, y),
            SeaCucumber::SouthFacing { x, y } => (x, y),
        };
        let other_unpack = match other {
            SeaCucumber::EastFacing { x, y } => (x, y),
            SeaCucumber::SouthFacing { x, y } => (x, y),
        };

        self_unpack == other_unpack
    }
}

impl Hash for SeaCucumber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SeaCucumber::EastFacing { x, y } => (x, y).hash(state),
            SeaCucumber::SouthFacing { x, y } => (x, y).hash(state),
        }
    }
}

impl SeaCucumber {
    fn new_east_facing(x: usize, y: usize) -> Self {
        SeaCucumber::EastFacing { x, y }
    }

    fn new_south_facing(x: usize, y: usize) -> Self {
        SeaCucumber::SouthFacing { x, y }
    }

    fn move_forward(self, area_size: &AreaSize) -> Self {
        match self {
            SeaCucumber::EastFacing { x, y } => Self::new_east_facing((x + 1) % area_size.x, y),
            SeaCucumber::SouthFacing { x, y } => Self::new_south_facing(x, (y + 1) % area_size.y),
        }
    }
}

mod solver {
    use crate::{AreaSize, SeaCucumber};

    use std::collections::HashSet;

    pub fn solve(
        mut sea_cucumbers: HashSet<SeaCucumber>,
        area_size: &AreaSize,
    ) -> (HashSet<SeaCucumber>, usize) {
        for iter_num in 1.. {
            let next_iteration = self::iteration(sea_cucumbers.clone(), &area_size);
            if next_iteration == sea_cucumbers {
                return (next_iteration, iter_num);
            }
            sea_cucumbers = next_iteration;
        }

        (HashSet::new(), 0)
    }

    fn iteration(
        sea_cucumbers: HashSet<SeaCucumber>,
        area_size: &AreaSize,
    ) -> HashSet<SeaCucumber> {
        let (east_facing_sea_cucumbers, south_facing_sea_cucumbers) = sea_cucumbers.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut east_facing, mut south_facing), sea_cucumber| {
                match sea_cucumber {
                    SeaCucumber::EastFacing { .. } => east_facing.push(sea_cucumber),
                    SeaCucumber::SouthFacing { .. } => south_facing.push(sea_cucumber),
                };
                (east_facing, south_facing)
            },
        );

        let sea_cucumbers =
            move_and_update(&east_facing_sea_cucumbers, sea_cucumbers.clone(), area_size);
        move_and_update(&south_facing_sea_cucumbers, sea_cucumbers, area_size)
    }

    fn move_and_update(
        facing_sea_cucumbers: &Vec<&SeaCucumber>,
        mut sea_cucumbers: HashSet<SeaCucumber>,
        area_size: &AreaSize,
    ) -> HashSet<SeaCucumber> {
        let mut moved = Vec::new();
        for sea_cucumber in facing_sea_cucumbers {
            if sea_cucumbers.insert(sea_cucumber.move_forward(area_size)) {
                moved.push(sea_cucumber);
            }
        }
        moved.into_iter().for_each(|moved_cucumber| {
            sea_cucumbers.remove(moved_cucumber);
        });

        sea_cucumbers
    }
}
fn part_1_result(file_name: &str) {
    let (sea_cucumbers, area_size) = load_data(file_name);
    let result = solver::solve(sea_cucumbers, &area_size);

    println!("Part 1. Result: {:?}", result.1);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_sea_cucumber_in_set() {
        let mut set = HashSet::new();
        set.insert(SeaCucumber::new_east_facing(1, 1));
        assert_eq!(set.len(), 1);
        set.insert(SeaCucumber::new_south_facing(1, 1));
        assert_eq!(set.len(), 1);
        set.insert(SeaCucumber::new_south_facing(1, 2));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_load_data() {
        let sea_cucumbers = load_data("./resources/test_load_data.txt");
        let expected = (
            HashSet::from_iter(vec![
                SeaCucumber::new_east_facing(3, 0),
                SeaCucumber::new_east_facing(6, 2),
                SeaCucumber::new_south_facing(0, 3),
                SeaCucumber::new_east_facing(6, 3),
                SeaCucumber::new_east_facing(6, 4),
                SeaCucumber::new_south_facing(2, 6),
                SeaCucumber::new_south_facing(3, 6),
                SeaCucumber::new_south_facing(4, 6),
            ]),
            AreaSize { x: 7, y: 7 },
        );
        assert_eq!(sea_cucumbers, expected);
    }

    #[test]
    fn test_part_1() {
        let (sea_cucumbers, area_size) = load_data("./resources/test_data.txt");
        let result = solver::solve(sea_cucumbers, &area_size);
        assert_eq!(result.1, 58);
    }
}
