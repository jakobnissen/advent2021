pub fn solve(s: &str) -> (usize, usize) {
    let nums: Vec<_> = s
        .lines()
        .map(|s| {
            s.trim().parse::<usize>().unwrap_or_else(|_| {
                println!("Cannot parse as usize: \"{}\"", s);
                std::process::exit(1);
            })
        })
        .collect();

    let part1 = nums
        .iter()
        .zip(&nums[1..])
        .filter(|(prev, next)| next > prev)
        .count();
    let part2 = nums
        .iter()
        .zip(&nums[3..])
        .filter(|(prev, next)| next > prev)
        .count();
    (part1, part2)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (7, 5));
    }
}
