pub fn solve(s: &str) -> (usize, String) {
    let (points, folds) = parse(s);
    let &firstfold = folds.first().unwrap();
    let mut onefold = points
        .iter()
        .map(|&p| fold(p, firstfold))
        .collect::<Vec<_>>();
    onefold.sort_unstable();
    onefold.dedup();
    let part1 = onefold.len();
    let mut allfolds = points
        .iter()
        .map(|&p| folds.iter().fold(p, |point, &newfold| fold(point, newfold)))
        .collect::<Vec<_>>();
    allfolds.sort_unstable();
    allfolds.dedup();
    (part1, res_string(&allfolds))
}

fn fold(point: (usize, usize), fld: (bool, usize)) -> (usize, usize) {
    if fld.0 && point.0 > fld.1 {
        (2 * fld.1 - point.0, point.1)
    } else if !fld.0 && point.1 > fld.1 {
        (point.0, 2 * fld.1 - point.1)
    } else {
        point
    }
}

fn res_string(v: &[(usize, usize)]) -> String {
    let nx = v.iter().max_by_key(|&&x| x.0).unwrap().0 + 1;
    let ny = v.iter().max_by_key(|&&x| x.1).unwrap().1 + 1;
    let mut stringbufs = vec![vec![b' '; nx]; ny];
    for (x, y) in v.iter() {
        stringbufs[*y][*x] = b'#'
    }
    let mut res = String::new();
    for i in stringbufs {
        res.push('\n');
        res.push_str(&String::from_utf8(i).unwrap());
    }
    res
}

fn parse(s: &str) -> (Vec<(usize, usize)>, Vec<(bool, usize)>) {
    let mut points = Vec::new();
    let mut folds = Vec::new();
    for line in s.lines().map(str::trim).filter(|s| !s.is_empty()) {
        if line.starts_with("fold along") {
            let d = line.chars().nth(11).unwrap();
            folds.push((d == 'x', line[13..].parse::<usize>().unwrap()));
        } else {
            let (a, b) = line.split_once(',').unwrap();
            points.push((a.parse().unwrap(), b.parse().unwrap()))
        }
    }
    (points, folds)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0
    
    fold along y=7
    fold along x=5";

    static RES_STR: &str = "
#####
#   #
#   #
#   #
#####";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (17, RES_STR.to_owned()));
    }
}
