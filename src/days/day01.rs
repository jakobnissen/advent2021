pub fn solve(s: &str) -> (usize, usize) {
    let nums: Vec<_> = s
        .lines()
        .map(|s| {
            s.trim().parse::<usize>().unwrap_or_else(|e| {
                println!("Cannot parse as usize: \"{}\"", e);
                std::process::exit(1);
            })
        })
        .collect();

    let part1 = nums
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count();
    let part2 = nums
        .windows(4)
        .filter(|window| window[3] > window[0])
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
