pub fn solve(s: &str) -> (usize, usize) {
    let (nums, mut boards) = parse(s);
    let index_scores = get_scores(&nums, &mut boards);
    return (
        index_scores.first().unwrap().1,
        index_scores.last().unwrap().1,
    );
}

fn get_scores(nums: &[usize], boards: &mut Vec<Board>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for &num in nums {
        // This is equivalent to drain_filter, which is currently an unstable feature
        let mut i: usize = 0;
        while i < boards.len() {
            boards[i].add_number(num);
            if boards[i].check_board() {
                result.push((i, boards[i].score(num)));
                boards.remove(i);
            } else {
                i += 1;
            }
        }
        if boards.is_empty() {
            return result;
        }
    }
    unreachable!()
}

struct Board {
    v: Vec<(usize, bool)>,
}

impl Board {
    fn add_number(&mut self, num: usize) {
        for i in 0..(self.v.len()) {
            if self.v[i].0 == num {
                self.v[i] = (num, true)
            }
        }
    }

    fn check_board(&self) -> bool {
        (0..5).into_iter().any(|i| {
            ((5 * i)..(5 * i + 5)).all(|j| self.v[j].1) || (i..25).step_by(5).all(|j| self.v[j].1)
        })
    }

    fn score(&self, n: usize) -> usize {
        self.v
            .iter()
            .filter(|(_, b)| !b)
            .map(|(v, _)| v)
            .sum::<usize>()
            * n
    }
}

fn parse(s: &str) -> (Vec<usize>, Vec<Board>) {
    let blocks = s.split("\n\n").collect::<Vec<_>>();
    let nums = blocks[0]
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let boards: Vec<Board> = blocks[1..]
        .iter()
        .map(|&block| {
            let mut v: Vec<usize> = Vec::new();
            for line in block.split('\n').map(|s| s.trim()) {
                let vi: Vec<usize> = line
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                v.extend(vi);
            }
            Board {
                v: v.iter().map(|&n| (n, false)).collect::<Vec<_>>(),
            }
        })
        .collect();
    (nums, boards)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (4512, 1924));
    }
}
