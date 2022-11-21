use crate::cuboid::Cuboid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CuboidsRange {
    pub x: (isize, isize),
    pub y: (isize, isize),
    pub z: (isize, isize),
}

impl CuboidsRange {
    pub fn new(x: (isize, isize), y: (isize, isize), z: (isize, isize)) -> Self {
        Self { x, y, z }
    }

    pub fn cuboid_in(&self, cuboid: &Cuboid) -> bool {
        let CuboidsRange {
            x: (x_min, x_max),
            y: (y_min, y_max),
            z: (z_min, z_max),
        } = self;

        let Cuboid {
            x: (cuboid_x_min, cuboid_x_max),
            y: (cuboid_y_min, cuboid_y_max),
            z: (cuboid_z_min, cuboid_z_max),
        } = cuboid;

        cuboid_x_min >= x_min
            && cuboid_x_max <= x_max
            && cuboid_y_min >= y_min
            && cuboid_y_max <= y_max
            && cuboid_z_min >= z_min
            && cuboid_z_max <= z_max
    }
}
