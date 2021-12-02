
#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<i64> {
	input.lines().map(|line| line.parse::<i64>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i64]) -> u32 {
	input.windows(2).filter(|&w| w[1]>w[0]).count() as u32
}

#[aoc(day1, part2)]
pub fn part2(input: &[i64]) -> u32 {
	let windows_of_3: Vec<i64> = input.windows(3).map(|w| w.iter().sum()).collect();
    part1(&windows_of_3)
}
