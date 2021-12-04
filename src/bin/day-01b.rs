use std::fs;

fn main() {
	let contents = fs::read_to_string("./src/inputs/input-01.txt").unwrap();
				
	let mut inc = 0;
	let mut prev = -1;
	
	let mut lines = contents.lines().map(|l| l.parse::<i32>().unwrap());
		
	while let Some(_) = lines.next() {
		let sum: i32 = lines.clone().take(3).sum();
		if sum > prev && prev > -1 { inc += 1 ; }
		prev = sum;
	}
	
	println!("{}", inc);	
}