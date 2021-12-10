const PARENS: [(u8, u8, u16); 4] = [
    (b'(', b')', 3),
    (b'[', b']', 57),
    (b'{', b'}', 1197),
    (b'<', b'>', 25137),
];

pub fn solve(s: &str) -> (usize, usize) {
    let mut stack: Vec<u8> = Vec::new();
    let mut autocompletes: Vec<usize> = Vec::new();
    let mut part1: usize = 0;
    'lineloop: for line in s.trim().lines().map(str::trim) {
        stack.truncate(0);
        'charloop: for &b in line.as_bytes() {
            for (open, close, score) in PARENS {
                if b == open {
                    stack.push(b);
                    continue 'charloop;
                } else if b == close {
                    if stack.pop().unwrap_or(0) != open {
                        part1 += score as usize;
                        continue 'lineloop;
                    }
                }
            }
        }
        autocompletes.push(
            stack
                .iter()
                .rev()
                .map(|paren| match paren {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |n, i| 5 * n + i),
        );
    }
    let len = autocompletes.len();
    let part2 = *autocompletes.select_nth_unstable(len / 2).1;
    (part1, part2)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (26397, 288957));
    }
}
