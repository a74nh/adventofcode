use std::collections::HashMap;

type Board = HashMap::<(usize, usize, usize), usize>;


#[aoc_generator(day4)]
pub fn parse(input: &str) -> (Vec<usize>, Board, usize) {

	let mut lines = input.lines();
	let picks : Vec<usize> = lines.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let mut boards = Board::new();
    let mut board_num = 0;
    let mut reset = false;
    let mut row = 0;

	for line in lines {
		if line.is_empty() {
			row = 0;
			reset = true;
		} else {
			if reset {
				reset = false;
				board_num += 1;
			}
			for (column,val) in line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).enumerate() {
				boards.insert((board_num, row, column), val);
			}
			row+=1;
		}
	}
	(picks,boards,board_num)
}

#[aoc(day4, part1)]
fn part1((picks,b,_n): &(Vec<usize>, Board, usize)) -> usize {
	let mut boards = b.clone();
	let mut board_match_rows = HashMap::<(usize, usize), usize>::new();
	let mut board_match_cols = HashMap::<(usize, usize), usize>::new();
	let max_rows = 5;
	let max_cols = 5;
	let mut winning_board = usize::MAX;
	let mut winning_pick = 0;

    'outer: for pick in picks {
    	// Find all instance of the pick
		let mut founds : Vec<(usize, usize, usize)> = Vec::new();
    	for (key, value) in &boards {
    		if value == pick {
    			founds.push(key.to_owned());
    		}
    	}
    	// Remove each found instance from the board and check for a winner
	    for (board_num, row, col) in founds.iter() {
	        boards.remove(&(*board_num, *row, *col));
	        *board_match_rows.entry((*board_num, *row)).or_insert(0) += 1;
	        *board_match_cols.entry((*board_num, *col)).or_insert(0) += 1;
	        if board_match_rows.get(&(*board_num, *row)).unwrap() == &max_rows ||
	           board_match_cols.get(&(*board_num, *col)).unwrap() == &max_cols {
	        	winning_board = *board_num;
	        	winning_pick = *pick;
	        	break 'outer;
	        }
	    }
    }
    // Calculate remaining values on winner board
    let mut sum = 0;
    for ((board_num, _row, _col), value) in &boards {
    	if board_num == &winning_board {
    		sum += value;
    	}
    }
	winning_pick*sum
}


#[aoc(day4, part2)]
fn part2((picks,b,num_boards): &(Vec<usize>, Board, usize)) -> usize {
	let mut boards = b.clone();
	let mut board_match_rows = HashMap::<(usize, usize), usize>::new();
	let mut board_match_cols = HashMap::<(usize, usize), usize>::new();
	let max_rows = 5;
	let max_cols = 5;
	let mut winning_boards : Vec<usize> = Vec::new();
	let mut winning_pick = 0;

    for pick in picks {
    	// Find all instance of the pick
		let mut founds : Vec<(usize, usize, usize)> = Vec::new();
    	for (key, value) in &boards {
    		if value == pick {
    			founds.push(key.to_owned());
    		}
    	}
    	// Remove each found instance from the board and check for a winner
	    for (board_num, row, col) in founds.iter() {
	        boards.remove(&(*board_num, *row, *col));
	        *board_match_rows.entry((*board_num, *row)).or_insert(0) += 1;
	        *board_match_cols.entry((*board_num, *col)).or_insert(0) += 1;
	        if ! winning_boards.contains(board_num) {
		        if board_match_rows.get(&(*board_num, *row)).unwrap() == &max_rows ||
		           board_match_cols.get(&(*board_num, *col)).unwrap() == &max_cols {
		        	winning_boards.push(*board_num);
		        	winning_pick = *pick;
		        }
	    	}
	    }
	    // If all boards have won then break now
	    if winning_boards.len() == *num_boards {
	    	break;
	    }
    }
    // Calculate remaining values on winner board
    let mut sum = 0;
    let winning_board = winning_boards.last().unwrap();
    for ((board_num, _row, _col), value) in &boards {
    	if board_num == winning_board {
    		sum += value;
    	}
    }
	winning_pick*sum
}
