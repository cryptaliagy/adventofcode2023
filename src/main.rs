use adventofcode2023::*;
use clap::Parser;

#[cfg(feature = "logging")]
use tracing::{error, info};

/// Advent of Code 2023 - A Rust CLI for solving Advent of Code 2023 puzzles.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The puzzle day to solve.
    day: u8,

    /// The part of the puzzle to solve.
    #[arg(short = 'p', long, default_value = "1")]
    part: u8,

    /// The input file to use. [default: `input/{day}.txt`]
    #[arg(short = 'f', long)]
    file: Option<String>,

    /// The verbosity level. Default is info. Set once for debug, twice or more for trace. Requires the `logging` feature.
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Cli::parse();

    // Setup logging based on verbosity level
    #[cfg(feature = "logging")]
    {
        let log_level = match args.verbose {
            0 => tracing::Level::INFO,
            1 => tracing::Level::DEBUG,
            _ => tracing::Level::TRACE,
        };

        let subscriber = tracing_subscriber::fmt()
            .pretty()
            .with_max_level(log_level)
            .with_target(true)
            .finish();

        tracing::subscriber::set_global_default(subscriber).unwrap();

        info!("Solving day {} part {}...", args.day, args.part);
    }
    #[cfg(not(feature = "logging"))]
    {
        println!("Solving day {} part {}...", args.day, args.part)
    }

    let input_file = args
        .file
        .clone() // Clone so there is no partial move
        .unwrap_or(format!("inputs/{:02}.txt", args.day));
    let input = match std::fs::read_to_string(input_file.as_str()) {
        Ok(input) => input,
        Err(e) => {
            #[cfg(feature = "logging")]
            {
                error!(?args, ?input_file, "Error reading input file: {}", e);
            }
            #[cfg(not(feature = "logging"))]
            {
                eprintln!("Error reading input file '{}': {}", input_file.as_str(), e);
            }
            std::process::exit(1);
        }
    };

    // Start the timer only if opting into timing through a feature flag.
    #[cfg(feature = "metrics")]
    let start = std::time::SystemTime::now();

    let result = match args.day {
        1 => match args.part {
            1 => day_01::part_one(&input),
            2 => day_01::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        2 => match args.part {
            1 => day_02::part_one(&input).into(),
            2 => day_02::part_two(&input).into(),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        4 => match args.part {
            1 => day_04::part_one(&input),
            // 2 => day_04::part_two(&input).into(),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        5 => match args.part {
            1 => day_05::part_one(&input),
            2 => day_05::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        6 => match args.part {
            1 => day_06::part_one(&input),
            2 => day_06::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        8 => match args.part {
            1 => day_08::part_one(&input),
            2 => day_08::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        9 => match args.part {
            1 => day_09::part_one(&input),
            2 => day_09::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        15 => match args.part {
            1 => day_15::part_one(&input),
            // 2 => day_15::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        18 => match args.part {
            1 => day_18::part_one(&input),
            2 => day_18::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        19 => match args.part {
            1 => day_19::part_one(&input),
            2 => day_19::part_two(&input),
            _ => {
                #[cfg(feature = "logging")]
                {
                    error!(?args, "Invalid part number",);
                }
                #[cfg(not(feature = "logging"))]
                {
                    eprintln!("Invalid part number: {}", args.part);
                }
                std::process::exit(1);
            }
        },
        _ => {
            #[cfg(feature = "logging")]
            {
                error!(?args, "Invalid day number");
            }
            #[cfg(not(feature = "logging"))]
            {
                eprintln!("Invalid day number: {}", args.day);
            }
            std::process::exit(1);
        }
    };

    #[cfg(feature = "metrics")]
    let elapsed = start.elapsed().unwrap();

    #[cfg(feature = "logging")]
    {
        info!("Result: {}", result);
    }
    #[cfg(not(feature = "logging"))]
    {
        println!("Result: {}", result);
    }

    #[cfg(feature = "metrics")]
    {
        #[cfg(feature = "logging")]
        {
            info!("Solution found in {}", utils::format_duration(elapsed));
        }
        #[cfg(not(feature = "logging"))]
        {
            println!("Solution found in {}", utils::format_duration(elapsed));
        }
    }
}
