use std::fs;
use std::cmp;
use std::collections::HashMap;

fn main() {
	let contents = fs::read_to_string("./inputs/input-05.txt").unwrap();
	let lines = contents.lines();
	
	// Parse strings
	let lines = lines.map(|l| { // iterate over lines
		let mut pair = l.split("->").map(|s| { // iterate over points
			let mut pair = s.trim().split(',');
			let left = pair.next().unwrap().parse().unwrap();
			let right = pair.next().unwrap().parse().unwrap();
			(left, right)
		});
		let left = pair.next().unwrap();
		let right = pair.next().unwrap();
		(left, right)
	});
	
	let lines : Vec<((i32,i32),(i32,i32))>= lines.collect();
	
	let mut points : HashMap<(i32,i32), u32>= HashMap::new();
	
	let mut maxx = 0;
	let mut maxy = 0;
	
	for line in lines {
		count_line(line, &mut points);
		let ((xa,ya),(xb,yb)) = line;
		let linemaxx = cmp::max(xa, xb);
		let linemaxy = cmp::max(ya, yb);
		maxx = cmp::max(maxx, linemaxx);
		maxy = cmp::max(maxy, linemaxy);
	}
	
	let count = points.iter().filter(|(_, c)| **c > 1).count();
	//print_diagram(&points, maxx, maxy);
	println!("{}", count);	
}

fn print_diagram(map: &HashMap<(i32,i32), i32>, maxx: i32, maxy: i32) {
	for y in 0..=maxy {
		for x in 0..=maxx {
			let point = match map.get(&(x,y)) {
				Some(val) => val.to_string(),
				None => String::from(".")
			};
			print!("{}", point);
		}
		print!("\n");
	}
}

fn count_line(((xa,ya),(xb,yb)): ((i32,i32),(i32,i32)), map: &mut HashMap<(i32,i32), u32>) {
	let fromx = cmp::min(xa,xb);
	let fromy = cmp::min(ya,yb);
	let tox = cmp::max(xa,xb);
	let toy = cmp::max(ya,yb);
	
	let range = cmp::max(tox-fromx, toy-fromy);
	
	let addx = (xb - xa).signum();
	let addy = (yb - ya).signum();
	
	for i in 0..=range {
		let count = map.entry((xa+addx*i, ya+addy*i)).or_insert(0);
		*count += 1;
	}
}
