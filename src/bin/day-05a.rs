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
	
	let lines : Vec<((u32,u32),(u32,u32))>= lines.filter(|((xa,ya),(xb,yb))| xa == xb || ya == yb).collect();
	
	let mut points : HashMap<(u32,u32), u32>= HashMap::new();
	
	for line in lines {
		count_line(line, &mut points);
	}
	
	let count = points.iter().filter(|(_, c)| **c > 1).count();
	println!("{}", count);	
}

fn count_line(((xa,ya),(xb,yb)): ((u32,u32),(u32,u32)), map: &mut HashMap<(u32,u32), u32>) {
	let fromx = cmp::min(xa,xb);
	let fromy = cmp::min(ya,yb);
	let tox = cmp::max(xa,xb);
	let toy = cmp::max(ya,yb);
	
	if fromx != tox {
		for x in fromx..=tox {
			let count = map.entry((x,fromy)).or_insert(0);
			*count += 1;
		}
	} else if fromy != toy {
		for y in fromy..=toy {
			let count = map.entry((fromx,y)).or_insert(0);
			*count += 1;
		}
	}
}
