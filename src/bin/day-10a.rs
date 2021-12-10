use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-10.txt").unwrap();
	
	let lines : Vec<&str> = contents.lines().collect();
	
	let mut score = 0;
	
	for line in lines {
		let mut stack : Vec<char> = Vec::new();
		
		for ch in line.chars() {
			if ch == '('
			|| ch == '['
			|| ch == '{'
			|| ch == '<' {
				stack.push(ch);
				continue;
			}
			let pair = (stack.pop().unwrap(), ch);
			if pair == ('(', ')')
			|| pair == ('[', ']')
			|| pair == ('{', '}')
			|| pair == ('<', '>') {
				continue;
			}
			
			match ch {
				')' => score += 3,
				']' => score += 57,
				'}' => score += 1197,
				'>' => score += 25137,
				_ => panic!("unexpected char")
			}
			
			break;
		}
	}
	
	println!("{}", score)
	
}