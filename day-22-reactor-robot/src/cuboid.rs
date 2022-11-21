use crate::parse_reboot_step_error::ParseRebootStepError;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Cuboid {
    pub x: (isize, isize),
    pub y: (isize, isize),
    pub z: (isize, isize),
}

impl Cuboid {
    pub fn new(x: (isize, isize), y: (isize, isize), z: (isize, isize)) -> Self {
        Self { x, y, z }
    }

    pub fn count_cubes_on(&self) -> usize {
        (self.x.1 - self.x.0 + 1).abs() as usize
            * (self.y.1 - self.y.0 + 1).abs() as usize
            * (self.z.1 - self.z.0 + 1).abs() as usize
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let Cuboid {
            x: (x_min, x_max),
            y: (y_min, y_max),
            z: (z_min, z_max),
        } = self;

        let Cuboid {
            x: (o_x_min, o_x_max),
            y: (o_y_min, o_y_max),
            z: (o_z_min, o_z_max),
        } = other;

        let overlaps_helper = |value_min, value_max, other_value_min, other_value_max| {
            (other_value_min >= value_min && other_value_min <= value_max)
                || (other_value_max >= value_min && other_value_max <= value_max)
                || (value_min >= other_value_min && value_min <= other_value_max)
                || (value_max >= other_value_min && value_max <= other_value_max)
        };

        let x_overlaps = overlaps_helper(x_min, x_max, o_x_min, o_x_max);
        let y_overlaps = overlaps_helper(y_min, y_max, o_y_min, o_y_max);
        let z_overlaps = overlaps_helper(z_min, z_max, o_z_min, o_z_max);

        x_overlaps && y_overlaps && z_overlaps
    }

    pub fn split(&self, other: &Self) -> HashSet<Self> {
        let mut result = HashSet::new();
        let Cuboid {
            x: (x_min, x_max),
            y: (y_min, y_max),
            z: (z_min, z_max),
        } = self;

        let Cuboid {
            x: (o_x_min, o_x_max),
            y: (o_y_min, o_y_max),
            z: (o_z_min, o_z_max),
        } = other;

        let cut = |value_min: &isize,
                   value_max: &isize,
                   other_value_min: &isize,
                   other_value_max: &isize| {
            let mut cut_result = vec![*value_min];
            if other_value_min > value_min && other_value_min <= value_max {
                cut_result.push(*other_value_min - 1);
                cut_result.push(*other_value_min);
            }
            if other_value_max >= value_min && other_value_max < value_max {
                cut_result.push(*other_value_max);
                cut_result.push(*other_value_max + 1);
            }
            cut_result.push(*value_max);
            cut_result
        };

        let cut_x = cut(x_min, x_max, o_x_min, o_x_max);
        let cut_y = cut(y_min, y_max, o_y_min, o_y_max);
        let cut_z = cut(z_min, z_max, o_z_min, o_z_max);

        for x in cut_x.chunks(2) {
            for y in cut_y.chunks(2) {
                for z in cut_z.chunks(2) {
                    let new_cuboid = Self::new((x[0], x[1]), (y[0], y[1]), (z[0], z[1]));
                    if !other.overlaps(&new_cuboid) {
                        result.insert(new_cuboid);
                    }
                }
            }
        }

        result
    }
}

impl FromStr for Cuboid {
    type Err = ParseRebootStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(",");
        let (x_min, x_max) = iter
            .next()
            .and_then(|s| {
                s.strip_prefix("x=")
                    .and_then(|s_without_prefix| s_without_prefix.split_once(".."))
            })
            .ok_or(ParseRebootStepError(
                "Can't parse x coordinate field".to_string(),
            ))?;

        let (y_min, y_max) = iter
            .next()
            .and_then(|s| {
                s.strip_prefix("y=")
                    .and_then(|s_without_prefix| s_without_prefix.split_once(".."))
            })
            .ok_or(ParseRebootStepError(
                "Can't parse y coordinate field".to_string(),
            ))?;

        let (z_min, z_max) = iter
            .next()
            .and_then(|s| {
                s.strip_prefix("z=")
                    .and_then(|s_without_prefix| s_without_prefix.split_once(".."))
            })
            .ok_or(ParseRebootStepError(
                "Can't parse z coordinate field".to_string(),
            ))?;

        let x = (x_min.parse().unwrap(), x_max.parse().unwrap());
        let y = (y_min.parse().unwrap(), y_max.parse().unwrap());
        let z = (z_min.parse().unwrap(), z_max.parse().unwrap());
        let coordinate_range = Cuboid { x, y, z };

        Ok(coordinate_range)
    }
}
