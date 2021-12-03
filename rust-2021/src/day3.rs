
#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
	let length = input.lines().next().unwrap().len();
	let mut lines = 0;
    let mut counts = vec![0; length];

	for powerstr in input.lines() {
		for (index,power) in powerstr.bytes().enumerate() {
			counts[index] += (power - 48) as usize;
		}
		lines += 1;
	}

    let mask = (1 << length) - 1;
	let gamma = counts.iter().fold(0, |acc, &n| { acc << 1 | if n > (lines - n) { 1 } else { 0 } });
	let epsilon = !gamma & mask;
	gamma*epsilon
}


fn get_bit_at(input: i64, n: usize, length: usize) -> i64 {
  if (input & (1 << (length-n-1))) != 0 { 1 } else { 0 }
}

fn get_rate(input: &str, a : i64, b : i64) -> i64 {
	let length = input.lines().next().unwrap().len();
    let mut oxygen : Vec<i64> = input.lines().map(|n| isize::from_str_radix(n, 2).unwrap() as i64).collect();

	for index in 0..length {
		if oxygen.len() > 1 {
			let oxygen_count : i64 = oxygen.iter().map(|&o| get_bit_at(o,index,length)).sum();
			let oxygen_msb = if oxygen_count >= (oxygen.len() as i64 - oxygen_count) { a } else { b };
			oxygen = oxygen.into_iter().filter(|&o| get_bit_at(o,index,length) == oxygen_msb).collect();
		}
	}
	*oxygen.first().unwrap()
}


#[aoc(day3, part2)]
fn part2(input: &str) -> i64 {
	get_rate(input, 1, 0)*get_rate(input, 0, 1)
}
