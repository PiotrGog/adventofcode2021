use std::{
    collections::HashSet,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn load_data(file_name: &str) -> Vec<Instruction> {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    file.lines()
        .map(|r_line| r_line.unwrap().parse::<Instruction>().unwrap())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ArithmeticLogicUnit {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl ArithmeticLogicUnit {
    pub fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn execute<Input>(&mut self, instruction: &Instruction, input: &mut Input)
    where
        Input: Iterator<Item = isize>,
    {
        match instruction {
            Instruction::Inp(value) => {
                if let Some(input_value) = input.next() {
                    self.inp(value.as_str(), input_value);
                } else {
                    panic!("No more input values for inp operation");
                }
            }
            Instruction::Add(value_a, value_b) => self.add(value_a.as_str(), value_b.as_str()),
            Instruction::Mul(value_a, value_b) => self.mul(value_a.as_str(), value_b.as_str()),
            Instruction::Div(value_a, value_b) => self.div(value_a.as_str(), value_b.as_str()),
            Instruction::Mod(value_a, value_b) => self.modulo(value_a.as_str(), value_b.as_str()),
            Instruction::Eql(value_a, value_b) => self.eql(value_a.as_str(), value_b.as_str()),
        }
    }

    pub fn inp(&mut self, variable: &str, value: isize) {
        *self.get_register(variable) = value;
    }

    pub fn add(&mut self, variable_a: &str, variable_b: &str) {
        *self.get_register(variable_a) += self.get_register_or_value(variable_b);
    }

    pub fn mul(&mut self, variable_a: &str, variable_b: &str) {
        *self.get_register(variable_a) *= self.get_register_or_value(variable_b);
    }

    pub fn div(&mut self, variable_a: &str, variable_b: &str) {
        *self.get_register(variable_a) /= self.get_register_or_value(variable_b);
    }

    pub fn modulo(&mut self, variable_a: &str, variable_b: &str) {
        *self.get_register(variable_a) %= self.get_register_or_value(variable_b);
    }

    pub fn eql(&mut self, variable_a: &str, variable_b: &str) {
        let variable_a_value = *self.get_register(variable_a);
        *self.get_register(variable_a) =
            if variable_a_value == self.get_register_or_value(variable_b) {
                1
            } else {
                0
            }
    }

    fn get_register_or_value(&mut self, value: &str) -> isize {
        if let Ok(num) = value.parse::<isize>() {
            num
        } else {
            *self.get_register(value)
        }
    }

    fn get_register(&mut self, variable_name: &str) -> &mut isize {
        match variable_name {
            "w" => &mut self.w,
            "x" => &mut self.x,
            "y" => &mut self.y,
            "z" => &mut self.z,
            var => panic!("Invalid ALU's variable: {}", var),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParseInstructionError(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Inp(String),
    Add(String, String),
    Mul(String, String),
    Div(String, String),
    Mod(String, String),
    Eql(String, String),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted_instruction: Vec<&str> = s.split_ascii_whitespace().collect();
        match splitted_instruction[0] {
            "inp" => Ok(Self::Inp(splitted_instruction[1].to_string())),
            "add" => Ok(Self::Add(
                splitted_instruction[1].to_string(),
                splitted_instruction[2].to_string(),
            )),
            "mul" => Ok(Self::Mul(
                splitted_instruction[1].to_string(),
                splitted_instruction[2].to_string(),
            )),
            "div" => Ok(Self::Div(
                splitted_instruction[1].to_string(),
                splitted_instruction[2].to_string(),
            )),
            "mod" => Ok(Self::Mod(
                splitted_instruction[1].to_string(),
                splitted_instruction[2].to_string(),
            )),
            "eql" => Ok(Self::Eql(
                splitted_instruction[1].to_string(),
                splitted_instruction[2].to_string(),
            )),
            instruction => Err(ParseInstructionError(format!(
                "Unsupported instruction '{}'",
                instruction
            ))),
        }
    }
}

fn split_instructions_by_inp(mut instructions: Vec<Instruction>) -> Vec<Vec<Instruction>> {
    instructions.reverse();
    instructions
        .as_slice()
        .split_inclusive(|instruction| {
            if let &Instruction::Inp(_) = instruction {
                true
            } else {
                false
            }
        })
        .map(|grouped_instructions| grouped_instructions.into_iter().rev().cloned().collect())
        .rev()
        .collect()
}

fn calculate_number<ValuesGenerator>(
    alu: ArithmeticLogicUnit,
    grouped_instructions: &[&[Instruction]],
    model_number: isize,
    values_generator: &ValuesGenerator,
    bad_alu_states: &mut HashSet<(ArithmeticLogicUnit, usize)>,
) -> Option<isize>
where
    ValuesGenerator: IntoIterator<Item = isize> + Clone,
{
    if grouped_instructions.is_empty() {
        if is_model_number_valid(&alu) {
            return Some(model_number);
        } else {
            return None;
        }
    }

    let next_grouped_instructions = &grouped_instructions[1..];
    if bad_alu_states.contains(&(alu.clone(), next_grouped_instructions.len())) {
        return None;
    }

    let model_number = model_number * 10;
    for number in values_generator.clone().into_iter() {
        let mut alu_copy = alu.clone();
        let mut input = [number].into_iter();
        grouped_instructions[0]
            .iter()
            .for_each(|instruction| alu_copy.execute(instruction, &mut input));

        if let Some(calculated_number) = calculate_number(
            alu_copy,
            next_grouped_instructions,
            model_number + number,
            values_generator,
            bad_alu_states,
        ) {
            return Some(calculated_number);
        }
    }

    bad_alu_states.insert((alu.clone(), next_grouped_instructions.len()));

    None
}

fn is_model_number_valid(alu: &ArithmeticLogicUnit) -> bool {
    alu.z == 0
}

fn part_1_result(file_name: &str) {
    let instructions = load_data(file_name);

    let grouped_instructions = split_instructions_by_inp(instructions);
    let grouped_instructions = grouped_instructions
        .iter()
        .map(|instructions| instructions.as_slice())
        .collect::<Vec<_>>();
    let result = calculate_number(
        ArithmeticLogicUnit::new(),
        grouped_instructions.as_slice(),
        0,
        &(1..=9).rev(),
        &mut HashSet::new(),
    );

    println!("Part 1. Result: {:?}", result);
}

fn part_2_result(file_name: &str) {
    let instructions = load_data(file_name);

    let grouped_instructions = split_instructions_by_inp(instructions);
    let grouped_instructions = grouped_instructions
        .iter()
        .map(|instructions| instructions.as_slice())
        .collect::<Vec<_>>();
    let result = calculate_number(
        ArithmeticLogicUnit::new(),
        grouped_instructions.as_slice(),
        0,
        &(1..=9),
        &mut HashSet::new(),
    );

    println!("Part 2. Result: {:?}", result);
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_1() {
        let instructions = load_data("./resources/test_data_1.txt");
        let mut alu = ArithmeticLogicUnit::new();
        let mut input = vec![2].into_iter();
        instructions
            .iter()
            .for_each(|instruction| alu.execute(instruction, &mut input));
        assert_eq!(
            alu,
            ArithmeticLogicUnit {
                w: 0,
                x: -2,
                y: 0,
                z: 0
            }
        );
    }

    #[test]
    fn test_data_2() {
        let instructions = load_data("./resources/test_data_2.txt");
        let mut alu = ArithmeticLogicUnit::new();
        let mut input = vec![2, 6].into_iter();
        instructions
            .iter()
            .for_each(|instruction| alu.execute(instruction, &mut input));
        assert_eq!(
            alu,
            ArithmeticLogicUnit {
                w: 0,
                x: 6,
                y: 0,
                z: 1
            }
        );
    }

    #[test]
    fn test_data_3() {
        let instructions = load_data("./resources/test_data_3.txt");
        let mut alu = ArithmeticLogicUnit::new();
        let mut input = vec![123].into_iter();
        instructions
            .iter()
            .for_each(|instruction| alu.execute(instruction, &mut input));
        assert_eq!(
            alu,
            ArithmeticLogicUnit {
                w: 1,
                x: 0,
                y: 1,
                z: 1
            }
        );
    }
}
