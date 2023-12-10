use std::collections::HashSet;

use lalrpop_util::lalrpop_mod;

pub(crate) mod ast;
lalrpop_mod!(pub(crate) parser, "/day_04/parser.rs");

use parser::CardsParser;

pub fn part_one(input: &str) -> u32 {
    CardsParser::new()
        .parse(input)
        .unwrap()
        .into_iter()
        .map(|card| {
            card.1
                .into_iter()
                .collect::<HashSet<_>>()
                .intersection(&card.2.into_iter().collect::<HashSet<_>>())
                .count() as u32
        })
        .filter(|&count| count > 0)
        .map(|count| 2u32.pow(count - 1))
        .sum()
}
