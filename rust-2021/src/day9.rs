use std::collections::HashMap;

type Board = HashMap::<(i64, i64), i64>;

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Board {
    let mut board = Board::new();
    for (y,line) in input.lines().enumerate() {
    	for (x,h) in line.chars().enumerate() {
    		board.insert((x as i64,y as i64),h.to_digit(10).unwrap() as i64);
    	}
    }
    board
}

fn get_low_points(board: &Board, inc: i64) -> Board {
  let mut low_points = Board::new();

  for ((x,y), value) in board {
  	if value < board.get(&(*x-1,*y)).unwrap_or(&i64::MAX)
  	&& value < board.get(&(*x+1,*y)).unwrap_or(&i64::MAX)
  	&& value < board.get(&(*x,*y-1)).unwrap_or(&i64::MAX)
  	&& value < board.get(&(*x,*y+1)).unwrap_or(&i64::MAX) {
  		low_points.insert((*x,*y), *value+inc as i64);
  	}
  }
  low_points
}

fn get_num_higher_points(board: &mut Board, x: i64, y: i64) -> i64 {
  let value = *board.get(&(x,y)).unwrap_or(&-1);
  if value >= 9 { return 0; }
  *board.entry((x,y)).or_insert(0) = -1;
  let mut result = 1;
  for (bx,by) in [(x-1,y),(x+1,y),(x,y-1),(x,y+1)].iter() {
  	if board.get(&(*bx,*by)).unwrap_or(&-1) > &value {
  		result += get_num_higher_points(board, *bx, *by);
  	}
  }
  result
}

#[aoc(day9, part1)]
fn part1(board: &Board) -> i64 {
  get_low_points(board,1).values().sum()
}

#[aoc(day9, part2)]
fn part2(b: &Board) -> i64 {
  let mut board = b.clone();
  let low_points = get_low_points(&board,0);

  let mut basin_sizes: Vec<i64> = Vec::new();
  for ((x,y), _value) in low_points {
  	basin_sizes.push(get_num_higher_points(& mut board,x,y));
  }

  basin_sizes.sort_unstable();
  basin_sizes.iter().rev().take(3).product()
}

