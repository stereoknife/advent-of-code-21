use std::fs;
use std::collections::HashMap;

fn main() {
	let contents = fs::read_to_string("./inputs/input-14.txt").unwrap();
	let mut lines = contents.lines();

	let polymer = lines.next().unwrap();
	
	// skip empty line
	let lines = lines.skip(1);
	
	let mut instructions: HashMap<&str, (String, String)> = HashMap::new();
	
	for line in lines {
		let mut vals = line.split(" -> ");
		let pair = vals.next().unwrap();
		let mut chars = pair.chars();
		let ch1 = chars.next().unwrap();
		let ch2 = chars.next().unwrap();
		let out = vals.next().unwrap().chars().next().unwrap();
		let mut s1 = String::new();
		let mut s2 = String::new();
		s1.push(ch1);
		s1.push(out);
		s2.push(out);
		s2.push(ch2);
		instructions.insert(pair, (s1, s2));
	}
	
	let mut pairs: HashMap<&str, u64> = HashMap::new();
	
	for pair in instructions.keys() {
		pairs.insert(pair, 0);
	}
	
	for i in 0..polymer.len() - 1 {
		let key = &polymer[i..i+2];
		pairs.entry(key).and_modify(|v| *v += 1);
	}
	
	//println!("{:?}", pairs);
	
	for _ in 0..40 {
		let mut new_pairs = pairs.clone();
		for (_, val) in new_pairs.iter_mut() {
			*val = 0;
		}
		
		for (pair, count) in &pairs {
			let (outcome1, outcome2) = &instructions[pair];
			new_pairs.entry(&outcome1).and_modify(|v| *v += count);
			new_pairs.entry(&outcome2).and_modify(|v| *v += count);
		}
		
		for (key, val) in pairs.iter_mut() {
			*val = new_pairs[key];
		}	
	}
	
	let mut counts: HashMap<char, u64> = HashMap::new();
	
	for (pair, count) in pairs {
		let mut chars = pair.chars();
		let ch1 = chars.next().unwrap();
		let ch2 = chars.next().unwrap();
		
		let entry1 = counts.entry(ch1).or_default();
		*entry1 += count;
		
		let entry2 = counts.entry(ch2).or_default();
		*entry2 += count;
	}
	
	// B and K are the beginnin and end letters so they will be missing 1 duplication
	counts.entry('B').and_modify(|v| *v += 1);
	counts.entry('K').and_modify(|v| *v += 1);
	
	
	//println!("{:?}", pairs);
	
	let max = counts.values().max().unwrap();
	let min = counts.values().min().unwrap();
	
	println!("{}", (max-min) / 2);

}