use std::fs;
use std::collections::HashMap;

#[derive(Clone)]
struct Room<'a> {
	small: bool,
	visited: bool,
	neighbours: Vec<&'a str>
}

fn main() {
	let contents = fs::read_to_string("./inputs/input-12.txt").unwrap();
	
	let lines = contents.lines();
	let mut map = {
		let mut out: HashMap<&str, Room> = HashMap::new();
		
		for line in lines {
			let mut vals = line.split('-');
			let key = vals.next().unwrap();
			let val = vals.next().unwrap();
			
			out.entry(key)
			   .and_modify(|r| r.neighbours.push(val))
			   .or_insert(Room {
					   small: key == key.to_lowercase(),
					   visited: false,
					   neighbours: vec![val]
				   });
				   
			out.entry(val)
			   .and_modify(|r| r.neighbours.push(key))
			   .or_insert(Room {
					   small: val == val.to_lowercase(),
					   visited: false,
					   neighbours: vec![key]
				   });
		}
		
		out
	};
	
	let paths = navigate(&mut map, "start", false);
		
	println!("{}", paths);
	
}

fn navigate(map: &mut HashMap<&str, Room>, room: &str, small_cave_visited: bool) -> u32 {
	map.get_mut(room).unwrap().visited = true;
	if room == "end" { return 1; }
	let mut paths = 0;
	for &neighbour in &map[room].neighbours {
		let n = &map[neighbour];
		if !n.visited || !n.small {
			paths += navigate(&mut map.clone(), neighbour, small_cave_visited);
		} else if !small_cave_visited && neighbour != "start" {
			paths += navigate(&mut map.clone(), neighbour, true);
		}
	}
	paths
}
