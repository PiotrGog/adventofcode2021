use std::str::FromStr;

use crate::{cuboid::Cuboid, parse_reboot_step_error::ParseRebootStepError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RebootStep {
    On(Cuboid),
    Off(Cuboid),
}

impl RebootStep {
    pub fn new_on(cuboid: Cuboid) -> Self {
        Self::On(cuboid)
    }
    pub fn new_off(cuboid: Cuboid) -> Self {
        Self::Off(cuboid)
    }

    pub fn cuboid(&self) -> &Cuboid {
        match self {
            Self::On(cuboid) => cuboid,
            Self::Off(cuboid) => cuboid,
        }
    }
}

impl FromStr for RebootStep {
    type Err = ParseRebootStepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        let instruction = iter.next().ok_or(ParseRebootStepError(
            "Can't get instruction field (on/off)".to_string(),
        ))?;

        let coordinate_range = iter
            .next()
            .ok_or(ParseRebootStepError(
                "Can't get coordinates field".to_string(),
            ))?
            .parse()?;

        Ok(match instruction {
            "on" => Self::On(coordinate_range),
            "off" => Self::Off(coordinate_range),
            _ => return Err(ParseRebootStepError("Can't parse instruction".to_string())),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{cuboid::Cuboid, reboot_step::RebootStep};

    #[test]
    fn test_reboot_step_from_str_on() {
        let string_repr = "on x=-20..26,y=-36..17,z=-47..7";
        let parse_result = string_repr.parse::<RebootStep>().unwrap();
        assert_eq!(
            parse_result,
            RebootStep::On(Cuboid {
                x: (-20, 26),
                y: (-36, 17),
                z: (-47, 7),
            })
        );
    }
}
