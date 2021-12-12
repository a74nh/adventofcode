use std::collections::HashMap;

type Input = Vec<(String, String)> ;
type Graph = HashMap::<String, Vec<String>>;
type Counts = HashMap::<String, i64>;

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Input {
	input.lines().map(|line| {
			let mut i = line.split("-");
            (i.next().unwrap().to_string(), i.next().unwrap().to_string(),
            )}).collect()
}

fn build_graph(input: &Input) -> Graph {
  let mut graph = Graph::new();

  for (l,r) in input {
  	if ! graph.contains_key(l) {
  		graph.insert(l.to_string(), Vec::<String>::new());
  	}
  	if ! graph.contains_key(r) {
  		graph.insert(r.to_string(), Vec::<String>::new());
  	}
  	graph.get_mut(l).unwrap().push(r.to_string());
  	graph.get_mut(r).unwrap().push(l.to_string());
  }
  graph
}

fn empty_counts(graph: &Graph) -> Counts {
	let mut counts = Counts::new();
	for (g,_) in graph {
		counts.insert(g.to_string(),0);
	}
	counts
}

fn visit(graph: &Graph, current: &String, counts: & mut Counts, double: & mut String) -> usize {
  if current == "end" { return 1; }
  let old_double = double.clone();
  let mut restore_double = false;
  if current.chars().next().unwrap().is_lowercase() {
  	if counts.get(current).unwrap() >= &2 { return 0; }
  	if counts.get(current).unwrap() == &1 {
  		if current == "start" { return 0; }
  		else if double.len() == 0 { *double = current.clone(); restore_double = true; }
  		else if double != current { return 0; }
  	}
  }

  *counts.get_mut(current).unwrap() += 1;
  
  let mut tot = 0;
  for next in graph.get(current).unwrap() {
  	tot += visit(graph, next, counts, double);
  }

  *counts.get_mut(current).unwrap() -= 1;
  if restore_double { *double = old_double.clone(); }
  tot
}

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
	let graph = build_graph(input);
	visit(&graph, &"start".to_string(), &mut empty_counts(&graph), &mut "NONE".to_string())
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
	let graph = build_graph(input);
	visit(&graph, &"start".to_string(), &mut empty_counts(&graph), &mut "".to_string())
}
