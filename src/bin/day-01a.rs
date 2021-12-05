use std::fs;

fn main() {
	// No error handling but it's ok
	let contents = fs::read_to_string("./inputs/input-01.txt").unwrap();
			
	let mut inc = 0;
	let mut prev = -1;
	
	for line in contents.lines() {
		let val = line.parse::<i32>().unwrap();
		if val > prev && prev > -1 { inc += 1 };
		prev = val;
	}
	
	println!("{}", inc);	
}