use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-09.txt").unwrap();
	
	let values : Vec<&str> = contents.lines().collect();
	let values : Vec<Vec<usize>> = values.iter().map(|s| s.chars().map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap()).collect()).collect();
	
	let mut lows : Vec<usize> = Vec::new();
	
	for (y, value) in values.iter().enumerate() {
		for (x, num) in value.iter().enumerate() {
			if x > 0 && num >= &value[x-1] { continue; }
			if x < value.len() - 1 && num >= &value[x+1] { continue; }
			if y > 0 && num >= &values[y-1][x] { continue; }
			if y < values.len() - 1 && num >= &values[y+1][x] { continue; }
			lows.push(*num);
		}	
	}
	
	println!("{}", lows.len() + lows.iter().sum::<usize>());
	
}