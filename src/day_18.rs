#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
struct Instruction(Direction, u64);

#[derive(Debug, PartialEq)]
struct Line(Instruction, Instruction);

pub fn part_one(input: &str) -> i64 {
    let instructions = parse_input(input);

    let points = instructions.into_iter().map(|line| line.0).collect();

    find_area(points)
}

pub fn part_two(input: &str) -> i64 {
    find_area(parse_input(input).into_iter().map(|line| line.1).collect())
}

fn find_area(points: Vec<Instruction>) -> i64 {
    let points = points
        .into_iter()
        .fold(vec![(0, 0)], |mut acc, instruction| {
            let Instruction(direction, steps) = instruction;

            let (x, y) = acc.last().unwrap_or(&(0, 0));

            let (x, y) = match direction {
                Direction::Right => (x + steps as i64, *y),
                Direction::Left => (x - steps as i64, *y),
                Direction::Up => (*x, y - steps as i64),
                Direction::Down => (*x, y + steps as i64),
            };

            acc.push((x, y));

            acc
        });

    let mut area = 0;
    let mut j = points.len() - 1;
    let mut perimeter = 0.;

    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[j];

        area += (y2 + y1) * (x2 - x1);

        let segment = ((x2 - x1).abs().pow(2) as f64 + (y2 - y1).abs().pow(2) as f64).sqrt();
        perimeter += segment;

        j = i;
    }
    let perimeter = perimeter as i64;

    area.abs() / 2 + (perimeter / 2 + 1)
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();

            let direction = match parts[0] {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => unreachable!(),
            };

            let steps = parts[1].parse::<u64>().unwrap();

            let hex_code = parts[2]
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(')')
                .unwrap();

            // Parse 5 hex digits into base 10 integer
            let distance = u64::from_str_radix(&hex_code[0..5], 16).unwrap();
            let hex_direction = match hex_code.chars().last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            };

            Line(
                Instruction(direction, steps),
                Instruction(hex_direction, distance),
            )
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)"#;

        let instructions = parse_input(input);

        assert_eq!(
            instructions,
            vec![
                Line(
                    Instruction(Direction::Right, 6),
                    Instruction(Direction::Right, 0x70c71)
                ),
                Line(
                    Instruction(Direction::Down, 5),
                    Instruction(Direction::Down, 0x0dc57)
                ),
                Line(
                    Instruction(Direction::Left, 2),
                    Instruction(Direction::Right, 0x5713f)
                ),
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#;

        assert_eq!(part_one(input), 62);

        let input = r#"R 6 (#70c710)
        D 2 (#d2c081)
        L 6 (#5713f0)
        U 2 (#caa171)"#;

        assert_eq!(part_one(input), 21);

        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 4 (#5713f0)
        U 3 (#d2c081)
        L 2 (#59c680)
        U 2 (#411b91)"#;

        assert_eq!(part_one(input), 36);

        let input = r#"R 6 (#70c710)
        D 4 (#0dc571)
        L 4 (#5713f0)
        U 2 (#d2c081)
        L 2 (#59c680)
        U 2 (#411b91)"#;

        assert_eq!(part_one(input), 31);

        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        L 4 (#59c680)
        U 2 (#411b91)
        R 2 (#8ceee2)
        U 3 (#caa173)
        L 2 (#1b58a2)
        U 2 (#caa171)"#;

        assert_eq!(part_one(input), 48);
    }

    #[test]
    fn test_part_two() {
        let input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#;

        assert_eq!(part_two(input), 952408144115);
    }
}
