use adventofcode2023::*;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The puzzle input ID. Should match with files in the input/ directory
    id: String,

    /// The part of the puzzle to solve
    #[arg(short = 'p', default_value = "1")]
    part: u8,
}

fn main() {
    let args = Cli::parse();

    let input = match std::fs::read_to_string(format!("inputs/{}.txt", args.id)) {
        Ok(input) => input,
        Err(_) => {
            eprintln!("Could not read input file");
            std::process::exit(1);
        }
    };

    let result = match args.id.as_str() {
        "1" => match args.part {
            1 => day_one::part_one(&input),
            2 => day_one::part_two(&input),
            _ => {
                eprintln!("Part {} not found", args.part);
                std::process::exit(1);
            }
        },
        "2" => match args.part {
            1 => day_two::part_one(&input),
            2 => day_two::part_two(&input),
            _ => {
                eprintln!("Part {} not found", args.part);
                std::process::exit(1);
            }
        },
        "4" => match args.part {
            1 => day_four::part_one(&input),
            _ => {
                eprintln!("Part {} not found", args.part);
                std::process::exit(1);
            }
        },
        "6" => match args.part {
            1 => day_six::part_one(&input) as u32,
            2 => day_six::part_two(&input) as u32,
            _ => {
                eprintln!("Part {} not found", args.part);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Id {} not found", args.id);
            std::process::exit(1);
        }
    };

    println!("{}", result);
}
