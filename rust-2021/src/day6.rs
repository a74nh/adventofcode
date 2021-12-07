
type Fishes = [usize; 9];

#[aoc_generator(day6)]
pub fn parse(input: &str) -> Fishes {
	let mut fish_array: Fishes = [ 0,0,0,0,0,0,0,0,0 ];
	let fish_list : Vec<usize> = input.lines().next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
	for fish in fish_list {
		fish_array[fish] += 1;
	}
	fish_array
}

fn do_fishes(f: &Fishes, days : usize) -> usize {
	let mut fishes = f.clone();
	for _day in 0..days {
		let mut new_fishes: Fishes = [ 0,0,0,0,0,0,0,0,0 ];
		for (i, fish) in fishes.iter().enumerate() {
 			if i == 0 {
 				new_fishes[6] += fish;
 				new_fishes[8] += fish;
 			} else {
 				new_fishes[i-1] += fish;
 			}
 		}
 		fishes = new_fishes;
	}
	fishes.iter().sum()
}

#[aoc(day6, part1)]
fn part1(f: &Fishes) -> usize {
  do_fishes(f,80)
}


#[aoc(day6, part2)]
fn part2(f: &Fishes) -> usize {
  do_fishes(f,256)
}
