pub fn solve(s: &str) -> (usize, usize) {
    let mut v: Vec<_> = s
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    if v.is_empty() {
        return (0, 0);
    }
    // The median can be calculated in O(n) time by using QuickSelect, but sort is fast enough.
    v.sort_unstable();
    let median = v[v.len() / 2];
    let part1 = v
        .iter()
        .map(|i| usize::try_from((i - median).abs()).unwrap())
        .sum();
    // The result can be mathematically proven to be either ceil(mean) or floor(mean).
    // The mean here is the floor, then we check both.
    let mean = v.iter().sum::<isize>() / v.len() as isize;
    let (y1, y2) = v.iter().fold((0, 0), |(y1, y2), i| {
        let (d1, d2) = ((mean - i).abs() as usize, (mean + 1 - i).abs() as usize);
        (y1 + d1 * (d1 + 1), y2 + d2 * (d2 + 1))
    });
    let part2 = y1.min(y2) / 2;
    (part1, part2)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (37, 168));
    }
}
