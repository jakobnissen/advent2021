pub fn solve(s: &str) -> (usize, usize) {
    let faultlines: Vec<_> = s
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(FaultLine::parse_from_line)
        .collect();
    let mut map = {
        let mapsize = faultlines.iter().fold(0isize, |max, i| {
            max.max(i.0).max(i.1).max(i.2).max(i.3)
        }) + 1;
        Map {
            noverlaps: 0,
            mapsize,
            counts: vec![0; (mapsize * mapsize) as usize],
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
    mapsize: isize,
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
        let mut index = self.0 + map.mapsize * self.2;
        let lastindex = self.1 + map.mapsize * self.3;
        let delta = (self.1 - self.0).signum() + (self.3 - self.2).signum() * map.mapsize;
        loop {
            let v = map.counts[index as usize];
            map.noverlaps += (v == 1) as usize;
            map.counts[index as usize] = v.saturating_add(1);
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
