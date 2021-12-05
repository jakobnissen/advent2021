pub fn solve(s: &str) -> (usize, usize) {
    let faultlines = parse(s);
    let mut map = {
        let (xmin, xmax, ymin, ymax) = faultlines.iter()
            .fold((usize::MAX, 0usize, usize::MAX, 0usize), |(xmin, xmax, ymin, ymax), faultline| {
            let (xi, xa, yi, ya) = faultline.bounding_box();
            (xmin.min(xi), xmax.max(xa), ymin.min(yi), ymax.max(ya))
        });
        let (rowsize, colsize) = ((xmax - xmin + 1), (ymax - ymin + 1));
        Map{noverlaps: 0, xmin, ymin, rowsize, counts: vec![0; rowsize * colsize]}
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

fn parse(s: &str) -> Vec<FaultLine> {
    s.lines().map(str::trim).filter(|s| !s.is_empty()).map(|line| {
        FaultLine::parse_from_line(line)
    }).collect()
}

struct Map {
    noverlaps: usize,
    xmin: usize,
    ymin: usize,
    rowsize: usize,
    counts: Vec<u8>
}

struct FaultLine {
    xmin: usize,
    y1: usize,
    delta: (u8, i8), // (0 | 1, -1 | 0 | 1)
    len: usize
}

impl FaultLine {
    fn parse_from_line(s: &str) -> Self {
        fn splitnumbers(s: &str) -> (usize, usize) {
            let (a, b) = s.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        }
        let (left, right) = s.split_once(" -> ").unwrap();
        let ((x1, y1), (x2, y2)) = (splitnumbers(left), splitnumbers(right));
        let (xmin, xmax, y1, y2) = if x1 < x2 {(x1, x2, y1, y2)} else {(x2, x1, y2, y1)};
        let dx = (xmax > xmin) as u8;
        let dy = (y2 as isize - y1 as isize).signum() as i8;
        let len = {
            let xlen = (xmax - xmin) + 1;
            let ylen = (y1.max(y2) - y1.min(y2)) + 1;
            if (xlen == 1 && ylen == 1) || (xlen > 1 && ylen > 1 && xlen != ylen) {
                panic!()
            }
            xlen.max(ylen)
        };
        FaultLine{xmin, y1, delta: (dx, dy), len}
    }

    fn is_level(&self) -> bool {
        self.delta.0 == 0 || self.delta.1 == 0
    }

    fn bounding_box(&self) -> (usize, usize, usize, usize) {
        let y2 = ((self.y1 as isize) + (self.delta.1 as isize) * ((self.len - 1) as isize)) as usize;
        (
            self.xmin,
            self.xmin + (self.delta.0 as usize) * (self.len - 1),
            self.y1.min(y2),
            self.y1.max(y2),
        )
    }

    fn add_to_map(&self, map: &mut Map) {
        let baseindex = (self.y1 - map.ymin) * map.rowsize + (self.xmin - map.xmin);
        let delta = self.delta.0 as isize + (self.delta.1 as isize) * (map.rowsize as isize);
        for i in 0..self.len {
            let index = (baseindex as isize + (delta * i as isize)) as usize;
            let v = map.counts[index];
            map.counts[index] = if v == 0 {
                1
            } else if v == 1 {
                map.noverlaps += 1;
                2
            } else {
                2
            };
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
