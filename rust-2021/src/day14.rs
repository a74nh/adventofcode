use std::collections::HashMap;
use itertools::Itertools;

type Polymer = HashMap<usize, u8>;
type Instrs = HashMap<(u8, u8), u8>;
type Input = (Polymer, Instrs);
type State = HashMap<(u8, u8), usize>;
type Counts = HashMap<u8, usize>;

#[aoc_generator(day14)]
fn parse(input: &str) -> Input {
    let (p, i) = input.split("\n\n").next_tuple().unwrap();
    let mut polymer = Polymer::new();
    for (index, c) in p.bytes().enumerate() { polymer.insert(index,c); };
    let mut instrs = Instrs::new();
    for line in i.lines() {
      let mut i = line.split(" -> ");
      let tl = i.next().unwrap().to_string();
      let mut l = tl.bytes();
      let tr = i.next().unwrap().to_string();
      let mut r = tr.bytes();
      instrs.insert((l.next().unwrap(), l.next().unwrap()),r.next().unwrap());
    }
    (polymer,instrs)
}

fn build_state(polymer: &Polymer) -> (State,u8) {
  let mut state = State::new();
  for index in 0..polymer.keys().len()-1 {
    let pl = *polymer.get(&index).unwrap();
    let pr = *polymer.get(&(index+1)).unwrap();
    *state.entry((pl,pr)).or_insert(0)+=1;
  }
  (state, *polymer.get(&(polymer.keys().len()-1)).unwrap())
}

fn iterate_state(state: &State, instrs: &Instrs) -> State {
  let mut new_state = State::new();
  for ((sl,sr),old_count) in state {
    let new_entry = *instrs.get(&(*sl,*sr)).unwrap();
    *new_state.entry((*sl,new_entry)).or_insert(0)+=old_count;
    *new_state.entry((new_entry,*sr)).or_insert(0)+=old_count;
  }
  new_state
}

fn get_counts(state: &State, last: u8) -> Counts {
  let mut counts = Counts::new();
  for ((sl,_),count) in state {
    *counts.entry(*sl).or_insert(0)+=count;
  }
  *counts.entry(last).or_insert(0)+=1;
  counts
}

fn do_poly(i: &Input, steps: usize) -> usize {
  let (polymer, instrs) = i;
  let (mut state, last) = build_state(polymer);
  for _ in 0..steps { state = iterate_state(&state, instrs); }
  get_counts(&state, last).values().minmax().into_option().map(|(min, max)| max - min).unwrap()
}

#[aoc(day14, part1)]
fn part1(i: &Input) -> usize {
  do_poly(i,10)
}

#[aoc(day14, part2)]
fn part2(i: &Input) -> usize {
  do_poly(i,40)
}
