
fn get_first_err(line: &str) -> u8 {
  let mut parse: Vec<u8> = Vec::new();
  for b in line.bytes() {
    match b {
        b'(' | b'[' | b'{' | b'<' => parse.push(b),
        b')' => if parse.pop().unwrap() != b'(' { return b },
        b']' => if parse.pop().unwrap() != b'[' { return b },
        b'}' => if parse.pop().unwrap() != b'{' { return b },
        b'>' => if parse.pop().unwrap() != b'<' { return b },
        _ => unreachable!()
    }
  }
  b' '
}

fn finish_line(line: &str) -> i64 {
  let mut parse: Vec<u8> = Vec::new();
  for b in line.bytes() {
    match b {
        b'(' | b'[' | b'{' | b'<' => parse.push(b),
        b')' => if parse.pop().unwrap() != b'(' { unreachable!() },
        b']' => if parse.pop().unwrap() != b'[' { unreachable!() },
        b'}' => if parse.pop().unwrap() != b'{' { unreachable!() },
        b'>' => if parse.pop().unwrap() != b'<' { unreachable!() },
        _ => unreachable!()
    }
  }
  let mut score = 0;
  for b in parse.iter().rev() {
    score *= 5;
    match b {
      b'(' => score += 1,
      b'[' => score += 2,
      b'{' => score += 3,
      b'<' => score += 4,
      _ => unreachable!()
    }
  }
  score
}

#[aoc(day10, part1)]
fn part1(input: &str) -> i64 {
  input.lines().map(|line| get_first_err(line)).map(|c| match c {
    b')' => 3,
    b']' => 57,
    b'}' => 1197,
    b'>' => 25137,
    b' ' => 0,
    _ => unreachable!()
  }).sum()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> i64 {
 let mut x : Vec<i64> = input.lines().filter(|line| get_first_err(line) == b' ' ).map(|line| finish_line(line)).collect();
 x.sort();
 x[(x.len()/2)]
}

