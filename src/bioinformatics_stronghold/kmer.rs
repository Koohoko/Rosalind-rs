use std::error::Error;
use needletail::{Sequence, parse_fastx_file};
use std::collections::HashMap;
use super::lexf;

pub fn kmer(filename:&str) -> Result<(), Box<dyn Error>>{
	let k = 4_usize;

	let mut kmers_all:Vec<Vec<char>> = Vec::new();
	let alphabets = vec!['A', 'C', 'G', 'T'];
	let mut str_tmp = vec![alphabets[0]; k];
	lexf::recursive(0, &mut str_tmp, &alphabets, k, &mut kmers_all);

	let kmers_all:Vec<Vec<u8>> = kmers_all.into_iter().map(|x| x.into_iter().map(|y|y as u8).collect::<Vec<u8>>()).collect();
	// dbg!(&kmers_all);
	
	let mut hm = HashMap::new();
	
	let mut records = parse_fastx_file(filename)?;
	if let Some(record) = records.next() {
		let seqrec = record?;
		let seq = seqrec.normalize(false);

		for kmer in seq.kmers(4) {
			let x = hm.entry(kmer.to_vec()).or_insert(0);
			*x += 1;
		}
	}

	for kmer in kmers_all {
		let x = hm.get(&kmer);
		match x {
			Some(v) => print!("{} ", v),
			None => print!("{} ", 0),
		}
	}
	println!("");

	Ok(())
}
