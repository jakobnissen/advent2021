use ndarray::Array2;

pub fn solve(s: &str) -> (usize, usize) {
    let (map, mut counts1, last) = parse(s);
    let mut counts2 = Array2::zeros((26, 26));
    let mut part1 = 0;
    for iteration in 0..40 {
        counts2.fill(0);
        for ((a, b), count) in counts1.indexed_iter() {
            let y = map[[a, b]] as usize;
            if y == 255 {
                // sentinel value for no map
                continue;
            }
            counts2[[a, y]] += count;
            counts2[[y, b]] += count;
        }
        std::mem::swap(&mut counts1, &mut counts2);
        if iteration == 9 {
            part1 = monomer_counts(&counts1, last)
        }
    }
    (part1, monomer_counts(&counts1, last))
}

// The counts are counts of monomer pairs. To get counts for each monomer, just take the
// first of each pair. Then manually add the last monomer, which is not the first in any pair.
fn monomer_counts(counts: &Array2<usize>, last: u8) -> usize {
    let mut v = counts.sum_axis(ndarray::Axis(1)).to_vec();
    v[last as usize] += 1;
    let min = v
        .iter()
        .fold(usize::MAX, |a, b| if *b == 0 { a } else { a.min(*b) });
    let max = v.iter().fold(usize::MIN, |a, b| a.max(*b));
    max - min
}

fn parse(s: &str) -> (Array2<u8>, Array2<usize>, u8) {
    let nbyte = |s: &str, n: usize| s.bytes().nth(n).unwrap() - b'A';
    let mut map = Array2::from_elem((26, 26), 255u8);
    let mut counts = Array2::zeros((26, 26));

    let mut lines = s.trim().lines();
    let pol = lines.next().unwrap().trim();
    for (i, j) in pol.bytes().zip(pol.bytes().skip(1)) {
        counts[[(i - b'A') as usize, (j - b'A') as usize]] += 1;
    }
    lines.next().unwrap();
    for line in lines {
        let (from, to) = line.trim().split_once(" -> ").unwrap();
        map[[nbyte(from, 0) as usize, nbyte(from, 1) as usize]] = nbyte(to, 0);
    }
    (map, counts, nbyte(pol, pol.len() - 1))
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
        assert_eq!(super::solve(TEST_STR), (1588, 2188189693529));
    }
}
