use needletail::{Sequence};
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn ba1a(filename:&str) -> Result<(), Box<dyn std::error::Error>>{
	let input = read_to_string(filename)?;
	let mut input = input.lines();
	let seq = input.next().expect("no seq").trim().as_bytes();
	let pattern = input.next().expect("no pattern").trim().as_bytes();

	println!("{}", pattern_count(seq, pattern));

	Ok(())
}

fn pattern_count(text:&[u8], p:&[u8]) -> usize{
	let k = p.len();
	let mut count:usize = 0;
	for kmer in text.kmers(k as u8) {
		if kmer == p {
			count += 1;
		}
	}
	count
}