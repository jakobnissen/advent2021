mod days;

use std::fmt::Display;
use std::fs::read_to_string;
use std::time::Instant;

use gumdrop::Options;

fn print_day(day: usize) {
    let now = Instant::now();
    let result: Option<(Box<dyn Display>, Box<dyn Display>)> = match day {
        1 => printbox(days::day01::solve, "data/day01.txt"),
        2 => printbox(days::day02::solve, "data/day02.txt"),
        3 => printbox(days::day03::solve, "data/day03.txt"),
        4 => printbox(days::day04::solve, "data/day04.txt"),
        5..=25 => None,
        _ => unreachable!(),
    };
    let elapsed = now.elapsed();
    print!("Day {:0>2}", day);
    match result {
        None => println!(": Unimplemented!\n"),
        Some(result) => {
            println!(
                " [{:.2?}]\n    Part 1: {}\n    Part 2: {}\n",
                elapsed, result.0, result.1
            );
        }
    }
}

fn printbox<F, A: 'static, B: 'static>(
    f: F,
    s: &str,
) -> Option<(Box<dyn Display>, Box<dyn Display>)>
where
    F: Fn(&str) -> (A, B),
    A: Display,
    B: Display,
{
    let data = match read_to_string(s) {
        Err(e) => {
            println!("Error when reading file {} to string: \"{}\"", s, e);
            std::process::exit(1);
        }
        Ok(s) => s,
    };
    let (a, b) = f(&data);
    Some((Box::new(a), Box::new(b)))
}

#[derive(Options, Debug)]
struct MyOptions {
    #[options(help = "print help message")]
    help: bool,
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
    days: Vec<usize>,
}

fn main() {
    let opts = MyOptions::parse_args_default_or_exit();
    match opts.command {
        None => {
            println!("No subcommand specified. See advent2021 --help");
            std::process::exit(1);
        }
        Some(Command::Solve(SolveOptions { days })) => {
            let mut days = days.iter().copied().collect::<Vec<_>>();
            days.sort_unstable();
            days.dedup();
            if days.is_empty() {
                println!(
                    "Usage: {} solve day [day ...]",
                    std::env::args().next().unwrap()
                );
                std::process::exit(1);
            }
            if *days.first().unwrap() < 1 || *days.last().unwrap() > 25 {
                println!("Can only solve days from 1 to 25 inclusive");
                std::process::exit(1);
            }
            for day in days {
                print_day(day)
            }
        }
    }
}
