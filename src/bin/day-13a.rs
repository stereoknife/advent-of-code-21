use std::fs;
use std::collections::HashSet;

fn main() {
	let contents = fs::read_to_string("./inputs/input-13.txt").unwrap();
	
	let coords: Vec<&str> = contents.lines().take_while(|l| !l.is_empty()).collect();
	let mut coords: HashSet<(usize, usize)> = coords.iter().map(|s| {
		let mut vals = s.split(',').map(|v| v.parse().unwrap());
		(vals.next().unwrap(), vals.next().unwrap())
	}).collect();
	
	let instructions = contents.lines().skip_while(|l| !l.is_empty()).skip(1); // Additional skip to ignore empty line
	let mut instructions = instructions.map(|l| {
		let mut vals = l.trim_start_matches("fold along ").split("=");
		let axis = if vals.next().unwrap() == "x" { 0 } else { 1 };
		(axis, vals.next().unwrap().parse::<usize>().unwrap())
	});
	
	let (axis, value) = instructions.next().unwrap();
	
	for (x, y) in coords.clone() {
		if axis == 0 && x > value {
			coords.remove(&(x, y));
			coords.insert((value-(x-value), y));
		} else if axis == 1 && y > value {
			coords.remove(&(x, y));
			coords.insert((x, value-(y-value)));
		}
	}
	
	println!("{}", coords.len());
}