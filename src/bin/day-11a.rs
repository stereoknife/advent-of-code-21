use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-11.txt").unwrap();
	
	let mut values : Vec<Vec<i32>> = contents.lines().map(|s| s.chars().map(|c| c.to_digit(10).map(|i| i32::try_from(i).unwrap()).unwrap()).collect()).collect();
	
	let flashes = vec![vec![false; 10]; 10];
	
	let mut step = 0;
	loop {
		step += 1;
		
		values = values.iter().map(|l| l.iter().map(|&v| v + 1).collect()).collect();
		let mut has_flashed = flashes.clone();
		
		while let Some((x, y)) = get_lvl_9(&values) {
			values[y][x] = -100;
			
			let ym = y > 0;
			let yM = y < 9;
			let xm = x > 0;
			let xM = x < 9;
			
			if ym {
				if xm { values[y-1][x-1] += 1; }
				if xM { values[y-1][x+1] += 1; }
				values[y-1][x] += 1;
			}
			if yM {
				if xm { values[y+1][x-1] += 1; }
				if xM { values[y+1][x+1] += 1; }
				values[y+1][x] += 1;
			}
			if xm { values[y][x-1] += 1; }
			if xM { values[y][x+1] += 1; }
		}
		
		values = values.iter().map(|l| l.iter().map(|&v| if v < 0 { 0 } else { v }).collect()).collect();
		
		if values.iter().map(|l| l.iter().sum::<i32>()).sum::<i32>() == 0 {
			break;
		}
	};
	
	println!("{}", step);
	
}

fn get_lvl_9 (ymap: &Vec<Vec<i32>>) -> Option<(usize, usize)> {
	for (y, xmap) in ymap.iter().enumerate() {
		for (x, val) in xmap.iter().enumerate() {
			if *val > 9 { return Some((x, y)); }
		}
	}
	
	None
}