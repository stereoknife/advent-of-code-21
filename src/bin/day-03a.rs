use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-03.txt").expect("No input file provided");
	
	let lines = contents.lines();
	let (input, len) = {
		let mut v : Vec<Vec<u32>> = Vec::new();
		let mut ct = 0;
		for line in lines {
			if v.capacity() < line.len() { v.resize(line.len(), Vec::new()); }
			
			for (i, c) in line.chars().enumerate() {
				v[i].push(c.to_digit(2).unwrap());
			}
			ct += 1;
		}
		(v, ct)
	};
	
	
	let (gamma, epsilon) = {
		let mut g = Vec::new();
		let mut e = Vec::new();
		for w in input {
			let sum : u32 = w.iter().sum();
			g.push(if sum > len - sum { 1 } else { 0 });
			e.push(if sum > len - sum { 0 } else { 1 });
		}
		(digits_to_numbers(2, &g), digits_to_numbers(2, &e))
	};
	
	println!("{:?}", gamma * epsilon);
}

fn digits_to_numbers(base : u32, digits : &Vec<u32>) -> u32 {
	let mut out = 0;
	let mut mul = 1;
	
	for digit in digits.iter().rev() {
    	out = out + digit * mul;
		mul *= base;
	}
	
	out
}