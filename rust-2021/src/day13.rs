use itertools::Itertools;
use std::collections::BTreeSet;

type Coords = BTreeSet<(i64, i64)>;
type Instr = (String, i64);
type Instrs = Vec<Instr> ;
type Input = (Coords, Instrs);
struct CoordsDisplay(Coords);

#[aoc_generator(day13)]
fn parse(input: &str) -> Input {
    let (c, i) = input.split("\n\n").next_tuple().unwrap();
    ( c.lines().map(|line| {
            let mut i = line.split(",");
            (i.next().unwrap().parse().unwrap(), i.next().unwrap().parse().unwrap(),
            )}).collect(),
      i.lines().map(|line| {
            let mut i = line.trim_start_matches("fold along ").split("=");
            (i.next().unwrap().to_string(), i.next().unwrap().parse().unwrap(),
            )}).collect())
}

fn fold(coords: &Coords, instr: &Instr) -> Coords {
  let (dir,pos) = instr;
  if dir == "x" {
    coords.iter().map(|(x,y)| (pos - (pos - x).abs(), *y)).collect()
  } else {
    coords.iter().map(|(x,y)| (*x, pos - (pos - y).abs())).collect()
  }
}

impl std::fmt::Debug for CoordsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = Vec::from_iter(&self.0);
        v.sort_by(|&(ax, ay), &(bx, by)| (ay,ax).cmp(&(by, bx)));
        let mut x = 0;
        let mut y = -1;
        for coord in &v {
        let (cx,cy) = coord;
            while y<*cy { write!(f,"\n")?; y+=1; x=0; }
            while x<*cx { write!(f," ")?; x+=1; }
            write!(f,"#")?;
            x+=1;
        }
        write!(f, "")
    }
}

impl std::fmt::Display for CoordsDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

#[aoc(day13, part1)]
fn part1(i: &Input) -> usize {
    let (c,instrs) = i;
    fold(c, instrs.first().unwrap()).iter().unique().count()
}

#[aoc(day13, part2)]
fn part2(i: &Input) -> CoordsDisplay {
    let (c,instrs) = i;
    let mut coords = c.clone();
    for instr in instrs {
        coords = fold(&coords, instr);
    }
    CoordsDisplay(coords)
}
