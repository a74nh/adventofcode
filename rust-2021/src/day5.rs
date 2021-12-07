use regex::Regex;
use std::collections::HashMap;

type Vents = Vec::<(i64, i64, i64, i64)>;
type Board = HashMap::<(i64, i64), i64>;

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Vents {
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    input.lines().map(|line| {
            let cap = re.captures(line).unwrap();
            let x1 = cap[1].parse::<i64>().unwrap();
            let y1 = cap[2].parse::<i64>().unwrap();
            let x2 = cap[3].parse::<i64>().unwrap();
            let y2 = cap[4].parse::<i64>().unwrap();
            (x1, y1, x2, y2)
        }).collect()
}

fn do_board(vents: Vents)-> usize {
    // Produce board
    let mut board = Board::new();
    for (x1, y1, x2, y2) in vents {

        let x_dir = if x1 == x2 { 0 } else if x1 > x2 { -1 } else { 1 };
        let y_dir = if y1 == y2 { 0 } else if y1 > y2 { -1 } else { 1 };
        let mut x = x1;
        let mut y = y1;
        let range = if x1 == x2 { if y1 > y2 {y2..y1+1} else {y1..y2+1} } else { if x1 > x2 {x2..x1+1} else {x1..x2+1} };

        for _c in range {
            *board.entry((x,y)).or_insert(0) += 1;
            x += x_dir;
            y += y_dir;
        }
    }
    // Count all points with multiple lines
    board.into_iter().filter(|(_k, v)| v > &1).collect::<Board>().values().count()
}

#[aoc(day5, part1)]
fn part1(v: &Vents) -> usize {
    let mut vents = v.clone();
    vents.retain(|(x1, y1, x2, y2)| x1==x2 || y1==y2);
    do_board(vents)
}

#[aoc(day5, part2)]
fn part2(v: &Vents) -> usize {
    let vents = v.clone();
    do_board(vents)
}
