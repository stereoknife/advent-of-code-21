use std::fs;
use regex::Regex;

fn main() {
	let contents = fs::read_to_string("./inputs/input-02.txt").unwrap();
	
	let re = Regex::new(r"(?m)^(forward|up|down) (\d+)$").unwrap();
	
	let mut x = 0;
	let mut d = 0;
	
	for cap in re.captures_iter(&contents) {
		
		let val : i32 = cap[2].parse().unwrap();
		match &cap[1] {
			"forward" => x += val,
			"up" => d -= val,
			"down" => d += val,
			_ => {}
		}
	}
	
	println!("{}", x * d)
}