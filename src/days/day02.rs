pub fn solve<I>(lines: I) -> (usize, usize)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let v = parse(lines);
    (part1(&v), part2(&v))
}

fn part1(v: &[(Direction, usize)]) -> usize {
    let (mut hor, mut depth): (usize, usize) = (0, 0);
    for (d, m) in v {
        match d {
            Direction::Forward => hor += m,
            Direction::Down => depth += m,
            Direction::Up => depth -= m,
        }
    }
    hor * depth
}

fn part2(v: &[(Direction, usize)]) -> usize {
    let (mut hor, mut depth, mut aim): (usize, usize, usize) = (0, 0, 0);
    for (d, m) in v {
        match d {
            Direction::Forward => {
                depth += aim * m;
                hor += m
            }
            Direction::Down => aim += m,
            Direction::Up => aim -= m,
        }
    }
    hor * depth
}

enum Direction {
    Forward,
    Down,
    Up,
}

fn parse<I>(lines: I) -> Vec<(Direction, usize)>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    lines
        .into_iter()
        .map(|s| {
            let (sdir, smag) = s.as_ref().split_once(' ').unwrap();
            let dir = match sdir {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => unreachable!(),
            };
            (dir, smag.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>()
}

mod tests {
    static TEST_STR: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test() {
        assert_eq!(super::solve(crate::test_lines(TEST_STR)), (150, 900));
    }
}
