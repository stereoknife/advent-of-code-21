use std::fs;
use std::collections::HashMap;

fn main() {
	let contents = fs::read_to_string("./inputs/input-14.txt").unwrap();
	let mut lines = contents.lines();

	let polymer = lines.next().unwrap();
	
	// skip empty line
	let lines = lines.skip(1);
	
	let mut instructions: HashMap<&str, char> = HashMap::new();
	
	for line in lines {
		let mut vals = line.split(" -> ");
		instructions.insert(vals.next().unwrap(), vals.next().unwrap().chars().next().unwrap());
	}
	
	let mut polymer = String::from(polymer);
	for _ in 0..10 {
    	let polymer_chars: Vec<char> = polymer.chars().collect();
		let mut new_polymer = String::new();
		for p in 0..polymer.len() - 1 {
			let pair = &polymer[p..=p+1];
			new_polymer.push(polymer_chars[p]);
			new_polymer.push(instructions[pair]);
		}
		new_polymer.push(*polymer_chars.last().unwrap());
		polymer = new_polymer;
	}
	
	let mut counts: HashMap<char, u32> = HashMap::new();
	
	for ch in polymer.chars() {
		let c = counts.entry(ch).or_insert(0);
		*c += 1;
	}
	
	let max = counts.values().max().unwrap();
	let min = counts.values().min().unwrap();
	
	println!("{}", max-min);

}