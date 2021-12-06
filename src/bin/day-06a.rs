use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-06.txt").unwrap();
	
	let mut fishes : Vec<i32> = contents.split(',').map(|v| v.parse().unwrap()).collect();
	let mut new_fish = 0;
	
	// brute force
	for _ in 0..80u32 {
		for fish in fishes.iter_mut() {
			*fish -= 1;
			if *fish == -1 {
				*fish = 6;
				new_fish += 1;
			}
		}
		fishes.append(&mut vec![8; new_fish]);
		new_fish = 0;
	}
	
	println!("{}", fishes.len());
}