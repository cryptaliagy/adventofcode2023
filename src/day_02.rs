use lalrpop_util::lalrpop_mod;

pub(crate) mod ast;
lalrpop_mod!(pub(crate) parser, "/day_02/parser.rs");

use ast::{Cube, Round};
use parser::GamesParser;

pub fn part_one(input: &str) -> u32 {
    let maxima = Round::from_cubes(vec![
        Cube(12, "red".to_string()),
        Cube(13, "green".to_string()),
        Cube(14, "blue".to_string()),
    ]);

    GamesParser::new()
        .parse(input)
        .unwrap()
        .into_iter()
        .filter(|game| game.is_possible_given(&maxima))
        .map(|game| game.id)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    GamesParser::new()
        .parse(input)
        .unwrap()
        .into_iter()
        .map(|game| game.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        r#"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            .trim()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(input()), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(input()), 2286);
    }
}
