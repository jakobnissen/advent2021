struct SquidGrid([[u8; 10]; 10]);

fn cartesian<T, E>(a: T, b: T) -> impl Iterator<Item=(E, E)>
where T: Iterator<Item=E> + Clone,
E: Copy
{
    a.flat_map(move |x| b.clone().map(move |y| (x, y)))
}

impl SquidGrid {
    fn parse(s: &str) -> Self {
        let mut arr = [[0u8; 10]; 10];
        for (lineno, line) in s.trim().lines().enumerate() {
            for (colno, b) in line.trim().as_bytes().iter().enumerate() {
                arr[lineno][colno] = b - b'0'
            }
        }
        Self(arr)
    }

    // Increase all squid's energy by 1
    fn increase_all(&mut self) {
        for (i, j) in cartesian(0..10, 0..10) {
            self.0[i][j] += 1;
        }
    }

    // Set all squids that have flashed this iteration to 0.
    fn reset_flashed(&mut self) {
        for (i, j) in cartesian(0..10, 0..10) {
            if self.0[i][j] > 9 {
                self.0[i][j] = 0;
            }
        }
    }

    // Flash the squid at grid[row][col]. We set its energy level to 20
    // to indicate it should not be flashing any more this iteration.
    fn flash(&mut self, row: isize, col: isize) {
        self.0[row as usize][col as usize] = 20;
        for (&drow, &dcol) in cartesian([-1, 0, 1].iter(), [-1, 0, 1].iter()) {
            if !(drow == 0 && dcol == 0)
                && (0..10).contains(&(row + drow))
                && (0..10).contains(&(col + dcol))
            {
                self.0[(row + drow) as usize][(col + dcol) as usize] += 1;
            }
        }
    }
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut grid = SquidGrid::parse(s);
    let mut flashes_first_100_its = 0;
    let mut iteration_at_all_flash: Option<usize> = None;
    for iteration in 1.. {
        let mut this_iteration_flashes = 0;
        grid.increase_all();
        let mut flashing_converged = false;

        while !flashing_converged {
            flashing_converged = true;
            for (i, j) in cartesian(0..10, 0..10) {
                // Squids that have already flashed has energy at 20 or above, and should not flash
                if (10..20).contains(&grid.0[i as usize][j as usize]) {
                    grid.flash(i, j);
                    this_iteration_flashes += 1;
                    flashing_converged = false;
                }
            }
        }
        grid.reset_flashed();

        if iteration_at_all_flash.is_none() && this_iteration_flashes == 100 {
            iteration_at_all_flash = Some(iteration);
        }
        if iteration <= 100 {
            flashes_first_100_its += this_iteration_flashes;
        }
        if iteration_at_all_flash.is_some() && iteration >= 100 {
            break;
        }
    }
    (flashes_first_100_its, iteration_at_all_flash.unwrap())
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (1656, 1656));
    }
}
