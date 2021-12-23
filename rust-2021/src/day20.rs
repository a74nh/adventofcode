use std::collections::HashMap;
use std::collections::BTreeSet;
use std::cmp;

type Enhancer = HashMap::<usize, bool>;
type Image = BTreeSet::<(i64, i64)>;
type Input = (Enhancer, Image);

struct Boundaries{
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

#[aoc_generator(day20)]
pub fn parse(input: &str) -> Input {
	let mut lines = input.lines();
	let mut enhancer = Enhancer::new();
	for (index, c) in lines.next().unwrap().chars().enumerate() {
		enhancer.insert(index, if c=='#' {true} else {false});
	}
	lines.next();
	let mut image = Image::new();
	for (y,line) in lines.enumerate() {
		for (x, c) in line.chars().enumerate() {
			if c=='#' { image.insert((x as i64,y as i64)); }
		}
	}
	(enhancer, image)
}



fn enhance(enhancer :&Enhancer, image :&Image, outside_lit :bool) -> Image {
	let boundaries = get_boundaries(&image);
	let mut new_image = Image::new();

	for y in boundaries.ymin-1..boundaries.ymax+2 {
		for x in boundaries.xmin-1..boundaries.xmax+2 {
			let mut epos = 0;

			for dy in -1..2 {
				for dx in -1..2 {
					epos = epos << 1;
					if !(boundaries.xmin..=boundaries.xmax).contains(&(&x+dx))
					|| !(boundaries.ymin..=boundaries.ymax).contains(&(&y+dy)) {
						if outside_lit { epos+=1; }
					} 
					else if image.contains(&(x+dx,y+dy)) { epos+=1; }
				}
			}
			if epos>=512 { unreachable!(); }
			if enhancer[&epos] { new_image.insert((x,y)); }
		}
	}

	new_image
}

fn get_boundaries(image :&Image) -> Boundaries {
	image.iter().fold(Boundaries { xmin: i64::MAX, xmax: i64::MIN, ymin: i64::MAX, ymax: i64::MIN },
		|mut b, (x,y)| { b.xmin = cmp::min(b.xmin,*x);
						 b.xmax = cmp::max(b.xmax,*x);
						 b.ymin = cmp::min(b.ymin,*y);
						 b.ymax = cmp::max(b.ymax,*y);
						 b })
}

fn do_it(enhancer :&Enhancer, i :&Image, n: usize) -> usize {
	let mut image = i.clone();
	let mut outside_lit = false;

	for _ in 0..n {
		image = enhance(enhancer, &image, outside_lit);
		outside_lit = !outside_lit;
	}
	image.len()
}


#[aoc(day20, part1)]
fn part1(input: &Input) -> usize {
	let (e, i) = input;
	do_it(e,i,2)
}

#[aoc(day20, part2)]
fn part2(input: &Input) -> usize {
	let (e, i) = input;
	do_it(e,i,50)
}



