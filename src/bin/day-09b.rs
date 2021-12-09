use std::fs;
use std::collections::HashSet;

fn main() {
	let contents = fs::read_to_string("./inputs/input-09.txt").unwrap();
	
	let values : Vec<&str> = contents.lines().collect();
	let values : Vec<Vec<usize>> = values.iter().map(|s| s.chars().map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap()).collect()).collect();
	
	let mut lows : Vec<(usize, usize)> = Vec::new();
	let mut basins : Vec<usize> = Vec::new();
	
	let ymax = values.len() - 1;
	let xmax = values[0].len() - 1;
	
	for (y, value) in values.iter().enumerate() {
		for (x, num) in value.iter().enumerate() {
			if x > 0 && num >= &value[x-1] { continue; }
			if x < xmax && num >= &value[x+1] { continue; }
			if y > 0 && num >= &values[y-1][x] { continue; }
			if y < ymax && num >= &values[y+1][x] { continue; }
			lows.push((x,y));
		}	
	}
	
	for low in lows {
		let mut queue : Vec<(usize, usize)> = Vec::new();
		let mut visited : HashSet<(usize, usize)> = HashSet::new();
		queue.push(low);
		
		while !queue.is_empty() {
    		let (x, y) = queue.pop().unwrap();
			if values[y][x] == 9 { continue }
			if visited.contains(&(x, y)) { continue; }
			visited.insert((x,y));
			if x > 0 { queue.push((x-1, y)) }
			if x < xmax { queue.push((x+1, y)) }
			if y > 0 { queue.push((x, y-1)) }
			if y < ymax { queue.push((x, y+1)) }
		}
		 basins.push(visited.len());
	}
	
	basins.sort_unstable();
	let basins : Vec<usize> = basins.into_iter().rev().collect();
	
	println!("{:?}", basins);
	println!("{}", basins[0] * basins[1] * basins[2]);
}