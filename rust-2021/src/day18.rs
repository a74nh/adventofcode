use std::ops::Add;
use std::cmp;

#[derive(Clone, Debug, Copy)]
enum Token {
    BracketL,
    BracketR,
    Comma,
    Num(usize),
}

impl Add for Token {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        num_to_token(token_to_num(self) + token_to_num(other))
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Token::BracketL => write!(f, "["),
			Token::BracketR => write!(f, "]"),
			Token::Comma => write!(f, ","),
			Token::Num(i) => write!(f, "{}", i),
		}
	}
}

type Nodes = Vec<Token>;
type NodesList = Vec<Nodes>;

// fn printnode(nodes : &Nodes) {
// 	for n in nodes {
// 		print!("{}", n);
// 	}
// 	println!("");
// }


#[aoc_generator(day18)]
fn parse(input: &str) -> NodesList {
	let mut nodeslist = NodesList::new();
	for l in input.lines() {
    	nodeslist.push(l.chars().map(|c| match c {
								                '0'..='9' => Token::Num(c.to_digit(10).unwrap() as usize),
								                '[' => Token::BracketL,
								                ']' => Token::BracketR,
								                ',' => Token::Comma,
								                _ => unreachable!(),
											 }).collect());
	}
	nodeslist
}

fn token_to_num(t: Token) -> usize {
	match t {
		Token::Num(i) => i,
		_ => unreachable!(),
	}
}

fn num_to_token(i: usize) -> Token {
	Token::Num(i)
}

fn do_add(left : &Nodes, right : &Nodes) -> Nodes {
	let mut res = Vec::new();
	res.push(Token::BracketL);
	res.extend(left.iter().cloned());
	res.push(Token::Comma);
	res.extend(right.iter().cloned());
	res.push(Token::BracketR);
	res
}

fn explode(nodes : &mut Nodes) -> bool {
	let mut nest = 0;
	let mut last_found_num_index = 0;
	let mut replace = 0;
	for (index,n) in nodes.iter().enumerate() {
		match n {
			Token::BracketL => { nest+=1; },
			Token::BracketR => { nest-=1; },
			Token::Comma => { },
			Token::Num(_) => { last_found_num_index=index; },
		}
		if nest==5 {replace=index; break;}
	}
	if nest == 5 {
		let mut res = Vec::new();
		res.extend(nodes[0..replace].iter().cloned());
		if last_found_num_index>0 {
			res[last_found_num_index] = res[last_found_num_index] + nodes[replace+1];
		}
		res.push(Token::Num(0));
		let mut found = false;
		res.extend(nodes[replace+5..].iter().cloned().map(|n| {
			match n {
				Token::Num(_) => { if !found {found=true; n+nodes[replace+3]} else {n} },
				_ => { n },
			}
		}));
		*nodes = res;
		return true;
	}
	false
}

fn split(nodes : &mut Nodes) -> bool {
	let mut found_num_index = 0;
	let mut found_num = 0;

	for (index,n) in nodes.iter().enumerate() {
		match n {
			Token::BracketL => { },
			Token::BracketR => { },
			Token::Comma => { }
			Token::Num(i) => { if *i>9 { found_num_index=index; found_num=*i; break; } },
		}
	}
	if found_num_index > 0 {
		let mut res = Vec::new();
		res.extend(nodes[0..found_num_index].iter().cloned());
		res.push(Token::BracketL);
		res.push(Token::Num(found_num/2));
		res.push(Token::Comma);
		res.push(Token::Num(found_num-(found_num/2)));
		res.push(Token::BracketR);
		res.extend(nodes[found_num_index+1..].iter().cloned());
		*nodes = res;
		return true;
	}
	false
}

fn magnitude(nodes : &Nodes) -> usize {
    let mut ops = vec![];
    let mut nums = vec![];

    for n in nodes.iter() {
        match n {
            Token::BracketL | Token::Comma => { ops.push(n); }
            Token::Num(i) => { nums.push(*i);}
            Token::BracketR => loop {
                match ops.pop() {
                    None => unreachable!(),
                    Some(Token::Comma) => {
                        let rval = nums.pop().unwrap();
                        let lval = nums.pop().unwrap();
                        nums.push(3 * lval + 2 * rval);
                    }
                    Some(Token::BracketL) => { break; }
                    Some(_) => unreachable!(),
                }
            }
        }
    }
    nums[0]
}

fn do_next(left : &Nodes, right : &Nodes) -> Nodes {
    let mut result = do_add(left, right);
    while explode(&mut result) || split(&mut result) {}
    result
}

#[aoc(day18, part1)]
fn part1(input: &NodesList) -> usize {
    let mut result = input[0].clone();
    for n in &input[1..] {
    	result = do_next(&result, n);
	}
	magnitude(&result)
}

#[aoc(day18, part2)]
fn part2(input: &NodesList) -> usize {
	let mut res = 0;
    for ln in 0..input.len() {
    	for lr in ln+1..input.len() {
    		res = cmp::max(res, magnitude(&do_next(&input[ln], &input[lr])));
    	}
    }
    res
}
