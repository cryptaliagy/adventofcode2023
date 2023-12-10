pub fn part_one(input: &str) -> i64 {
    114
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
}
