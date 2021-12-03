mod days;

use std::io::{BufRead, BufReader};
use std::fs::{File};

fn day_lines(path: &str) -> impl Iterator<Item = String> {
    let buf = path.to_owned();
    let file = File::open(&path).unwrap_or_else(|e| {
        println!("Error when opening file: {:?}: {}", &buf, e);
        std::process::exit(1);
    });
    BufReader::new(file).lines().enumerate().map(move |(lineno, result)| {
        let string = result.unwrap_or_else(|e| {
            println!("Error when reading line {} of file at {:?}: {}", lineno, &buf, e);
            std::process::exit(1);
        });
        (lineno, string)
    }).map(|(_, s)| {s}).filter(|s| {!s.is_empty()})
}

fn main() {
    println!("{:?}", days::day01::solve(day_lines("data/day01.txt")));
    println!("{:?}", days::day02::solve(day_lines("data/day02.txt")));
}
