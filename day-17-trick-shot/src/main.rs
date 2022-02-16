#[derive(Clone, Copy, Debug)]
struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

fn part_1_result(target_area: TargetArea) {
    // y_n=V_y_0 * n - sum_0^n i
    // x
}

fn main() {
    // target area: x=240..292, y=-90..-57
    let target_area = TargetArea {
        x: (240, 292),
        y: (-90, -57),
    };
    part_1_result(target_area.clone());
}

#[cfg(test)]
mod tests {
    use crate::TargetArea;

    #[test]
    fn part_1_test_data() {
        // target area: x=20..30, y=-10..-5
        let target_area = TargetArea {
            x: (20, 30),
            y: (-10, -5),
        };
    }
}
