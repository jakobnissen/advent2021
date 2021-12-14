use std::collections::HashMap;
use std::hash::Hash;

pub fn solve(s: &str) -> (usize, usize) {
    let (map, mut counts1, first, last) = parse(s);
    let mut counts2 = HashMap::new();
    let mut part1 = 0;
    // Do polymer expansion
    for iteration in 0..40 {
        counts2.clear();
        for (&(a, b), &n) in counts1.iter() {
            match map.get(&(a, b)) {
                Some(&y) => {
                    update(&mut counts2, (a, y), n);
                    update(&mut counts2, (y, b), n);
                }
                None => {
                    counts2.insert((a, b), n);
                }
            }
        }
        std::mem::swap(&mut counts1, &mut counts2);
        if iteration == 9 {
            part1 = true_counts(&counts1, first, last)
        }
        //println!("{:?}", counts1);
    }
    (part1, true_counts(&counts1, first, last))
}

fn true_counts(m: &HashMap<(char, char), usize>, first: char, last: char) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for (&(a, b), &j) in m.iter() {
        update(&mut counts, a, j);
        update(&mut counts, b, j);
    }
    update(&mut counts, first, 1);
    update(&mut counts, last, 1);
    let v = counts.iter().map(|(_, &n)| n / 2).collect::<Vec<_>>();
    let min = v.iter().fold(usize::MAX, |a, b| a.min(*b));
    let max = v.iter().fold(usize::MIN, |a, b| a.max(*b));
    max - min
}

fn update<K>(m: &mut HashMap<K, usize>, k: K, n: usize)
where
    K: Eq + Hash + Copy,
{
    m.insert(k, m.get(&k).unwrap_or(&0) + n);
}

fn parse(
    s: &str,
) -> (
    HashMap<(char, char), char>,
    HashMap<(char, char), usize>,
    char,
    char,
) {
    let nchar = |s: &str, n: usize| s.chars().nth(n).unwrap();
    let mut map = HashMap::new();
    let mut counts = HashMap::new();

    let mut lines = s.trim().lines();
    let pol = lines.next().unwrap().trim();
    for (i, j) in pol.chars().zip(pol.chars().skip(1)) {
        update(&mut counts, (i, j), 1);
    }
    lines.next().unwrap();
    for line in lines {
        let (from, to) = line.trim().split_once(" -> ").unwrap();
        map.insert((nchar(from, 0), nchar(from, 1)), nchar(to, 0));
    }
    (
        map,
        counts,
        nchar(pol, 0),
        pol.chars().rev().nth(0).unwrap(),
    )
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
