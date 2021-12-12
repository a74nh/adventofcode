use std::collections::HashMap;
use itertools::Itertools;

type Displays = Vec<(Vec<String>, Vec<String>)>;

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Displays {
	input.lines().map(|line| {
		    let mut splitter = line.splitn(2, '|');
		    let mut l : Vec<String> = splitter.next().unwrap().split_whitespace().map(|l| l.chars().sorted().collect::<String>()).collect();
		    l.sort_by(|a, b| a.len().cmp(&b.len()));
		    let r : Vec<String> = splitter.next().unwrap().split_whitespace().map(|l| l.parse().unwrap()).collect();
		    (l,r)
        }).collect()
}

fn find_first_diff(small : &String, big: &String) -> u8 {
  for (i,b) in big.bytes().enumerate() {
    if small.bytes().nth(i).unwrap_or(0) != b { return b; }
  }
  unreachable!();
}

fn decode_display(display: &Vec<String>) -> HashMap::<u8, u8> {
  let mut segment_map = HashMap::<u8, u8>::new();

  //Find "a"
  segment_map.insert(find_first_diff(&display[0], &display[1]),b'a');

  //find "g"
  for c in b'a'..b'h' {
  	if ! display[0].contains(c as char)
    && ! display[1].contains(c as char)
    && ! display[2].contains(c as char)
    && display[4].contains(c as char)
    && display[5].contains(c as char)
    && display[6].contains(c as char)
    && display[7].contains(c as char)
    && display[8].contains(c as char)
    && display[9].contains(c as char) {
    	segment_map.insert(c,b'g');
    }
  }

  //Find "f" and "c"
  for c in display[0].bytes() {
  	if display[6].contains(c as char)
  	&& display[7].contains(c as char)
  	&& display[8].contains(c as char) {
  		segment_map.insert(c,b'f');
  	} else {
  		segment_map.insert(c,b'c');
  	}
  }

  //Find "d"
  for c in b'a'..b'h' {
  	if ! display[0].contains(c as char)
    && ! display[1].contains(c as char)
    && display[2].contains(c as char)
    && display[3].contains(c as char)
    && display[4].contains(c as char)
    && display[5].contains(c as char) {
  		segment_map.insert(c,b'd');
    }
  }

  //Find "b"
  for c in display[2].bytes() {
  	if ! segment_map.contains_key(&c) {
  	  segment_map.insert(c,b'b');
  	}
  }

  //Find "e"
  let mut v = 0;
  let mut k = 0;
  for c in b'a'..b'h' {
  	if ! segment_map.values().any(|&val| val == c) {
  		v = c;
  	}
  	if ! segment_map.contains_key(&c) {
  		k = c;
  	}
  }
  segment_map.insert(k,v);

  segment_map
}

fn decode_output(segment_map: HashMap::<u8, u8>, output: &Vec<String>) -> i64 {
	let mut numbers = 0;
	for num in output {
		numbers *= 10;
		if num.len() == 2 {
			numbers += 1;
		} else if num.len() == 3 {
			numbers += 7;
		} else if num.len() == 4 {
			numbers += 4;
		} else if num.len() == 7 {
			numbers += 8;
		} else if num.len() == 5 {
			let num_decoded : Vec<u8> = num.bytes().map(|n| *segment_map.get(&n).unwrap()).collect();
			if ! num_decoded.contains(&b'b') && ! num_decoded.contains(&b'f') { numbers += 2; }
			if ! num_decoded.contains(&b'b') && ! num_decoded.contains(&b'e') { numbers += 3; }
			if ! num_decoded.contains(&b'c') && ! num_decoded.contains(&b'e') { numbers += 5; }
		} else {
			let num_decoded : Vec<u8> = num.bytes().map(|n| *segment_map.get(&n).unwrap()).collect();
			if ! num_decoded.contains(&b'd') { numbers += 0; }
			if ! num_decoded.contains(&b'c') { numbers += 6; }
			if ! num_decoded.contains(&b'e') { numbers += 9; }
		}
	}
    numbers
}

#[aoc(day8, part1)]
fn part1(d: &Displays) -> usize {
  d.iter().map(|(l,_)| l.iter().map(|l| l.len()).filter(|len| [2, 4, 3, 7].contains(len)).count()).sum()
}

#[aoc(day8, part2)]
fn part2(d: &Displays) -> i64 {
  d.iter().map(|(l,r)| decode_output(decode_display(l), r)).sum()
}
