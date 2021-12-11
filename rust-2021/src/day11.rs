const SIZE: usize = 10;
const TOT_TILES: i64 = (SIZE*SIZE) as i64;
type Board = [[i64; SIZE]; SIZE];

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Board {
    let mut board : Board = [[0; SIZE]; SIZE];
    for (y,line) in input.lines().enumerate() {
    	for (x,h) in line.chars().enumerate() {
    		board[x][y] = h.to_digit(10).unwrap() as i64;
    	}
    }
    board
}

fn do_iteration(board: &mut Board) -> i64 {
  // Inc all by one
  for x in 0..SIZE {
  	for y in 0..SIZE {
  	  board[x][y] += 1;
  	}
  }
  // Propagate flashes until everything is stable
  let mut tot_flashes = 0;
  loop {
  	let mut new_flashes = 0;
  	for x in 0..SIZE {
  	  for y in 0..SIZE {
  	  	if board[x][y] > 9 {
  	  	  board[x][y] = 0;
  	  	  new_flashes += 1;
  	  	  for (bx,by) in [(x-1,y-1),(x,y-1),(x+1,y-1),(x-1,y),(x+1,y),(x-1,y+1),(x,y+1),(x+1,y+1)].iter() {
  	  	  	if *bx < SIZE && *by < SIZE && board[*bx][*by] != 0 { board[*bx][*by] += 1; }
  	  	  }
  	  	}
  	  }
    }
    if new_flashes == 0 { break; }
    tot_flashes += new_flashes;
  }
  tot_flashes
}

#[aoc(day11, part1)]
fn part1(b: &Board) -> i64 {
  let mut board = b.clone();
  (0..100).map(|_| do_iteration(&mut board)).sum()
}

#[aoc(day11, part2)]
fn part2(b: &Board) -> i64 {
  let mut board = b.clone();
  (1..).find(|_| do_iteration(&mut board) == TOT_TILES).unwrap()
}
