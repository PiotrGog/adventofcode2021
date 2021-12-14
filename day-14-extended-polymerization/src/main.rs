use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn load_data(file_name: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let file = File::open(file_name).expect(&format!("Can't read file {}", file_name));
    let file = BufReader::new(file);

    let mut lines = file.lines();
    let polymer = lines.next().unwrap().unwrap().chars().collect();
    lines.next();

    (
        polymer,
        lines.map(|rule| {
            let rule = rule.unwrap();
            let rule = rule.split(" -> ").collect::<Vec<_>>();
            let key = rule[0].to_string();
            let mut key = key.chars();
            ((key.next().unwrap(), key.next().unwrap()), rule[1].chars().next().unwrap())}
        ).collect()
    )
}

fn generate_polymer(polymer: Vec<char>, insertion_rule: &HashMap<(char, char), char>) -> Vec<char> {
    let mut result = vec![];
    result.reserve(polymer.len() * 2);
    let mut peekable = polymer.into_iter().peekable();

    let mut current_char = peekable.next().unwrap();
    result.push(current_char);

    while let Some(next_char) = peekable.peek() {
        result.push(insertion_rule[&(current_char, *next_char)]);
        result.push(*next_char);
        current_char = peekable.next().unwrap();
    }
    result
}

fn count_chars(polymer: &Vec<char>) -> HashMap<char, usize> {
    polymer.iter().fold(HashMap::new(), |mut acc, c| {
        if let Some(val) = acc.get_mut(c) {
            *val+=1;
        } else {
            acc.insert(*c, 1);
        }
        acc
    })
}

fn get_difference_between_most_and_least_common_element(polymer: &Vec<char>) -> usize {
    let counted_elements =  count_chars(polymer);
    let mut counted_elements = counted_elements.into_iter().collect::<Vec<_>>();
    counted_elements.sort_by(|e1, e2| e2.1.cmp(&e1.1));
    counted_elements.first().unwrap().1 - counted_elements.last().unwrap().1
}

fn part_1_result(file_name: &str) {
    let (mut polymer, insertion_rule) = load_data(file_name);
    for _ in 0..10 {
        polymer = generate_polymer(polymer, &insertion_rule);
    }
    println!(
        "Part 1. Result: {}",
        get_difference_between_most_and_least_common_element(&polymer)
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{generate_polymer, load_data, get_difference_between_most_and_least_common_element};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let (polymer, insertion_rule) = load_data(TEST_DATA_FILENAME);
        let mut polymer_org = polymer.clone();
        let expected = "NNCB".chars().collect::<Vec<_>>();
        assert_eq!(polymer, expected);
        let expected = "NCNBCHB".chars().collect::<Vec<_>>();
        let polymer = generate_polymer(polymer, &insertion_rule);
        assert_eq!(polymer, expected);
        let expected = "NBCCNBBBCBHCB".chars().collect::<Vec<_>>();
        let polymer = generate_polymer(polymer, &insertion_rule);
        assert_eq!(polymer, expected);
        let expected = "NBBBCNCCNBBNBNBBCHBHHBCHB".chars().collect::<Vec<_>>();
        let polymer = generate_polymer(polymer, &insertion_rule);
        assert_eq!(polymer, expected);
        let expected = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars().collect::<Vec<_>>();
        let polymer = generate_polymer(polymer, &insertion_rule);
        assert_eq!(polymer, expected);

        for _ in 0..10 {
            polymer_org = generate_polymer(polymer_org, &insertion_rule);
        }
        assert_eq!(get_difference_between_most_and_least_common_element(&polymer_org), 1588);
    }
}
