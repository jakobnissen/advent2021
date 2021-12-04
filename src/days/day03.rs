pub fn solve<I>(lines: I) -> (usize, usize)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let (nums, bitwidth) = parse(lines);
    let gamma = most_common_bits(&nums, bitwidth);
    let part1 = gamma * (!gamma & ((1 << bitwidth) - 1));
    let part2 =
        filter_bit_criteria(&nums, bitwidth, true) * filter_bit_criteria(&nums, bitwidth, false);
    (part1, part2)
}

fn filter_bit_criteria(v: &[usize], bitwidth: usize, oxygen: bool) -> usize {
    let mut nums: Vec<_> = v.to_vec();
    for pos in (0..bitwidth).rev() {
        let bitcount = nums.iter().filter(|&&n| bitpos(n, pos)).count();
        let most_common_bit = 2 * bitcount >= nums.len();
        let target_bit = most_common_bit ^ !oxygen;
        nums.retain(|&n| bitpos(n, pos) == target_bit);
        if nums.len() < 2 {
            break;
        }
    }
    *nums.first().unwrap()
}

fn bitpos(n: usize, shift: usize) -> bool {
    (n >> shift & 1) == 1
}

fn most_common_bits(v: &[usize], bitwidth: usize) -> usize {
    let mut counts: Vec<usize> = vec![0; bitwidth];
    for i in v {
        for j in 0..bitwidth {
            counts[j] += (i >> (bitwidth - j - 1)) & 1;
        }
    }
    counts
        .iter()
        .fold(0, |n, count| (n << 1) | (2 * count > v.len()) as usize)
}

pub fn parse<I>(lines: I) -> (Vec<usize>, usize)
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut bitwidth: Option<usize> = None;
    let v: Vec<_> = lines
        .into_iter()
        .map(|line| {
            let str = line.as_ref().trim();
            let nbits = str.len();
            if nbits > (usize::BITS as usize) {
                panic!()
            };
            match bitwidth {
                None => bitwidth = Some(nbits),
                Some(n) => {
                    if n != nbits {
                        panic!()
                    }
                }
            }
            usize::from_str_radix(str, 2).unwrap()
        })
        .collect::<Vec<usize>>();
    (v, bitwidth.unwrap())
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    #[test]
    fn test() {
        assert_eq!(super::solve(crate::test_lines(TEST_STR)), (198, 230));
    }
}
