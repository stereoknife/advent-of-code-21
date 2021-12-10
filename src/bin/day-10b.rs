use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-10.txt").unwrap();
	
	let lines : Vec<&str> = contents.lines().collect();
	
	let mut scores : Vec<usize> = Vec::new();
	
	'line: for line in lines {
		let mut stack : Vec<char> = Vec::new();
		let mut score = 0;
		
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
			continue 'line;
		}
		
		for ch in stack.iter().rev() {
			score *= 5;
			match ch {
				'(' => score += 1,
				'[' => score += 2,
				'{' => score += 3,
				'<' => score += 4,
				_ => panic!("unexpected char")
			}
		}
		
		scores.push(score);
	}
	
	scores.sort();
	
	println!("{}", scores[scores.len() / 2])
	
}