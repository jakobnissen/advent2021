pub fn solve<I>(lines: I) -> (usize, usize)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let nums: Vec<_> = lines
        .into_iter()
        .map(|s| {
            s.as_ref().parse::<usize>().unwrap_or_else(|_| {
                println!("Cannot parse as usize: \"{}\"", s.as_ref());
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
        assert_eq!(super::solve(crate::test_lines(TEST_STR)), (7, 5));
    }
}
