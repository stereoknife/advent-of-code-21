use std::fs;
use regex::Regex;

fn main() {
	let contents = fs::read_to_string("./inputs/input-08.txt").unwrap();
	
	let lines : Vec<&str> = contents.lines().collect();
	let lines : Vec<(Vec<&str>, Vec<&str>)> = lines.iter().map(
		|s| {
			let mut x = s.split("|").map(|v| v.trim().split(" ").collect());
			(x.next().unwrap(), x.next().unwrap())
		}
	).collect();
	
	let mut count = 0;
	
	for (control, vals) in lines {
		// unique lengths
		let s1 = *control.iter().filter(|&&s| s.len() == 2).next().unwrap();
		let s4 = *control.iter().filter(|&&s| s.len() == 4).next().unwrap();
		let s7 = *control.iter().filter(|&&s| s.len() == 3).next().unwrap();
		let s8 = *control.iter().filter(|&&s| s.len() == 7).next().unwrap();

		// length 6
		let s9 = {
			let regex = Regex::new(format!("(.*[{0}]){{4}}", s4).as_str()).unwrap();
			*control.iter().filter(|&&s| s.len() == 6 && regex.is_match(s)).next().unwrap()
		};
		let s0 = {
			let regex = Regex::new(format!("(.*[{0}]){{3}}", s7).as_str()).unwrap();
			*control.iter().filter(|&&s| s.len() == 6 && regex.is_match(s) && s != s9).next().unwrap()
		};
		let s6 = {
			*control.iter().filter(|&&s| s.len() == 6 && s != s0 && s != s9).next().unwrap()
		};
		
		// length 7
		let s3 = {
			let regex = Regex::new(format!("(.*[{0}]){{3}}", s7).as_str()).unwrap();
			*control.iter().filter(|&&s| s.len() == 5 && regex.is_match(s)).next().unwrap()
		};
		let s5 = {
			let regex = Regex::new(format!("(.*[{0}]){{3}}", s4).as_str()).unwrap();
			*control.iter().filter(|&&s| s.len() == 5 && regex.is_match(s) && s != s3).next().unwrap()
		};
		let s2 = {

			*control.iter().filter(|&&s| s.len() == 5 && s != s3 && s != s5).next().unwrap()
		};
		
		let rx1 = rx(s1);
		let rx2 = rx(s2);
		let rx3 = rx(s3);
		let rx4 = rx(s4);
		let rx5 = rx(s5);
		let rx6 = rx(s6);
		let rx7 = rx(s7);
		let rx8 = rx(s8);
		let rx9 = rx(s9);
		let rx0 = rx(s0);
		
		let out : Vec<i32> = vals.iter().map(|&s| {
			if rx0.is_match(s) { return 0 };
			if rx1.is_match(s) { return 1 };
			if rx2.is_match(s) { return 2 };
			if rx3.is_match(s) { return 3 };
			if rx4.is_match(s) { return 4 };
			if rx5.is_match(s) { return 5 };
			if rx6.is_match(s) { return 6 };
			if rx7.is_match(s) { return 7 };
			if rx8.is_match(s) { return 8 };
			if rx9.is_match(s) { return 9 };
			-1
		}).collect();
		
		//println!("{:?}", to_decimal(&out));
		
		count += to_decimal(&out);
	}
	
	println!("{}", count);
}

fn rx(number: &str) -> Regex {
	Regex::new(format!("^[{}]{{{}}}$", number, number.len()).as_str()).unwrap()
}

fn to_decimal(v: &Vec<i32>) -> i32 {
	v[0] * 1000 + v[1] * 100 + v[2] * 10 + v[3]
}
