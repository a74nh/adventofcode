use itertools::Itertools;

type Input = Vec<u8>;
type Values = Vec<usize>;

#[aoc_generator(day16)]
fn parse(input: &str) -> Input {
    input.chars().map(|c| format!("{:04b}", c.to_digit(16).unwrap())).join("").chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

#[derive(Debug)]
struct Packets<'a> {
    input: &'a Input,
    pos: usize,
    vtotals: usize,
}
impl Packets<'_> {
	fn get_num(&mut self, size: usize) -> usize {
		let r = self.input[self.pos..self.pos+size].iter().fold(0usize, |acc, bit| (acc << 1) | *bit as usize);
		self.pos += size;
		r
	}
}

fn parse_literal(p: & mut Packets) -> usize {
	let mut val = 0;
	loop {
		let cont = p.get_num(1);
		val = (val << 4) + p.get_num(4);
		if cont == 0 { return val; }
	}
}

fn parse_operator(p: & mut Packets) -> Values {
	let mut values = Values::new();
	match p.get_num(1) {
		0 => { let tot = p.get_num(15) + p.pos;
			   while p.pos < tot { values.push(parse_packet(p)); } },
		1 => for _ in 0..p.get_num(11) { values.push(parse_packet(p)); },
        _ => unreachable!()
	}
	values
}

fn parse_packet(p: & mut Packets) -> usize {
	let ver = p.get_num(3);
	let id = p.get_num(3);
	p.vtotals+=ver;
	match id {
		0 => { parse_operator(p).iter().sum() },
		1 => { parse_operator(p).iter().product() },
		2 => { *parse_operator(p).iter().min().unwrap() },
		3 => { *parse_operator(p).iter().max().unwrap() },
		4 => { parse_literal(p) },
		5 => { let r=parse_operator(p); (r[0]>r[1]) as usize },
		6 => { let r=parse_operator(p); (r[0]<r[1]) as usize },
		7 => { let r=parse_operator(p); (r[0]==r[1]) as usize },
        _ => unreachable!()
	}
}

#[aoc(day16, part1)]
fn part1(i: &Input) -> usize {
	let mut packets = Packets { input: i, pos: 0, vtotals: 0 };
    parse_packet(& mut packets);
    packets.vtotals
}

#[aoc(day16, part2)]
fn part2(i: &Input) -> usize {
	let mut packets = Packets { input: i, pos: 0, vtotals: 0 };
    parse_packet(& mut packets)
}
