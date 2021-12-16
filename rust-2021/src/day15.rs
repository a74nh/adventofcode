use std::collections::HashMap;

type Risks = HashMap<(i64, i64), i64>;
type Input =  (Risks, i64, i64);
type Distances = HashMap<(i64, i64), i64>;
type Frontier = HashMap<(i64, i64), i64>;

#[aoc_generator(day15)]
fn parse(input: &str) -> Input {
    let mut risks = Risks::new();
    let mut xmax : i64 = 0;
    let mut ymax : i64 = 0;

    for (y,l) in input.lines().enumerate() {
        for (x,risk) in l.bytes().enumerate() {
            risks.insert((x as i64,y as i64),(risk-b'0') as i64);
            xmax = x as i64;
        }
        ymax = y as i64;
    }
    (risks, xmax+1, ymax+1)
}

fn get_min(frontier: &Frontier) -> (i64,i64) {
    let ((xpos,ypos),_) = frontier.iter().min_by_key(|entry | entry.1).unwrap();
    (*xpos,*ypos)
}

fn get_risk(multiplex: i64, risks: &Risks, x: i64, y: i64, xmax: i64, ymax: i64) -> i64 {
    if x<0 || y<0 || x>=xmax*multiplex || y>=ymax*multiplex { return i64::MAX; }
    if ! risks.contains_key(&(x%xmax,y%ymax)) { return i64::MAX; }
    ((*risks.get(&(x%xmax,y%xmax)).unwrap() + (x/xmax) + (y/ymax) - 1) % 9 ) + 1
}

fn heuristic(ax: i64, ay: i64, bx: i64, by: i64) -> i64 {
   i64::abs(ax - bx) + i64::abs(ay - by)
}

fn visit_next(multiplex: i64, risks: &Risks, distances: &mut Distances, frontier: &mut Frontier, xmax: i64, ymax: i64) -> bool {
  if frontier.len() == 0 { return false }
  let (xpos,ypos) = get_min(frontier);
  if xpos==(xmax*multiplex)-1 && ypos==(ymax*multiplex)-1 { return false }

  frontier.remove(&(xpos, ypos));
  let risk_current = *distances.get(&(xpos,ypos)).unwrap();

  for (nx,ny) in [(xpos-1,ypos),(xpos+1,ypos),(xpos,ypos-1),(xpos,ypos+1)] {
    let mut new_risk = get_risk(multiplex, risks, nx, ny, xmax, ymax);
    if new_risk != i64::MAX {
        new_risk+=risk_current;
        if ! distances.contains_key(&(nx,ny)) {
            distances.insert((nx,ny), new_risk);
            frontier.insert((nx,ny), new_risk + heuristic(xmax, ymax, nx, ny));
        } else if new_risk < *distances.get_mut(&(nx,ny)).unwrap() {
            *distances.get_mut(&(nx,ny)).unwrap() = new_risk;
            frontier.insert((nx,ny), new_risk + heuristic(xmax, ymax, nx, ny));
        }
    }
  }
  true
}

fn run(i: &Input, multiplex: i64) -> i64 {
    let (risks, xmax, ymax) = i;
    let mut distances = Distances::new();
    let mut frontier = Frontier::new();
    distances.insert((0,0), 0);
    frontier.insert((0, 0), 0);
    while visit_next(multiplex, risks, &mut distances, &mut frontier, *xmax, *ymax) {}
    *distances.get(&((*xmax*multiplex)-1,(*ymax*multiplex)-1)).unwrap()
}

#[aoc(day15, part1)]
fn part1(i: &Input) -> i64 {
    run(i, 1)
}

#[aoc(day15, part2)]
fn part2(i: &Input) -> i64 {
    run(i, 5)
}

