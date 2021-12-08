pub fn solve(s: &str) -> (usize, usize) {
    let records = s.trim().lines().map(Record::parse).collect::<Vec<_>>();
    let part1 = records.iter().map(Record::count_certain).sum();
    let part2 = records.iter().map(Record::decode).sum();
    (part1, part2)
}


struct SegmentSet(u8);

impl SegmentSet {
    fn parse(s: &str) -> Self {
        SegmentSet(s.bytes().fold(0, |x, i| {
            x | 1 << ((i - b'a') & 7)
        }))
    }

    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn intersect_length(&self, other: &SegmentSet) -> usize {
        (self.0 & other.0).count_ones() as usize
    }
}

struct Record {
    signal: Vec<SegmentSet>,
    output: Vec<SegmentSet>,
}

impl Record {
    fn parse(s: &str) -> Self {
        let vecfrom = |s: &str| {s.split_ascii_whitespace().map(SegmentSet::parse).collect::<Vec<_>>()};
        let (signal, output) = s.trim().split_once(" | ").unwrap();
        Record{signal: vecfrom(signal), output: vecfrom(output)}
    }

    fn decode(&self) -> usize {
        let one = self.signal.iter().find(|s| s.len() == 2).unwrap();
        let four = self.signal.iter().find(|s| s.len() == 4).unwrap();
        self.output.iter().map(|set| {
            let (len, o1, o4) = (set.len(), set.intersect_length(one), set.intersect_length(four));
            match (len, o1, o4) {
                (6, 2, 3) => 0,
                (2, _, _) => 1,
                (5, 1, 2) => 2,
                (5, 2, _) => 3,
                (4, _, _) => 4,
                (5, 1, 3) => 5,
                (6, 1, _) => 6,
                (3, _, _) => 7,
                (7, _, _) => 8,
                (6, 2, 4) => 9,
                _ => unreachable!()
            }
        }).fold(0, |n, i| {10*n + i})
    }

    fn count_certain(&self) -> usize {
        self.output.iter().filter(|s| {
            [2, 3, 4, 7].contains(&s.len())
        }).count()
    }
}

/* fn collect_into_array<T, const N: usize>(iter: &mut impl Iterator<Item=T>)->[T; N]{
    [();N].map(|_|{
        iter.next().expect("Not enough elements to complete array.")
    })
}
let base_array = [0i32;5];
let mut array_iter = base_array.into_iter();
let first_3_elements : [i32; 3] = collect_into_array(&mut array_iter);
first_3_elements */

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (26, 61229));
    }
}
