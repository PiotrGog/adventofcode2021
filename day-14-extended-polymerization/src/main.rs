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

fn polymer_to_pairs(polymer: &Vec<char>) -> HashMap<(char, char), usize> {
    polymer.windows(2).fold(HashMap::new(), |mut acc, chars| {
        if let Some(val) = acc.get_mut(&(chars[0], chars[1])) {
            *val+=1;
        } else {
            acc.insert((chars[0], chars[1]), 1);
        }
        acc
    })
}

fn generate_polymer_from_pairs(polymer: HashMap<(char, char), usize>, insertion_rule: &HashMap<(char, char), char>) -> HashMap<(char, char), usize> {
    let mut result = HashMap::new();

    for (key, value) in polymer {
        let matching_part = insertion_rule[&key];
        if let Some(val) = result.get_mut(&(key.0, matching_part)) {
            *val+=value;
        } else {
            result.insert((key.0, matching_part), value);
        }

        if let Some(val) = result.get_mut(&( matching_part, key.1)) {
            *val+=value;
        } else {
            result.insert((matching_part, key.1), value);
        }
    }

    result
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

fn count_chars_pairs(polymer: &HashMap<(char, char), usize>) -> HashMap<char, usize> {
    polymer.iter().fold(HashMap::new(), |mut acc, ((c1, c2), count)| {
        if let Some(val) = acc.get_mut(c1) {
            *val += count/2;
        } else {
            acc.insert(*c1, count/2);
        }
        if let Some(val) = acc.get_mut(c2) {
            *val+= count/2;
        } else {
            acc.insert(*c2, count/2);
        }
        acc
    })
}

fn get_difference_between_most_and_least_common_element(polymer: &HashMap<(char, char), usize>) -> usize {
    let counted_elements =  count_chars_pairs(polymer);
    let mut counted_elements = counted_elements.into_iter().collect::<Vec<_>>();
    counted_elements.sort_by(|e1, e2| e2.1.cmp(&e1.1));
    counted_elements.first().unwrap().1 - counted_elements.last().unwrap().1
}

fn part_1_result(file_name: &str) {
    let (polymer, insertion_rule) = load_data(file_name);
    let mut polymer = polymer_to_pairs(&polymer);
    for _ in 0..10 {
        polymer = generate_polymer_from_pairs(polymer, &insertion_rule);
    }
    println!(
        "Part 1. Result: {}",
        get_difference_between_most_and_least_common_element(&polymer)
    );
}

fn part_2_result(file_name: &str) {
    let (polymer, insertion_rule) = load_data(file_name);
    let mut polymer = polymer_to_pairs(&polymer);
    for _ in 0..40 {
        polymer = generate_polymer_from_pairs(polymer, &insertion_rule);
    }
    println!(
        "Part 2. Result: {}",
        get_difference_between_most_and_least_common_element(&polymer)
    );
}

fn main() {
    const DATA_FILENAME: &str = "./resources/data.txt";
    part_1_result(DATA_FILENAME);
    part_2_result(DATA_FILENAME);
}

#[cfg(test)]
mod tests {
    use crate::{generate_polymer, load_data, get_difference_between_most_and_least_common_element, polymer_to_pairs, generate_polymer_from_pairs};

    #[test]
    fn part_1_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let (polymer, insertion_rule) = load_data(TEST_DATA_FILENAME);
        let polymer_org = polymer.clone();
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

        let mut polymer = polymer_to_pairs(&polymer_org);
        for _ in 0..10 {
            polymer = generate_polymer_from_pairs(polymer, &insertion_rule);
        }
        assert_eq!(get_difference_between_most_and_least_common_element(&polymer), 1588);
    }

    #[test]
    fn part_2_test_data() {
        const TEST_DATA_FILENAME: &str = "./resources/test_data.txt";
        let (polymer, insertion_rule) = load_data(TEST_DATA_FILENAME);

        let mut polymer = polymer_to_pairs(&polymer);
        for _ in 0..40 {
            polymer = generate_polymer_from_pairs(polymer, &insertion_rule);
        }
        assert_eq!(get_difference_between_most_and_least_common_element(&polymer), 2188189693529);
    }
}
