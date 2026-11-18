use anyhow::{Result, anyhow};
// use aoc2015::{AOC, AOCPrint, P1, P2};
use clap::{ArgGroup, Parser};
use colored::Colorize;
use indicatif::ProgressBar;
use std::time::Duration;
use std::{fmt::Display, fs::File, io::BufReader, path::PathBuf};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
// mod day12;

pub const AOC: &str = "AoC2018";
pub const P1: &str = "Part one";
pub const P2: &str = "Part two";

pub trait AOCPrint {
    fn aoc_print(&self, part: &str);
}

impl<T: Display> AOCPrint for Option<T> {
    #[cfg(not(tarpaulin_include))]
    fn aoc_print(&self, part: &str) {
        match self {
            Some(value) => {
                let value_str = value.to_string();
                println!("{} = {}", part, value_str.green())
            }
            None => {
                println!("{} = {}", part, "no result".red())
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("target")
        .required(true)
        .args(["day", "all"]),
))]
struct Args {
    /// Номер дня (1..=25)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=12))]
    day: Option<u8>,

    /// Запустить все дни
    #[arg(short, long)]
    all: bool,

    /// Путь к входному файлу (нельзя использовать с --all)
    #[arg(
        short,
        long,
        global = true,
        value_name = "FILE",
        conflicts_with = "all"
    )]
    input: Option<PathBuf>,
}

#[cfg(not(tarpaulin_include))]
fn run_day(day: u8, input_file: PathBuf) -> Result<(Option<String>, Option<String>)> {
    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
    let (res_1, res_2) = match day {
        1 => day01::solve(reader)?,
        2 => day02::solve(reader)?,
        3 => day03::solve(reader)?,
        4 => day04::solve(reader)?,
        5 => day05::solve(reader)?,
        6 => day06::solve(reader)?,
        7 => day07::solve(reader)?,
        8 => day08::solve(reader)?,
        9 => day09::solve(reader)?,
        10 => day10::solve(reader)?,
        11 => day11::solve(reader)?,
        // 12 => day12::solve(reader)?,
        _ => Err(anyhow!("Day{:02} is not implemented", day))?,
    };

    Ok((res_1, res_2))
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let args = Args::parse();
    if !args.all {
        let day = args.day.expect("Day number should be provaded");
        let input_file = args
            .input
            .unwrap_or_else(|| PathBuf::from(format!("inputs/day{:02}.txt", day)));

        let t0 = std::time::Instant::now();
        let (res1, res2) = run_day(day, input_file)?;
        let duration = t0.elapsed();

        println!("{} day{:02}", AOC, day);
        println!("-------------");

        res1.aoc_print(P1);
        res2.aoc_print(P2);

        println!("-------------");
        println!("Duration {} ms", duration.as_millis());
    } else {
        let mut res = Vec::new();
        let max_day: u8 = 12;
        let pb = ProgressBar::new(max_day as u64);
        let mut total_duration = Duration::new(0, 0);
        for day in 1..=max_day {
            let t0 = std::time::Instant::now();
            let day_results = run_day(day, (&format!("inputs/day{:02}.txt", day)).into());
            let duration = t0.elapsed();
            total_duration += duration;

            if let Ok((res1, res2)) = day_results {
                res.push((day, res1, res2, duration));
            } else {
                res.push((day, None, None, duration));
            }
            pb.inc(1);
        }
        pb.finish_with_message("Готово!");

        println!("{}", AOC);
        for rec in res {
            println!(
                "Day {:02} {P1} = {} {P2} = {} Duration = {:>4} ms",
                rec.0,
                if rec.1.is_some() { '✅' } else { '❌' },
                if rec.2.is_some() { '✅' } else { '❌' },
                rec.3.as_millis()
            );
        }
        println!("-------------");
        println!("Duration {} ms", total_duration.as_millis());
    }
    Ok(())
}
