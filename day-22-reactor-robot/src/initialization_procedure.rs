use std::collections::HashSet;

use crate::{cuboid::Cuboid, cuboids_range::CuboidsRange, reboot_step::RebootStep};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InitializationProcedure {
    pub steps: Vec<RebootStep>,
}

impl InitializationProcedure {
    pub fn new(steps: Vec<RebootStep>) -> Self {
        Self { steps }
    }

    pub fn run(&self, range: Option<CuboidsRange>) -> HashSet<Cuboid> {
        let mut current_cuboids = HashSet::new();

        for step in self.steps.iter().filter(|step| {
            if let Some(range) = &range {
                range.cuboid_in(step.cuboid())
            } else {
                true
            }
        }) {
            current_cuboids = Self::process_single_step(step, &current_cuboids);
        }

        current_cuboids
    }

    fn process_single_step(
        step: &RebootStep,
        current_cuboids: &HashSet<Cuboid>,
    ) -> HashSet<Cuboid> {
        let (on, new_cuboid) = match step {
            RebootStep::On(cuboid) => (true, cuboid),
            RebootStep::Off(cuboid) => (false, cuboid),
        };
        let mut result = HashSet::new();
        if current_cuboids.is_empty() && on {
            result.insert(new_cuboid.clone());
        }

        for current_cuboid in current_cuboids {
            if on {
                if current_cuboid.overlaps(&new_cuboid) {
                    result.extend(current_cuboid.split(&new_cuboid));
                } else {
                    result.insert(current_cuboid.clone());
                }
                result.insert(new_cuboid.clone());
            } else {
                if current_cuboid.overlaps(&new_cuboid) {
                    result.extend(current_cuboid.split(&new_cuboid));
                } else {
                    result.insert(current_cuboid.clone());
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::reboot_step::RebootStep;

    use super::*;

    #[test]
    fn test_process_single_step() {
        let instruction_1 = "on x=10..12,y=10..12,z=10..12";
        let instruction_2 = "on x=11..13,y=11..13,z=11..13";
        let instruction_3 = "off x=9..11,y=9..11,z=9..11";
        let instruction_4 = "on x=10..10,y=10..10,z=10..10";

        let current_cuboids = HashSet::new();

        let step_1 = RebootStep::from_str(instruction_1).unwrap();
        let current_cuboids =
            InitializationProcedure::process_single_step(&step_1, &current_cuboids);
        let result = current_cuboids
            .iter()
            .fold(0, |sum, cuboid| sum + cuboid.count_cubes_on());
        let expected_result = 27;
        assert_eq!(result, expected_result);

        let step_2 = RebootStep::from_str(instruction_2).unwrap();
        let current_cuboids =
            InitializationProcedure::process_single_step(&step_2, &current_cuboids);
        let result = current_cuboids
            .iter()
            .fold(0, |sum, cuboid| sum + cuboid.count_cubes_on());
        let expected_result = expected_result + 19;
        assert_eq!(result, expected_result);

        let step_3 = RebootStep::from_str(instruction_3).unwrap();
        let current_cuboids =
            InitializationProcedure::process_single_step(&step_3, &current_cuboids);
        let result = current_cuboids
            .iter()
            .fold(0, |sum, cuboid| sum + cuboid.count_cubes_on());
        let expected_result = expected_result - 8;
        assert_eq!(result, expected_result);

        let step_4 = RebootStep::from_str(instruction_4).unwrap();
        let current_cuboids =
            InitializationProcedure::process_single_step(&step_4, &current_cuboids);
        let result = current_cuboids
            .iter()
            .fold(0, |sum, cuboid| sum + cuboid.count_cubes_on());
        let expected_result = 39;
        assert_eq!(result, expected_result);
    }
}
