use std::cmp;

#[aoc_generator(day21)]
fn parse(_: &str) -> (usize,usize) {
	(8,7)
}

fn roll(pos: &mut usize, score: &mut usize, dice: &mut usize) {
  *pos = ((*pos + *dice*3 + 2) %10 ) + 1;
  *dice = *dice + 3;
  *score += *pos;
}

#[aoc(day21, part1)]
fn part1(input: &(usize,usize)) -> usize {
  let (mut p1, mut p2) = input;
  let mut dice = 1;
  let mut p1score = 0;
  let mut p2score = 0;
  let mut rolls = 0;

  loop {
  	roll(&mut p1, &mut p1score, &mut dice);
 	rolls += 3;
 	if p1score>=1000 { return p2score * rolls; }
  	roll(&mut p2, &mut p2score, &mut dice);
 	rolls += 3;
 	if p2score>=1000 { return p1score * rolls; }
  }
}

fn roll_part2_p1(p1: usize, p1score: usize, p2: usize, p2score: usize, ways: usize, ways_to_roll : &[usize; 10], wins : &mut [usize; 2]) {
	for roll in 3..10 {
        let new_ways = ways * ways_to_roll[roll];
        let new_p = ((p1 + roll - 1) %10 ) + 1;
        let new_score = p1score + new_p;

        if new_score >= 21 {
            wins[0] += new_ways;
        }
        else {
            roll_part2_p2(new_p, new_score, p2, p2score, new_ways, ways_to_roll, wins);
        }
    }
}

fn roll_part2_p2(p1: usize, p1score: usize, p2: usize, p2score: usize, ways: usize, ways_to_roll : &[usize; 10], wins : &mut [usize; 2]) {
	for roll in 3..10 {
        let new_ways = ways * ways_to_roll[roll];
        let new_p = ((p2 + roll - 1) %10 ) + 1;
        let new_score = p2score + new_p;

        if new_score >= 21 {
            wins[1] += new_ways;
        }
        else {
            roll_part2_p1(p1, p1score, new_p, new_score, new_ways, ways_to_roll, wins);
        }
    }
}

#[aoc(day21, part2)]
fn part2(input: &(usize,usize)) -> usize {
  let (p1, p2) = input;

  let mut ways_to_roll: [usize; 10] = [0; 10];
  for i in 1..4 {
  	for j in 1..4 {
		for k in 1..4 {
			ways_to_roll[i + j + k]+=1;
		}
	}
  }
  let mut wins: [usize; 2] = [0; 2];
  roll_part2_p1(*p1, 0, *p2, 0, 1, &ways_to_roll, &mut wins);
  cmp::max(wins[0],wins[1])
}
