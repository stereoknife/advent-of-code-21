use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-06.txt").unwrap();
	
	let nums : Vec<usize> = contents.split(',').map(|v| v.parse().unwrap()).collect();
	
	// fish won't be waiting for more than 8 days (len 9)
	let mut fishes = vec![0u64; 9];  // wowie rly making me use u64 here
	
	for fish in nums {
		fishes[fish] += 1;
	}
	
	for _i in 0..256 {
		let mut next = vec![0; 9];
		let mut fiter = fishes.iter().enumerate();
		let (_, &reset) = fiter.next().unwrap();
		
		for (i, fish) in fiter {
			next[i-1] = *fish;
		}
		
		println!("{}", next[6]);
		
		next[6] += reset; // move fish at 0 back to 6
		next[8] = reset; // and add children
		
		fishes = next;
	}
	
	println!("{}", fishes.iter().sum::<u64>());
	
}