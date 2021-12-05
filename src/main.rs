mod days;

use std::fmt::Display;
use std::fs::{metadata, read_to_string};
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{anyhow, Result};
use gumdrop::Options;
use reqwest::blocking::Client;

fn print_day(day: usize) {
    let now = Instant::now();
    let result: Option<(Box<dyn Display>, Box<dyn Display>)> = match day {
        1 => printbox(days::day01::solve, "data/day01.txt"),
        2 => printbox(days::day02::solve, "data/day02.txt"),
        3 => printbox(days::day03::solve, "data/day03.txt"),
        4 => printbox(days::day04::solve, "data/day04.txt"),
        5 => printbox(days::day05::solve, "data/day05.txt"),
        6..=25 => None,
        _ => unreachable!(),
    };
    let elapsed = now.elapsed();
    print!("Day {:0>2}", day);
    match result {
        None => println!(": Unimplemented!"),
        Some(result) => {
            println!(
                " [{:.2?}]\n    Part 1: {}\n    Part 2: {}",
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

fn getdays(days: &[usize]) -> Vec<usize> {
    let mut days = days.to_vec();
    days.sort_unstable();
    days.dedup();
    let lastday = *days.last().unwrap();
    if *days.first().unwrap() < 1 || lastday > 25 {
        println!("Can only process days from 1 to 25 inclusive");
        std::process::exit(1);
    }
    days
}

fn download_inputs_if_missing(path: &Path, days: &[usize]) -> Result<()> {
    let mut client: Option<Client> = None;
    if !path.exists() {
        std::fs::create_dir(path)?
    } else {
        let md = metadata(path)?;
        if !md.is_dir() {
            return Err(anyhow!(
                "Path exists and is not a directory: {}",
                path.display()
            ));
        }
    }
    for &day in days {
        let daypath = path.join(format!("day{:0>2}.txt", day));
        if !daypath.exists() {
            println!("Downloading day {:0>2}...", day);
            if client.is_none() {
                client = Some(make_client()?)
            }
            let input = download_input(client.as_ref().unwrap(), day)?;
            std::fs::write(daypath, input)?;
        } else {
            println!("Input already exists: Day {:0>2}...", day);
        }
    }
    Ok(())
}

fn download_input(client: &Client, day: usize) -> Result<String> {
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let resp = client.get(url.as_str()).send()?;
    if !resp.status().is_success() {
        return Err(anyhow!(
            "Server request was not successful: {}",
            resp.text()?
        ));
    }
    Ok(resp.text()?)
}

fn make_client() -> Result<Client> {
    let mut headers = reqwest::header::HeaderMap::default();
    let session = match std::env::var("ADVENTOFCODE_SESSION") {
        Ok(s) => s,
        Err(e) => {
            println!(
                "Error: Could not load environmental variable ADVENTOFCODE_SESSION: \"{}\"",
                e
            );
            std::process::exit(1);
        }
    };
    let cookie =
        reqwest::header::HeaderValue::from_str(format!("session={}", session).as_str()).unwrap();
    headers.insert("Cookie", cookie);
    Ok(Client::builder().default_headers(headers).build()?)
}

#[derive(Options)]
struct MyOptions {
    #[options(help = "print help message")]
    help: bool,
    #[options(command)]
    command: Option<Command>,
}

#[derive(Options)]
enum Command {
    Solve(SolveOptions),
    Download(DownloadOptions),
}

#[derive(Options)]
struct SolveOptions {
    #[options(free)]
    days: Vec<usize>,
}

#[derive(Options, Debug)]
struct DownloadOptions {
    #[options(free)]
    path: PathBuf,
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
            if days.is_empty() {
                println!(
                    "Usage: {} solve day [day ...]",
                    std::env::args().next().unwrap()
                );
                std::process::exit(1);
            }
            let days = getdays(&days);
            let &lastday = days.last().unwrap();
            for day in days {
                print_day(day);
                if day != lastday {
                    println!();
                }
            }
        }
        Some(Command::Download(DownloadOptions { path, days })) => {
            if days.is_empty() {
                println!(
                    "Usage: {} download path day [day ...]",
                    std::env::args().next().unwrap()
                );
                std::process::exit(1);
            }
            let days = getdays(&days);
            match download_inputs_if_missing(&path, &days) {
                Ok(_) => (),
                Err(e) => {
                    println!("Error when downloading input: {}", e);
                    std::process::exit(1);
                }
            };
        }
    }
}
