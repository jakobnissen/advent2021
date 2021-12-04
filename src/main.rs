mod days;

use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

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

fn main() {
    println!("{:?}", days::day01::solve(day_lines("data/day01.txt")));
    println!("{:?}", days::day02::solve(day_lines("data/day02.txt")));
    println!("{:?}", days::day03::solve(day_lines("data/day03.txt")));
    println!(
        "{:?}",
        days::day04::solve(&read_to_string("data/day04.txt").unwrap())
    );
}
