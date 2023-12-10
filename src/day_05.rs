use lalrpop_util::lalrpop_mod;

pub(crate) mod ast;
lalrpop_mod!(pub(crate) parser, "/day_05/parser.rs");

use parser::{Puzzle2Parser, PuzzleParser};

pub fn part_one(input: &str) -> i64 {
    PuzzleParser::new()
        .parse(input)
        .unwrap()
        .find_closest_location()
}

pub fn part_two(input: &str) -> i64 {
    let puzzle = Puzzle2Parser::new().parse(input).unwrap();

    println!("{:?}", puzzle);

    puzzle.find_closest_location()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#
            .trim()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(input()), 35);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(input()), 46);
    }
}
