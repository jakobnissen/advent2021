pub fn solve(s: &str) -> (usize, usize) {
    let v = parse(s);
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

fn parse(s: &str) -> Vec<(Direction, usize)> {
    s.lines().map(|s| {
        let (sdir, smag) = s.trim().split_once(' ').unwrap();
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

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (150, 900));
    }
}
