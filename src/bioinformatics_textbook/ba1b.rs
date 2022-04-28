use needletail::{Sequence};
use std::{fs::read_to_string, collections::HashMap};

#[allow(dead_code)]
pub fn ba1b(filename:&str) -> Result<(), Box<dyn std::error::Error>>{
	let input = read_to_string(filename)?;
	let mut input = input.lines();
	let seq = input.next().expect("no seq").trim().as_bytes();
	let k:u8 = input.next().expect("no pattern").trim().parse().expect("k is not a number");

	let mut hm = HashMap::new();

	for kmer in seq.kmers(k) {
		let n = hm.entry(kmer).or_insert(0);
		*n += 1;
	}

	let max_value = hm.values().max().unwrap();
	// dbg!(max_value);
	for (k,v) in hm.iter(){
		if v == max_value {
			print!("{} ", String::from_utf8(k.to_vec()).expect("cannot convert from u8 to string"))
		}
	}
	print!("\n");

	Ok(())
}

