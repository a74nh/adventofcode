use std::cmp;
type Crabs = Vec::<i64>;

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Crabs {
	input.lines().next().unwrap().split(",").map(|s| s.parse().unwrap()).collect()
}

fn calc_fuel<F>(crabs: &Crabs, cost: F) -> i64 where F: Fn(i64) -> i64 {
  let mut fuel = i64::MAX;
  for pos in 0..crabs.iter().max().unwrap()+1 {
  	fuel = cmp::min(crabs.iter().map(|c| cost((c-pos).abs())).sum(), fuel);
  }
  fuel
}

#[aoc(day7, part1)]
fn part1(crabs: &Crabs) -> i64 {
	calc_fuel(crabs, |n| n )
}

#[aoc(day7, part2)]
fn part2(crabs: &Crabs) -> i64 {
	calc_fuel(crabs, |n| (n * (n+1))/2 )
}


