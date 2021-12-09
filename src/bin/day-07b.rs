use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-07.txt").unwrap();
	
	let crabs : Vec<usize> = contents.split(',').map(|v| v.parse().unwrap()).collect();
	
	let maxd = *crabs.iter().max().unwrap();
	let mut results = vec![0; maxd+1];
	
	println!("{}", maxd);
	
	for i in 0..=maxd {
		let res = crabs.iter().fold(0, |acc, &v| acc + diff(v, i));
		
		println!("{}: {}", i, res);
		
		results[i] = res;
	}
	
	println!("{}", results.iter().min().unwrap());
}

// Brute forced here but oh well
fn diff (a: usize, b: usize) -> usize {
	let r = if a > b {
		a - b
	} else {
		b - a
	};
	(0..=r).sum()
}