use hashbrown::HashMap;

type Board = HashMap<(u8, u8), u8>;
type Input = (Board, u8, u8);

#[aoc_generator(day25)]
pub fn parse(input: &str) -> Input {
    let mut board = Board::new();
    let mut xmax = 0;
    let mut ymax = 0;
    for (y,line) in input.lines().enumerate() {
        for (x,c) in line.bytes().enumerate() {
            match c {
                b'>' => {board.insert((x as u8,y as u8),1);},
                b'v' => {board.insert((x as u8,y as u8),2);},
                _ => {},
            }
            xmax = x;
        }
        ymax = y;
    }
    (board, xmax as u8, ymax as u8)
}

// fn print_board(board: &Board, xmax :u8, ymax :u8) {
//     for y in 0..=ymax {
//         for x in 0..=xmax {
//             if board.contains_key(&(x,y)) {
//                 print!("{}", board.get(&(x,y)).unwrap());
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
//     println!("");
// }

fn run(board: &Board, xmax :u8, ymax :u8, cont : &mut bool) -> Board {
    let mut rboard = Board::new();
    *cont = false;

    for ((x,y),d) in board {
        if *d == 1 {
            let mut nx = x+1;
            if nx > xmax { nx = 0; }
            if !board.contains_key(&(nx,*y)) { rboard.insert((nx,*y),1); *cont=true; continue; }
        }
        rboard.insert((*x,*y),*d); 
    }

    let mut dboard = Board::new();
    for ((x,y),d) in &rboard {
        if *d == 2 {
            let mut ny = y+1;
            if ny > ymax { ny = 0; }
            if !rboard.contains_key(&(*x,ny)) { dboard.insert((*x,ny),2); *cont=true; continue; }
        }
        dboard.insert((*x,*y),*d); 
    }

    dboard
}

#[aoc(day25, part1)]
fn part1(i: &Input) -> usize {
    let (b,xmax,ymax) = i;
    let mut board = b.clone();
    let mut count = 0;
    let mut cont = true;
    while cont { board = run(&board, *xmax, *ymax, &mut cont); count+=1; }
    count
}