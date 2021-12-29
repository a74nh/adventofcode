use hashbrown::HashMap;
use std::collections::BinaryHeap;

type Board = Vec<Vec<i64>>;
type Distances = HashMap<Board, i64>;
type Frontier = BinaryHeap<(i64, Board)>;

#[aoc_generator(day23)]
fn parse(input: &str) -> Board {
    input.lines().map(|l| l.bytes().map(|c|
            match c {
                b'.' => 0,
                b'A' => 1,
                b'B' => 2,
                b'C' => 3,
                b'D' => 4,
                _ => -1,
            }
        ).collect()).collect()
}

fn boards_eq(left: &Board, right: &Board) -> bool {
    left.iter().zip(right.iter()).filter(|&(a, b)| a == b).count() == left.len()
}

fn cost(c :i64) -> i64 {
    10_i64.pow(c as u32-1)
}

// fn print_board(board: &Board) {
//     for (_,line) in board.iter().enumerate() {
//         for (_, c) in line.iter().enumerate() {
//             match c {
//                 0 => print!("."),
//                 1 => print!("A"),
//                 2 => print!("B"),
//                 3 => print!("C"),
//                 4 => print!("D"),
//                 _ => print!(" "),
//             }
//         }
//         println!("");
//     }
//     println!("");
// }

fn valid_col_for(c: i64) -> usize {
    ((c*2)+1) as usize
}

fn valid_c_for(x: usize) -> i64 {
    (x as i64 - 1)/2 
}

fn valid_square(board: &Board, x: usize, y: usize) -> bool {
    (board[y][x]*2)+1 == x as i64
}

fn board_clone_replace(board: &Board, oldx: usize, oldy: usize, newx: usize, newy: usize ) -> Board {
    let mut new_board = board.clone();
    new_board[newy][newx] = board[oldy][oldx];
    new_board[oldy][oldx] = 0;
    new_board
}

fn check_empty(board: &Board, x: usize, y: usize) -> bool {
    board[y][x] == 0
}

fn do_move(moves :&mut Distances, board: &Board, oldx: usize, oldy: usize, newx: usize, newy: usize, d: i64) {
    moves.insert(board_clone_replace(board,oldx,oldy,newx,newy), d*cost(board[oldy][oldx]));
}

fn check_and_move_down(moves :&mut Distances, board: &Board, oldx: usize, newx: usize, roomsize: usize) {
    //Assuming oldy is 1.
    let direction : i64 = if newx>oldx {1} else {-1};
    let mut checkx = oldx;
    let mut d = 0;

    //Check corridor is free.
    loop {
        checkx =(checkx as i64 + direction) as usize;
        d+=1;
        if !check_empty(board, checkx, 1) { return; }
        if checkx==newx { break; }
    }

    //Check first spot in room is free.
    let mut lowest = 2;
    d+=1;
    if !check_empty(board, newx, lowest) { return; }

    //Find lowest spot can move into.
    for _ in 0..roomsize-1 {
        if check_empty(board, newx, lowest+1) {lowest+=1; d+=1;}
        else { break; }
    }

    //Check everything below lowest is valid.
    for n in lowest+1..=roomsize+1 {
        if !valid_square(board, newx, n) { return; }
    }

    do_move(moves, board, oldx, 1, newx, lowest, d);
}

fn check_empty_upwards(board: &Board, oldx: usize, oldy: usize) -> bool {
    for newy in 1..oldy {
        if !check_empty(board, oldx, newy) { return false; }
    }
    true
}

fn check_valid_downwards(board: &Board, oldx: usize, oldy: usize, roomsize: usize) -> bool {
    for newy in oldy..=roomsize+1 {
        if !valid_square(board, oldx, newy) { return false; }
    }
    true
}

fn check_and_move_up(moves :&mut Distances, board: &Board, oldx: usize, oldy: usize, newx: usize) {
    //Assuming can move out of room.
    let direction : i64 = if newx>oldx {1} else {-1};
    let mut checkx = oldx;
    let mut d = oldy-1;
    if !check_empty(board, checkx, 1) { return; }

    //Check corridor is free.
    loop {
        checkx =(checkx as i64 + direction) as usize;
        d+=1;
        if !check_empty(board, checkx, 1) { return; }
        if checkx==newx { break; }
    }

    do_move(moves, board, oldx, oldy, newx, 1, d as i64);
}

fn get_moves(board: &Board, roomsize: usize) -> Distances {
    let mut moves = Distances::new();

    for (y,line) in board.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c>0 {

                //In room: If needs moving and can reach corridor
                if y>=2 && !check_valid_downwards(board, x, y, roomsize) && check_empty_upwards(board, x, y) {
                    //Try every corridor position
                    for newx in [1,2,4,6,8,10,11] {
                        check_and_move_up(&mut moves, board, x, y, newx);
                    }
                }

                //Corridor: Try and move into correct room
                if y==1 { check_and_move_down(&mut moves, board, x, valid_col_for(*c), roomsize ); }
            }
        }
    }
    moves
}

fn run(start_board: &Board, win_state: &Board, roomsize: usize) -> i64 {
    let mut distances = Distances::new();
    let mut frontier = Frontier::new();
    frontier.push((0, start_board.clone()));

    while let Some((testing_cost,board)) = frontier.pop() {
        let testing_cost=-testing_cost; //back to +ve
        if boards_eq(&board,win_state) { return testing_cost }

        if distances.contains_key(&board) {
            if testing_cost >= *distances.get(&board).unwrap() { continue; }
        }
        distances.insert(board.clone(),testing_cost);

        for (boardm,additional_cost) in get_moves(&board, roomsize) {
            frontier.push((-(testing_cost+additional_cost), boardm));
        }
    }
    unreachable!();
}


#[aoc(day23, part1)]
fn part1(board: &Board) -> i64 {

    //Construct win state.
    let mut win_state = board.clone();
    for x in [3,5,7,9] {
        for y in 2..=3 {
            win_state[y][x] = valid_c_for(x);
        }
    }
    run(board, &win_state, 2)
}

#[aoc(day23, part2)]
fn part2(b: &Board) -> i64 {

    //Expand board.
    let mut board = b.clone();
    let mut row = Vec::<i64>::new();
    for _ in 0..=12 { row.push(-1); }
    board.push(row.clone());
    board.push(row.clone());
    for x in [3,5,7,9] {
        board=board_clone_replace(&board,x,3,x,5);
    }
    board[3][3] = 4;
    board[3][5] = 3;
    board[3][7] = 2;
    board[3][9] = 1;
    board[4][3] = 4;
    board[4][5] = 2;
    board[4][7] = 1;
    board[4][9] = 3;

    //Construct win state.
    let mut win_state = board.clone();
    for x in [3,5,7,9] {
        for y in 2..=5 {
            win_state[y][x] = valid_c_for(x);
        }
    }

    run(&board, &win_state, 4)
}
