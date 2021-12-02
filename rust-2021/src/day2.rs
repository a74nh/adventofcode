
#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<(String, u32)> {
	input.lines().map(|line| {
			let mut i = line.split_whitespace();
            (
                i.next().unwrap().to_string(),
                i.next().unwrap().parse::<u32>().unwrap(),
            )}).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(String, u32)]) -> u32 {
	let (mut hpoz, mut depth) = (0, 0);
	for (direction, amount) in input {
		match direction.as_str() {
			"forward" => hpoz=hpoz+amount,
			"down" => depth=depth+amount,
			"up" => depth=depth-amount,
			_ => unreachable!(),
		}
	}
	hpoz*depth
}

#[aoc(day2, part2)]
pub fn part2(input: &[(String, u32)]) -> u32 {
	let (mut hpoz, mut depth, mut aim) = (0, 0, 0);
	for (direction, amount) in input {
		match direction.as_str() {
			"forward" => { hpoz=hpoz+amount; depth=depth+(aim*amount) },
			"down" => aim=aim+amount,
			"up" => aim=aim-amount,
			_ => unreachable!(),
		}
	}
	hpoz*depth
}
