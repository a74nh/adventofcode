use itertools::Itertools;

type Coord = (i64,i64,i64);
type Locations = Vec<Coord>;
//Beacons in .0, and scanners in .1
type ScannerData = (Locations, Locations);
type Scanners = Vec<ScannerData>;

#[aoc_generator(day19)]
fn gen(input: &str) -> Scanners {
    input.split("\n\n")
         .map(|sec| { (sec.lines().skip(1)
						  .filter_map(|line| { line.split(',').filter_map(|n| n.parse().ok()).collect_tuple() })
						  .collect(),
					   vec![(0,0,0); 1]) }).collect()
}

fn rotations() -> impl Iterator<Item = fn(Coord) -> Coord> {
    [   // front facing, rotate around x
        |(x, y, z): Coord| (x, y, z),   |(x, y, z): Coord| (x, z, -y),  |(x, y, z): Coord| (x, -y, -z),  |(x, y, z): Coord| (x, -z, y),
        // looking one side, rotate around y
        |(x, y, z): Coord| (y, -x, z),  |(x, y, z): Coord| (-z, -x, y), |(x, y, z): Coord| (-y, -x, -z), |(x, y, z): Coord| (z, -x, -y),
        // looking back, rotate around x
        |(x, y, z): Coord| (-x, -y, z), |(x, y, z): Coord| (-x, z, y),  |(x, y, z): Coord| (-x, y, -z),  |(x, y, z): Coord| (-x, -z, -y),
        // looking the other side, rotate around y
        |(x, y, z): Coord| (-y, x, z),  |(x, y, z): Coord| (-z, x, -y), |(x, y, z): Coord| (y, x, -z),   |(x, y, z): Coord| (z, x, y),
        // looking up, rotate around z
        |(x, y, z): Coord| (-z, y, x),  |(x, y, z): Coord| (y, z, x),   |(x, y, z): Coord| (z, -y, x),   |(x, y, z): Coord| (-y, -z, x),
        // looking down, rorate around z
        |(x, y, z): Coord| (z, y, -x),  |(x, y, z): Coord| (y, -z, -x), |(x, y, z): Coord| (-y, z, -x),  |(x, y, z): Coord| (-z, -y, -x),
    ].into_iter()
}

fn manhatten(l: Coord, r: Coord) -> i64 {
	(l.0-r.0).abs() + (l.1-r.1).abs() + (l.2-r.2).abs()
}

fn try_merge_at(ln: usize, rn: usize, l: &ScannerData, r: &ScannerData, required_matches: usize) -> Option<ScannerData> {
	let r_reseater = (r.0[rn].0 - l.0[ln].0, r.0[rn].1 - l.0[ln].1, r.0[rn].2 - l.0[ln].2); 

	// Reseat all right hand side beacons.
	let mut merged_beacons = r.0.iter().map(|rb| (rb.0-r_reseater.0, rb.1-r_reseater.1, rb.2-r_reseater.2)).collect_vec();

	// Merge left into merged, counting matches.
	let matches = l.0.iter().filter_map(|lb| if merged_beacons.contains(lb) {Some(lb)} else {merged_beacons.push(*lb); None}).count();
	if matches < required_matches { return None; }

	// Merge scanners.
	let mut merged_scanners = l.1.clone();
	merged_scanners.extend(r.1.iter().map(|rs| (rs.0-r_reseater.0, rs.1-r_reseater.1, rs.2-r_reseater.2)));

	Some((merged_beacons, merged_scanners))
}

fn try_merge(l: &ScannerData, r: &ScannerData, required_matches: usize) -> Option<ScannerData> {
	if l.0.len() > 0 && r.0.len() > 0 {
		for rot in rotations() {
			let rotated = (r.0.iter().map(|c| rot(*c)).collect_vec(), r.1.iter().map(|c| rot(*c)).collect_vec());
			for i in 0..l.0.len() {
				for j in 0..r.0.len() {
					if let Some(merged) = try_merge_at(i,j,&l,&rotated,required_matches) { return Some(merged); }
				}
		    }
		}
	}
	None
}

fn do_it(s: &Scanners) -> ScannerData {
	let mut scanners = s.clone();
	loop {
		let mut found = false;
		'outer: for i in 0..scanners.len() {
			for j in i+1..scanners.len() {
		    	if let Some(merged_scanner) = try_merge(&scanners[i],&scanners[j],12) {
		    		// println!("found {:?}", (i,j));
		    		scanners[i].0 = merged_scanner.0;
		    		scanners[i].1 = merged_scanner.1;
		    		scanners[j].0 = Locations::new();
		    		scanners[j].1 = Locations::new();
		    		found = true;
		    		continue 'outer;
		    	}
		    }
		}
		if !found { break; }
	}
	scanners[0].clone()
}

#[aoc(day19, part1)]
fn part1(s: &Scanners) -> usize {
	do_it(s).0.len()
}

#[aoc(day19, part2)]
fn part2(s: &Scanners) -> i64 {
	do_it(s).1.into_iter().tuple_combinations().map(|(l, r)| manhatten(l,r)).max().unwrap()
}
