use regex::Regex;
use std::cmp;

type Range = (i64,i64);
type Command = (bool,Entry);
type Commands = Vec<Command>;
type Board = Vec<Entry>;

#[derive(Clone, Copy)]
struct Entry {x: Range, y: Range, z: Range}

impl Entry {

	fn count(&self) -> usize {
		(self.z.0..=self.z.1).count() * (self.y.0..=self.y.1).count() * (self.x.0..=self.x.1).count()
	}

	fn from(x: Range, y: Range, z: Range) -> Self {
        Self { x, y, z }
    }

	fn contains(&self, contained: &Entry) -> bool {
		contained.x.0 >= self.x.0 && contained.x.1 <= self.x.1
		&& contained.y.0 >= self.y.0 && contained.y.1 <= self.y.1
		&& contained.z.0 >= self.z.0 && contained.z.1 <= self.z.1
	}

	fn overlaps(&self, rhs: &Entry) -> bool {
		!((self.x.0 > rhs.x.1) || (self.x.1 < rhs.x.0)
		|| (self.y.0 > rhs.y.1) || (self.y.1 < rhs.y.0)
		|| (self.z.0 > rhs.z.1) || (self.z.1 < rhs.z.0))
	}

	// Return multiple Entry's that don't overlap with rhs
	fn split_overlapping(&self, rhs: &Entry) -> Board {
		let mut board = Board::new();
		if self.overlaps(rhs) {
			// Split self into multiple slices that don't overlap with rhs
			if self.z.0 < rhs.z.0 { board.push(Entry::from(self.x, self.y, (self.z.0, rhs.z.0-1))); }
			if self.z.1 > rhs.z.1 { board.push(Entry::from(self.x, self.y, (rhs.z.1+1, self.z.1))); }
			let zrange = (cmp::max(rhs.z.0,self.z.0),cmp::min(rhs.z.1,self.z.1));
			if self.y.0 < rhs.y.0 { board.push(Entry::from(self.x, (self.y.0, rhs.y.0-1), zrange)); }
			if self.y.1 > rhs.y.1 { board.push(Entry::from(self.x, (rhs.y.1+1, self.y.1), zrange)); }
			let yrange = (cmp::max(rhs.y.0,self.y.0),cmp::min(rhs.y.1,self.y.1));
			if self.x.0 < rhs.x.0 { board.push(Entry::from((self.x.0, rhs.x.0-1), yrange, zrange)); }
			if self.x.1 > rhs.x.1 { board.push(Entry::from((rhs.x.1+1, self.x.1), yrange, zrange)); }
		} else {
			board.push(*self);
		}
  		board
	}

}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    	write!(f, "({:?}..{:?})", self.x.0, self.x.1)?;
    	write!(f, "({:?}..{:?})", self.y.0, self.y.1)?;
    	write!(f, "({:?}..{:?})", self.z.0, self.z.1)?;
    	write!(f, "[{:?}]", self.count())?;
        write!(f, "")
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Commands {
	let reg = Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();

	input.lines().map(|l| { let c=reg.captures(l).unwrap();
        (if &c[1] == "on" {true} else {false},
         Entry::from((c[2].parse::<i64>().unwrap(), c[3].parse::<i64>().unwrap()),
        			 (c[4].parse::<i64>().unwrap(), c[5].parse::<i64>().unwrap()),
					 (c[6].parse::<i64>().unwrap(), c[7].parse::<i64>().unwrap())))
    }).collect()
}

#[aoc(day22, part1)]
fn part1(commands: &Commands) -> usize {
	let max = Entry::from((-50,50),(-50,50),(-50,50));
	let mut board = Board::new();
	for (c_state,c_entry) in commands {
		if !max.contains(c_entry) { continue; }
		board = board.into_iter().flat_map(|e| e.split_overlapping(c_entry)).collect();
		if *c_state { board.push(*c_entry); }
	}
	board.iter().fold(0, |acc,entry| acc+entry.count())
}

#[aoc(day22, part2)]
fn part2(commands: &Commands) -> usize {
	let mut board = Board::new();
	for (c_state,c_entry) in commands {
		board = board.into_iter().flat_map(|e| e.split_overlapping(c_entry)).collect();
		if *c_state { board.push(*c_entry); }
	}
	board.iter().fold(0, |acc,entry| acc+entry.count())
}
