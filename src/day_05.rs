use lalrpop_util::lalrpop_mod;

pub(crate) mod ast;
lalrpop_mod!(pub(crate) parser, "/day_05/parser.rs");

use parser::{Puzzle2Parser, PuzzleParser};

pub fn part_one(input: &str) -> usize {
    PuzzleParser::new()
        .parse(input)
        .unwrap()
        .find_closest_location()
}

pub fn part_two(input: &str) -> usize {
    let puzzle = Puzzle2Parser::new().parse(input).unwrap();

    println!("{:?}", puzzle);

    puzzle.find_closest_location()
}
