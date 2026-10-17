use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
// mod day10;
mod day11;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_parser = clap::value_parser!(u8).range(1..=12))]
    day: u8,
    #[arg(short, long, default_value_t = false, global = true)]
    gui: bool,
    #[arg(short, long, global = true, value_name = "FILE", value_parser = clap::value_parser!(PathBuf))]
    input: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input_file = args
        .input
        .unwrap_or_else(|| PathBuf::from(format!("puzzles/day{:02}.txt", args.day)));
    if args.gui {
        unimplemented!("GUI mod is not implemented");
    } else {
        let (res_1, res_2) = match args.day {
            1 => day01::solve(&input_file)?,
            2 => day02::solve(&input_file)?,
            3 => day03::solve(&input_file)?,
            4 => day04::solve(&input_file)?,
            5 => day05::solve(&input_file)?,
            6 => day06::solve(&input_file)?,
            7 => day07::solve(&input_file)?,
            8 => day08::solve(&input_file)?,
            9 => day09::solve(&input_file)?,
            // 10 => day10::solve(&input_file)?,
            11 => day11::solve(&input_file)?,
            _ => unimplemented!("day{:02} is not implemented", args.day),
        };
        if let Some(res_1) = res_1 {
            println!("Part One = {}", res_1.yellow());
        }
        if let Some(res_2) = res_2 {
            println!("Part Two = {}", res_2.yellow());
        }
    }

    Ok(())
}
