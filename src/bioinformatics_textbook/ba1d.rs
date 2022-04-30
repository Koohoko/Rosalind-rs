use std::error::Error;
use std::fs::read_to_string;
use needletail::Sequence;

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>>{
	let input = read_to_string(file_path)?;
	let mut input = input.lines();
	let pattern = input.next().unwrap().as_bytes();
	let seq = input.next().unwrap().as_bytes();

	let results = pattern_match(seq, pattern);
	for id in results{
		print!("{} ", id);
	}

	Ok(())
}

pub fn pattern_match(text:&[u8], p:&[u8]) -> Vec<usize>{
	let k = p.len();
	let mut id:usize = 0;
	let mut matches:Vec<usize> = Vec::new();
	for kmer in text.kmers(k as u8) {
		if kmer == p {
			matches.push(id);
		}
		id += 1;
	}
	matches
}