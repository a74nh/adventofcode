use std::cmp;

type Target = (i64, i64, i64, i64);
type Results = Vec<i64>;

#[aoc_generator(day17)]
fn parse(_: &str) -> Target {
	//TODO: Parse input
    (236, 262, -78, -58)
}

fn inside_target(x: i64, y: i64, target: &Target) -> bool {
	let (tx_min, tx_max, ty_min, ty_max) = target;
	return x >= *tx_min && x <= *tx_max && y >= *ty_min && y <= *ty_max
}

fn missed_target(_: i64, y: i64, target: &Target) -> bool {
	let (_, _, ty_min, _) = target;
	return y < *ty_min
}

fn solve_for(start_vx: i64, start_vy: i64, target: &Target) -> i64 {
	let mut x = 0;
	let mut y = 0;
	let mut vx = start_vx;
	let mut vy = start_vy;
	let mut ymax = 0;

	loop {
		if inside_target(x,y,target) { return ymax }
		if missed_target(x,y,target) { return i64::MIN }
        x += vx;
        y += vy;
        if vx>0 { vx-=1 } else if vx<0 { vx+=1 };
        vy -= 1;
        ymax = cmp::max(y,ymax);
	}
}

#[aoc(day17, part1)]
fn part1(target: &Target) -> i64 {
	let mut results = Results::new();
	//TODO: Don't brute force this
	for y in 0..100 {
		for x in 0..100 {
			let r = solve_for(x,y,target);
			if r > i64::MIN { results.push(r) }
		}
	}
    *results.iter().max().unwrap()
}

#[aoc(day17, part2)]
fn part2(target: &Target) -> usize {
	let mut results = Results::new();
	//TODO: Don't brute force this
	for y in -1000..1000 {
		for x in -1000..1000 {
			let r = solve_for(x,y,target);
			if r > i64::MIN { results.push(r) }
		}
	}
    results.iter().count()
}

