
type Displays<'a> = Vec::<(Vec::<&'a str>, Vec::<&'a str>)>;

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Displays {
	input.lines().map(|line| {
		    let mut splitter = line.splitn(2, '|');
		    let l : Vec::<&str> = splitter.next().unwrap().split(" ").map(|l| l.parse().unwrap()).collect();
		    let r : Vec::<&str> = splitter.next().unwrap().split(" ").map(|l| l.parse().unwrap()).collect();
			(l,r)
        }).collect()
}

#[aoc(day8, part1)]
fn part1(d: &Displays<'a>) -> usize {
  
  println!("{:?}", d);

  0
}