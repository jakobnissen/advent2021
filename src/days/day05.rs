pub fn solve(s: &str) -> (usize, usize) {
    let faultlines: Vec<_> = s
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(FaultLine::parse_from_line)
        .collect();
    let mut map = {
        let (xmin, xmax, ymin, ymax) = faultlines.iter().fold(
            (isize::MAX, 0isize, isize::MAX, 0isize),
            |(xmin, xmax, ymin, ymax), i| {
                (xmin.min(i.0).min(i.1), xmax.max(i.0).max(i.1), ymin.min(i.2).min(i.3), ymax.max(i.2).max(i.3))
            },
        );
        let (rowsize, colsize) = ((xmax - xmin + 1), (ymax - ymin + 1));
        Map {
            noverlaps: 0,
            xmin,
            ymin,
            rowsize,
            counts: vec![0; (rowsize * colsize) as usize],
        }
    };
    for faultline in faultlines.iter().filter(|x| x.is_level()) {
        faultline.add_to_map(&mut map)
    }
    let part1 = map.noverlaps;
    for faultline in faultlines.iter().filter(|x| !x.is_level()) {
        faultline.add_to_map(&mut map)
    }
    let part2 = map.noverlaps;
    (part1, part2)
}

struct Map {
    noverlaps: usize,
    xmin: isize,
    ymin: isize,
    rowsize: isize,
    counts: Vec<u8>,
}

#[derive(Debug)]
struct FaultLine(isize, isize, isize, isize);

impl FaultLine {
    fn parse_from_line(s: &str) -> Self {
        fn splitnumbers(s: &str) -> (isize, isize) {
            let (a, b) = s.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        }
        let (left, right) = s.split_once(" -> ").unwrap();
        let ((x1, y1), (x2, y2)) = (splitnumbers(left), splitnumbers(right));
        FaultLine(x1, x2, y1, y2)
    }

    fn is_level(&self) -> bool {
        self.0 == self.1 || self.2 == self.3
    }

    fn add_to_map(&self, map: &mut Map) {
        let mut index = (self.0 - map.xmin) + map.rowsize * (self.2 - map.ymin);
        let lastindex = (self.1 - map.xmin) + map.rowsize * (self.3 - map.ymin);
        let delta = (self.1 - self.0).signum() + (self.3 - self.2).signum() * map.rowsize;
        loop {
            let v = map.counts[index as usize];
            map.counts[index as usize] = if v == 0 {
                1
            } else if v == 1 {
                map.noverlaps += 1;
                2
            } else {
                2
            };
            if index == lastindex {break}
            index += delta;
        }
    }
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
    ";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (5, 12));
    }
}
