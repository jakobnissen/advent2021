pub fn solve(s: &str) -> (usize, usize) {
    let mut fishes: [usize; 9] = [0; 9];
    s.trim()
        .split(',')
        .for_each(|s| fishes[s.parse::<usize>().unwrap()] += 1);
    let mut part1 = 0;
    for _day in 0..256 {
        fishes[7] += fishes[0];
        fishes.rotate_left(1);
        if _day == 79 {
            part1 = fishes.iter().sum();
        }
    }
    (part1, fishes.iter().sum())
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "3,4,3,1,2";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (5934, 26984457539));
    }
}
