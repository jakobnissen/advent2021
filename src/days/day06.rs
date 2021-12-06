pub fn solve(s: &str) -> (usize, usize) {
    let lut: [(usize, usize); 7] = [
        (1421, 6703087164),
        (1401, 6206821033),
        (1191, 5617089148),
        (1154, 5217223242),
        (1034, 4726100874),
        ( 950, 4368232009),
        ( 905, 3989468462)
    ];
    s.trim().as_bytes().iter().enumerate().fold((0, 0), |(p1, p2), (i, &b)| {
        if i % 2 == 1 {
            if b != b',' {
                panic!()
            }
            return (p1, p2)
        }
        let n = b - b'0';
        if !(0x01..0x08).contains(&n) {
            panic!()
        }
        let (a1, a2) = lut[n as usize];
        (p1 + a1, p2 + a2)
    })
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "3,4,3,1,2";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (5934, 26984457539));
    }
}
