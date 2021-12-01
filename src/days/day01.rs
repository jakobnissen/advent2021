pub fn solve<T: Iterator<Item=String>>(lines: T) -> (usize, usize) {
    let nums: Vec<_> = lines.map(|s| {
        s.parse::<usize>().unwrap_or_else(|_| {
            println!("Cannot parse as usize: \"{}\"", s);
            std::process::exit(1);
        })
    }).collect();

    let part1 = nums.iter()
        .zip(&nums[1..])
        .filter(|(prev, next)| {next > prev}).count();
    let part2 = nums.iter()
        .zip(&nums[3..])
        .filter(|(prev, next)| {next > prev}).count();
    (part1, part2)
}
