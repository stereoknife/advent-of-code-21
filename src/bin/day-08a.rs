use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-08.txt").unwrap();
	
	let values : Vec<&str> = contents.lines().collect();
	let values : Vec<&str> = values.iter().map(|s| s.split("|").last().unwrap().trim()).collect();
	let values : Vec<&str> = values.iter().map(|s| s.split(" ").collect::<Vec<&str>>()).flatten().collect();
	
	let count: u32 = values.iter().fold(0, |acc, s| if is_unique(s) { acc+1 } else { acc } );
	
	println!("{}", count);
}

fn is_unique(s: &str) -> bool {
	s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7
}