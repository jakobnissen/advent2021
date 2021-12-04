mod days;

use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};
use std::fmt::Display;

use gumdrop::Options;

fn day_lines(path: &str) -> impl Iterator<Item = String> {
    let buf = path.to_owned();
    let file = File::open(&path).unwrap_or_else(|e| {
        println!("Error when opening file: {:?}: {}", &buf, e);
        std::process::exit(1);
    });
    BufReader::new(file)
        .lines()
        .enumerate()
        .map(move |(lineno, result)| {
            let string = result.unwrap_or_else(|e| {
                println!(
                    "Error when reading line {} of file at {:?}: {}",
                    lineno, &buf, e
                );
                std::process::exit(1);
            });
            (lineno, string)
        })
        .map(|(_, s)| s)
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
fn test_lines(s: &'static str) -> impl Iterator<Item = String> {
    s.lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
}

fn print_day(day: usize) {
    let result: (Box<dyn Display>, Box<dyn Display>) = match day {
        1 => printbox(days::day01::solve(day_lines("data/day01.txt"))),
        2 => printbox(days::day02::solve(day_lines("data/day02.txt"))),
        3 => printbox(days::day03::solve(day_lines("data/day03.txt"))),
        4 => printbox(days::day04::solve(&read_to_string("data/day04.txt").unwrap())),
        _ => unreachable!(),
    };
    println!("Day {:0>2}\n\tPart 1: {}\n\tPart 2: {}", day, result.0, result.1);
}

fn printbox<A: 'static, B: 'static>(s: (A, B)) -> (Box<dyn Display>, Box< dyn Display>)
where
    A: Display,
    B: Display
{
    (Box::new(s.0), Box::new(s.1))
}


#[derive(Options, Debug)]
struct MyOptions {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "be verbose")]
    verbose: bool,
    #[options(command)]
    command: Option<Command>,
}

#[derive(Options, Debug)]
enum Command {
    Solve(SolveOptions),
}

#[derive(Options, Debug)]
struct SolveOptions {
    #[options(free)]
    days: Vec<usize>
}

fn main() {
    let opts = MyOptions::parse_args_default_or_exit();
    match opts.command {
        None => {
            println!("No subcommand specified. See advent2021 --help");
            std::process::exit(1);
        },
        Some(Command::Solve(SolveOptions {days})) => {
            let mut days = days.iter().copied().collect::<Vec<_>>();
            days.sort_unstable();
            days.dedup();
            if days.is_empty() {
                println!("Pass at least one day to solve");
                std::process::exit(1);
            }
            if *days.first().unwrap() < 1 || *days.last().unwrap() > 25 {
                println!("Can only solve days from 1 to 25 inclusive");
                std::process::exit(1);
            }
            if *days.last().unwrap() > 4 {
                println!("Day {} not yet implemented.", days.last().unwrap());
                std::process::exit(1);
            }
            for day in days {
                print_day(day)
            }
        }
    }
}
