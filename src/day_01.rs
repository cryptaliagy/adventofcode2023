use std::collections::HashMap;

pub fn part_one(input: &str) -> i64 {
    let mut total = 0;

    for line in input.lines() {
        let first = line.find(|c: char| c.is_numeric()).unwrap();
        let first = line.chars().nth(first).unwrap().to_digit(10).unwrap();

        let last = line.rfind(|c: char| c.is_numeric()).unwrap();
        let last = line.chars().nth(last).unwrap().to_digit(10).unwrap();

        total += (first * 10) + last;
    }

    total as i64
}

pub fn part_two(input: &str) -> i64 {
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

fn find_earliest_val(line: &str, numbers: &HashMap<&str, i64>) -> i64 {
    numbers
        .iter()
        .map(|(&k, &v)| (line.find(k), v))
        .filter_map(|(i, v)| i.map(|ind| (ind, v)))
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .expect("No numbers found")
        .1
}

fn find_latest_val(line: &str, numbers: &HashMap<&str, i64>) -> i64 {
    numbers
        .iter()
        .map(|(&k, &v)| (line.rfind(k), v))
        .filter_map(|(i, v)| i.map(|ind| (ind, v)))
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#
        .trim();
        assert_eq!(part_one(input), 142);
    }

    #[test]
    fn test_part_two() {
        let input = r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
                "#
        .trim();
        assert_eq!(part_two(input), 281);
    }
}
