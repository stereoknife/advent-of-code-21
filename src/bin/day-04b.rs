use std::fs;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Cell {
	value: u32,
	drawn: bool
}

fn main() {
	let contents = fs::File::open("./inputs/input-04.txt").unwrap();
	let mut reader = BufReader::new(contents);
	
	let nums_v : Vec<u32> = {
		let mut line = String::new();
		reader.read_line(&mut line).unwrap();
		line.split(',').map(|s| s.trim().parse().unwrap()).collect()
	};
	
	let mut nums = nums_v.iter();
	let mut boards : Vec<Vec<Cell>> = {
		let mut raw = String::new();
		reader.read_to_string(&mut raw).unwrap();
		
		let mut vals : Vec<Cell>= raw.split_whitespace().map(|v| Cell { value: v.parse::<u32>().unwrap(), drawn: false }).collect();
		let mut boards : Vec<Vec<Cell>> = Vec::new();
		
		while !vals.is_empty() {
			boards.push(vals.drain(..25).collect());
		}
		
		boards
	};
	
	let mut last_num = 0;
	let mut last_board: Vec<Cell> = Vec::new();
	
	while !boards.is_empty() {
		let num = nums.next().unwrap();
		
		for board in &mut boards {
			mark_board(*num, board);
		}
		
		while let (true, i) = check_boards(&boards) {
			last_num = *num;
			last_board = boards.remove(i);
		}
	};
	
	let points = last_board.iter().fold(0, |acc, b| acc + if !b.drawn { b.value } else { 0 });
	
	println!("{}", points * last_num);
}

fn mark_board(num: u32, board: &mut Vec<Cell>) {
	for cell in board {
		if cell.value == num { cell.drawn = true }
	}
}

fn check_boards(boards: &Vec<Vec<Cell>>) -> (bool, usize) {
	for (i, board) in boards.iter().enumerate() {
		if check_board(board) { return (true, i); }
	}
	(false, 0)
}

fn check_board(board: &Vec<Cell>) -> bool {
	check_rows(board) || check_cols(board)
}

fn check_rows(board: &Vec<Cell>) -> bool {
	let mut win = false;
	for i in 0..5 {
		win |= board.iter().skip(i*5).take(5).fold(true, |acc, val| val.drawn && acc);
	};
	win
}

fn check_cols(board: &Vec<Cell>) -> bool {
	let mut win = false;
	for i in 0..5 {
		win |= board.iter().skip(i).step_by(5).fold(true, |acc, val| val.drawn && acc)
	};
	win
}