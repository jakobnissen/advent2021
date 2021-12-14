fn parse(s: &str) -> (Vec<u8>, [[u8; 28]; 28]) {
    let mut lut = [[0u8; 28]; 28];
    let mut lines = s.trim().lines();
    let v = lines.next().unwrap().trim().as_bytes().iter().map(|b| b - b'A').collect::<Vec<_>>();
    lines.next().unwrap();
    for line in lines {
        let (from, to) = line.trim().split_once(" -> ").unwrap();
        let i1 = from.bytes().nth(0).unwrap() - b'A';
        let i2 = from.bytes().nth(1).unwrap() - b'A';
        let y =  to.bytes().nth(0).unwrap() - b'A';
        lut[i1 as usize][i2 as usize] = y
    }
    (v, lut)
}

pub fn part1(s: &str) -> usize {
    let (mut v1, lut) = parse(s);
    let mut v2: Vec<u8> = Vec::new();
    for _ in 0..10 {
        v2.truncate(0);
        for (i, j) in v1.iter().zip(v1[1..].iter()) {
            let y = lut[*i as usize][*j as usize];
            v2.push(*i);
            if y != 0 {
                v2.push(y)
            }
        }
        v2.push(*v1.last().unwrap());
        //println!("{:?}", v2);
        std::mem::swap(&mut v1, &mut v2);
    }
    let mut counts: Vec<usize> = vec![0; 28];
    for i in v1.iter() {
        counts[*i as usize] += 1;
    }
    let max = counts.iter().fold(0usize, |a, b| a.max(*b));
    let min = counts.iter().fold(usize::MAX, |a, b| {
        if *b == 0 {a} else {a.min(*b)}
    });
    max - min
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";

    #[test]
    fn test() {
        assert_eq!(super::part1(TEST_STR), 1588);
    }
}
