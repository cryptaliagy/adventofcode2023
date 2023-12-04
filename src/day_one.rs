use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let first = line.find(|c: char| c.is_numeric()).unwrap();
        let first = line.chars().nth(first).unwrap().to_digit(10).unwrap();

        let last = line.rfind(|c: char| c.is_numeric()).unwrap();
        let last = line.chars().nth(last).unwrap().to_digit(10).unwrap();

        total += (first * 10) + last;
    }

    total
}

pub fn part_two(input: &str) -> u32 {
    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("0", 0),
    ]);

    let mut total = 0;

    for line in input.lines() {
        let first = find_earliest_val(line, &numbers);
        let last = find_latest_val(line, &numbers);

        total += (first * 10) + last;
    }

    total
}

fn find_earliest_val(line: &str, numbers: &HashMap<&str, u32>) -> u32 {
    numbers
        .iter()
        .map(|(&k, &v)| (line.find(k), v))
        .filter_map(|(i, v)| i.map(|ind| (ind, v)))
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .unwrap()
        .1
}

fn find_latest_val(line: &str, numbers: &HashMap<&str, u32>) -> u32 {
    numbers
        .iter()
        .map(|(&k, &v)| (line.rfind(k), v))
        .filter_map(|(i, v)| i.map(|ind| (ind, v)))
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .unwrap()
        .1
}
