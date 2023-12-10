use std::collections::HashSet;

use lalrpop_util::lalrpop_mod;

pub(crate) mod ast;
lalrpop_mod!(pub(crate) parser, "/day_04/parser.rs");

use parser::CardsParser;

pub fn part_one(input: &str) -> i64 {
    CardsParser::new()
        .parse(input)
        .unwrap()
        .into_iter()
        .map(|card| {
            card.1
                .into_iter()
                .collect::<HashSet<_>>()
                .intersection(&card.2.into_iter().collect::<HashSet<_>>())
                .count()
        })
        .filter(|&count| count > 0)
        .map(|count| 2i64.pow((count - 1) as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .trim()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(input()), 13);
    }
}
