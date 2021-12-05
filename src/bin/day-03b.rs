use std::fs;

fn main() {
	let contents = fs::read_to_string("./inputs/input-03.txt")
		.expect("No input file provided");
	
	let lines : Vec<&str> = contents.lines().collect();
	let input = {
		let mut v : Vec<Vec<u32>> = Vec::new();
		for line in &lines {
			v.push(line.chars().map(|c| c.to_digit(2).unwrap()).collect());
		}
		v
	};
	
	
	let oxygen = {
		let mut o = input.clone();
		let mut i = 0;
		loop {
			let sum : u32 = o.iter().fold(0, |acc, x| acc + x[i]);
			let len = u32::try_from(o.len()).unwrap();
			let cmp = if sum >= len - sum { 1 } else { 0 };
			o = o.into_iter()
				.filter(|v| v[i] == cmp)
				.collect();
			if o.len() == 1 { break; }
			i += 1;
		}
		digits_to_numbers(2, &o[0])
	};
	
	let co2 = {
		let mut o = input.clone();
		let mut i = 0;
		loop {
			let sum : u32 = o.iter().fold(0, |acc, x| acc + x[i]);	
			let len = u32::try_from(o.len()).unwrap();
			let cmp = if sum >= len - sum { 0 } else { 1 };
			o = o.into_iter()
				.filter(|v| v[i] == cmp)
				.collect();
			if o.len() <= 1 { break; }
			i += 1;
		}
		digits_to_numbers(2, &o[0])
	};
	
	println!("{:?}", oxygen * co2);
	
	fn digits_to_numbers(base : u32, digits : &Vec<u32>) -> u32 {
		let mut out = 0;
		let mut mul = 1;
		
		for digit in digits.iter().rev() {
			out = out + digit * mul;
			mul *= base;
		}
		
		out
	}
}