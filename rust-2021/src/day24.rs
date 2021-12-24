use hashbrown::HashMap;

type Op = Vec<String>;
type Input = Vec<Op>;
type State = [i64; 4];
type Cache = HashMap<(State, usize), Option<i64>>;

#[aoc_generator(day24)]
fn parse(input: &str) -> Input {
   input.lines().map(|l| l.split_whitespace().map(|s| s.to_string()).collect()).collect()
}

fn get_val(var: &String, state: &State) -> i64 {
	if let Ok(val) = var.parse() {
		return val;
	}
	state[(var.bytes().next().unwrap()-b'w') as usize]
}

fn set_state(var: &String, val :i64, state: &mut State) {
	state[(var.bytes().next().unwrap()-b'w') as usize] = val;
}

fn do_inp(m :i64, op : &Op, state: &mut State) {
	if op[0] != "inp" { unreachable!(); }
	set_state(&op[1], m, state);
}

fn do_op(op : &Op, state: &mut State) -> bool {
	match op[0].as_ref() {
		"inp" => return false,
		"add" => set_state(&op[1], get_val(&op[1], state) + get_val(&op[2], state), state),
		"mul" => set_state(&op[1], get_val(&op[1], state) * get_val(&op[2], state), state),
		"div" => set_state(&op[1], get_val(&op[1], state) / get_val(&op[2], state), state),
		"mod" => set_state(&op[1], get_val(&op[1], state) % get_val(&op[2], state), state),
		"eql" => set_state(&op[1], if get_val(&op[1], state) == get_val(&op[2] , state) {1} else {0}, state),
		_ => unreachable!(),
	}
	true
}

fn run(start_pc :usize, input: &Input, start_state :State, cache :&mut Cache, iter: &[i64; 9]) -> Option<i64> {

    if let Some(c) = cache.get(&(start_state, start_pc)) {
        return *c;
    }

	'outer: for m in iter {
		let mut state = start_state.clone();
		let mut pc = start_pc;

		do_inp(*m, &input[pc], &mut state);
		pc+=1;

		while do_op(&input[pc], &mut state) {
			pc+=1;
			if pc>=input.len() {
				// println!("end {:?}", (m,get_val(&"z".to_string(), &state)) );
				if get_val(&"z".to_string(), &state) == 0 {
					cache.insert((state.clone(),pc), Some(*m));
					println!("found {:?}", m);
					return Some(*m)
				}
				continue 'outer;
			}
		}

		if let Some(val) = run(pc, input, state, cache, iter) {
			let r = Some(val * 10 + m);
			cache.insert((state.clone(),pc), r);
			return r;
		}
	}

	cache.insert((start_state,start_pc), None);
	None
}

#[aoc(day24, part1)]
fn part1(input: &Input) -> i64 {
	run(0, &input, [0; 4], &mut Cache::new(), &[9,8,7,6,5,4,3,2,1]).unwrap().to_string().chars().rev().collect::<String>().parse().unwrap()
}


#[aoc(day24, part2)]
fn part2(input: &Input) -> i64 {
	run(0, &input, [0; 4], &mut Cache::new(), &[1,2,3,4,5,6,7,8,9]).unwrap().to_string().chars().rev().collect::<String>().parse().unwrap()
}