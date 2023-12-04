use crate::day_two_ast::{ColorEnum, Cube, Game, Round};
use crate::day_two_parser::GamesParser;

pub fn part_one(input: &str) -> u32 {
    let parser = GamesParser::new();

    let maxima = Round::from_cubes(vec![
        Cube(12, ColorEnum::Red),
        Cube(13, ColorEnum::Green),
        Cube(14, ColorEnum::Blue),
    ]);

    let games: Vec<Game> = parser.parse(input).unwrap();

    games
        .into_iter()
        .filter(|game| game.is_possible_given(&maxima))
        .map(|game| game.id)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let parser = GamesParser::new();

    let games: Vec<Game> = parser.parse(input).unwrap();

    games.iter().map(|game| game.power()).sum()
}
