struct Area {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Area {
    pub fn point_in(&self, (x, y): (i32, i32)) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
}

struct TrickShot {
    current_x_velocity: i32,
    current_y_velocity: i32,
    current_x: i32,
    current_y: i32,
}

impl TrickShot {
    pub fn new(initial_x_velocity: i32, initial_y_velocity: i32) -> Self {
        Self {
            current_x_velocity: initial_x_velocity,
            current_y_velocity: initial_y_velocity,
            current_x: 0,
            current_y: 0,
        }
    }

    fn step(&mut self) {
        self.current_x += self.current_x_velocity;
        self.current_y += self.current_y_velocity;
        self.current_x_velocity -= self.current_x_velocity.signum();
        self.current_y_velocity -= 1;
    }
}

struct Simulator {
    target_area: Area,
}

impl Simulator {
    pub fn run(&self, initial_x_velocity: i32, initial_y_velocity: i32) -> (bool, Vec<(i32, i32)>) {
        let mut shot = TrickShot::new(initial_x_velocity, initial_y_velocity);
        let mut trajectory = vec![];

        while !self.is_in_target_area(&shot) && !self.missed(&shot) {
            shot.step();
            trajectory.push((shot.current_x, shot.current_y));
        }

        (self.is_in_target_area(&shot), trajectory)
    }

    pub fn find_all_matching_trajectories(&self) -> Vec<Vec<(i32, i32)>> {
        let mut trajectories = vec![];
        let minimal_initial_x = self.get_min_initial_x_velocity();
        let maximal_initial_x = self.get_max_initial_x_velocity();
        // the values for initial_y velocity are selected arbitrary
        let minimal_initial_y = -1000;
        let maximal_initial_y = 1000;
        for initial_x_velocity in minimal_initial_x..=maximal_initial_x {
            for initial_y_velocity in minimal_initial_y..=maximal_initial_y {
                let (succeeded, trajectory) = self.run(initial_x_velocity, initial_y_velocity);
                if succeeded {
                    trajectories.push(trajectory);
                }
            }
        }
        trajectories
    }

    fn is_in_target_area(&self, shot: &TrickShot) -> bool {
        self.target_area.point_in((shot.current_x, shot.current_y))
    }

    fn missed(&self, shot: &TrickShot) -> bool {
        self.current_x_is_greater_than_max_x(&shot)
            || self.current_y_is_less_than_min_y(&shot)
            || self.current_x_is_less_than_min_x_and_zero_x_velocity(&shot)
    }

    fn current_x_is_greater_than_max_x(&self, shot: &TrickShot) -> bool {
        shot.current_x > self.target_area.max_x
    }

    fn current_y_is_less_than_min_y(&self, shot: &TrickShot) -> bool {
        shot.current_y < self.target_area.min_y
    }

    fn current_x_is_less_than_min_x_and_zero_x_velocity(&self, shot: &TrickShot) -> bool {
        shot.current_x < self.target_area.min_x && shot.current_x_velocity == 0
    }

    fn get_min_initial_x_velocity(&self) -> i32 {
        (0..)
            .take_while(|x| x * (x + 1) / 2 < self.target_area.min_x)
            .last()
            .unwrap()
            + 1
    }

    fn get_max_initial_x_velocity(&self) -> i32 {
        self.target_area.max_x
    }
}

fn get_maximal_y_from_trajectories(trajectories: &Vec<Vec<(i32, i32)>>) -> i32 {
    trajectories
        .iter()
        .map(|trajectory| trajectory.iter().map(|(_x, y)| *y).max().unwrap())
        .max()
        .unwrap()
}

fn part_1_result(min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    let target_area = Area {
        min_x,
        max_x,
        min_y,
        max_y,
    };
    let simulator = Simulator { target_area };
    let trajectories = simulator.find_all_matching_trajectories();
    println!(
        "Part 1. Result: {}",
        get_maximal_y_from_trajectories(&trajectories)
    );
}

fn main() {
    const MIN_X: i32 = 240;
    const MAX_X: i32 = 292;
    const MIN_Y: i32 = -90;
    const MAX_Y: i32 = -57;
    part_1_result(MIN_X, MAX_X, MIN_Y, MAX_Y);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_AREA: Area = Area {
        min_x: 20,
        max_x: 30,
        min_y: -10,
        max_y: -5,
    };

    #[test]
    fn sample_data_initial_velocity_7_2_succeeded() {
        let simulator = Simulator {
            target_area: TARGET_AREA,
        };
        let initial_x_velocity = 7;
        let initial_y_velocity = 2;
        let (succeeded, _trajectory) = simulator.run(initial_x_velocity, initial_y_velocity);
        assert!(succeeded);
    }

    #[test]
    fn sample_data_initial_velocity_6_3_succeeded() {
        let simulator = Simulator {
            target_area: TARGET_AREA,
        };
        let initial_x_velocity = 6;
        let initial_y_velocity = 3;
        let (succeeded, _trajectory) = simulator.run(initial_x_velocity, initial_y_velocity);
        assert!(succeeded);
    }

    #[test]
    fn sample_data_initial_velocity_9_0_succeeded() {
        let simulator = Simulator {
            target_area: TARGET_AREA,
        };
        let initial_x_velocity = 9;
        let initial_y_velocity = 0;
        let (succeeded, _trajectory) = simulator.run(initial_x_velocity, initial_y_velocity);
        assert!(succeeded);
    }

    #[test]
    fn sample_data_initial_velocity_17_minus4_failed() {
        let simulator = Simulator {
            target_area: TARGET_AREA,
        };
        let initial_x_velocity = 17;
        let initial_y_velocity = -4;
        let (succeeded, _trajectory) = simulator.run(initial_x_velocity, initial_y_velocity);
        assert!(!succeeded);
    }

    #[test]
    fn test_part_1() {
        let simulator = Simulator {
            target_area: TARGET_AREA,
        };
        let trajectories = simulator.find_all_matching_trajectories();
        let maximal_y = get_maximal_y_from_trajectories(&trajectories);
        assert_eq!(maximal_y, 45);
    }
}
