pub(crate) mod models;

use models::*;

pub fn part_one(input: &str) -> i64 {
    let mut total = 0;

    let mut interpolator = Interpolator::new();

    for line in input.lines() {
        let numbers = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<_>().unwrap())
            .collect::<Vec<_>>();

        let x = numbers.len() as i64;

        total += interpolator.interpolate(&numbers, x).unwrap();
    }

    total
}

pub fn part_two(input: &str) -> i64 {
    let mut total = 0;

    let mut interpolator = Interpolator::new();

    for line in input.lines() {
        let numbers = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<_>().unwrap())
            .collect::<Vec<_>>();

        total += interpolator.interpolate(&numbers, -1).unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#
            .trim();

        assert_eq!(part_one(input), 114);
    }

    #[test]
    fn test_part_two() {
        let input = r#"0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"#
            .trim();

        assert_eq!(part_two(input), 2);
    }
}
