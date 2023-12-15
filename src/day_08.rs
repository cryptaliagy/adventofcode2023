use std::collections::HashMap;

type Paths<'a> = HashMap<&'a str, (&'a str, &'a str)>;

pub fn part_one(input: &str) -> i64 {
    let (sequence, paths) = construct_from_input(input);

    find_end("AAA", |node| node == "ZZZ", sequence, &paths)
}

pub fn part_two(input: &str) -> i64 {
    let (sequence, paths) = construct_from_input(input);

    let find_end = |x| find_end(x, |node| node.ends_with('Z'), sequence, &paths);

    paths
        .iter()
        .filter(|(&key, _)| key.ends_with('A'))
        .map(|(&key, _)| find_end(key))
        .reduce(|a, b| (a * b) / gcd(a, b))
        .unwrap()
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn find_end(
    start: &str,
    condition: impl Fn(&str) -> bool,
    sequence: &str,
    paths: &Paths<'_>,
) -> i64 {
    let mut current = start;

    let mut count = 0;

    while !condition(current) {
        for key in sequence.chars() {
            let (left, right) = paths.get(current).unwrap();

            if key == 'L' {
                current = left;
            } else {
                current = right;
            }

            count += 1;

            if condition(current) {
                break;
            }
        }
    }

    count
}

fn construct_from_input(input: &str) -> (&str, Paths<'_>) {
    let mut lines = input.lines();

    let sequence = lines.next().unwrap();

    let mut paths = HashMap::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (from, to) = line.split_once(" = ").unwrap();

        let from = from.trim();
        let to = to.trim();

        let (left, right) = to
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        paths.insert(from, (left, right));
    }

    (sequence, paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"
        LLR
AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            .trim();

        assert_eq!(part_one(input), 6);
    }
}
