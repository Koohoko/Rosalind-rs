use std::error::Error;
use needletail::{Sequence, parse_fastx_file};

#[allow(dead_code)]
pub fn revp(filename: &str) -> Result<(), Box<dyn Error>>{
	let mut reader = parse_fastx_file(filename)?;
	let min_length:usize = 4;
	let max_length:usize = 12;
	let record = reader.next().expect("invalid fasta record");
	
	let seqrec = record?;
	let seq = seqrec.seq();
	let seq_length = seqrec.num_bases();
	'outer: for i in 0..=(seq_length-min_length) {
		for j in (i+min_length)..=(seq_length){
			let length_ij = j-i;
			if length_ij % 2 !=0 {
				continue;
			}
			if length_ij > max_length {
				continue 'outer;
			}
			let seq_tmp = &seq[i..(i+length_ij/2)];
			let seq_rc = &seq[(i+length_ij/2)..j];
			let seq_rc = seq_rc.reverse_complement();
			if seq_tmp.to_vec() == seq_rc {
				println!("{} {}", i+1, length_ij)
			}
		}
	}

	Ok(())
}
