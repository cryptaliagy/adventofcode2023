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
