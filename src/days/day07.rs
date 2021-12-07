pub fn solve(s: &str) -> (usize, usize) {
    let mut v: Vec<_> = s
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    if v.is_empty() {
        return (0, 0);
    }
    let len = v.len();
    let median = *v.select_nth_unstable(len / 2).1;
    // The result can be mathematically proven to be either ceil(mean) or floor(mean).
    // The mean here is the floor, then we check both in the fold below.
    let mean = v.iter().sum::<isize>() / len as isize;
    let (y1, y2, part1) = v.iter().fold((0, 0, 0), |(y1, y2, part1), i| {
        let (d1, d2) = ((mean - i).abs(), (mean + 1 - i).abs());
        (
            y1 + d1 * (d1 + 1),
            y2 + d2 * (d2 + 1),
            part1 + (i - median).abs(),
        )
    });
    (part1.try_into().unwrap(), (y1.min(y2) / 2).try_into().unwrap())
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (37, 168));
    }
}
