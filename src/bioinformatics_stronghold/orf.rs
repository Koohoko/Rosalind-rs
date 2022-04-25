use std::error::Error;
use needletail::{parse_fastx_file, Sequence};
use protein_translate::translate; //use array indexing is faster than hashmap https://github.com/dweb0/protein-translate
use itertools::Itertools;


#[allow(dead_code)]
pub fn orf(filename :&str) -> Result<(), Box<dyn Error>> {
	let mut reader = parse_fastx_file(filename)?;
	while let Some(r) = reader.next() {
		let seqrec = r?;
		let seq = seqrec.normalize(false);
		let mut orfs = find_orf(seq.to_vec());
		let seq_rc = seqrec.normalize(false).reverse_complement();
		let mut orfs_r = find_orf(seq_rc);
		orfs.append(&mut orfs_r);
		let orfs:Vec<_> = orfs.into_iter().unique().collect();
		for orf in orfs {
			println!("{}", orf);
		}
	}

	Ok(())
}

fn find_orf(seq:Vec<u8>) -> Vec<String>{
	let mut pos_start = Vec::new();
	let mut pos_stop = Vec::new();
	let mut orfs = Vec::new();
	for i in 0..(seq.len()-3) {
		let tmp = &seq[i..(i+3)];
		match tmp {
			b"ATG" => pos_start.push(i),
			b"TAA" => pos_stop.push(i),
			b"TAG" => pos_stop.push(i),
			b"TGA" => pos_stop.push(i),
			_ => (),
		}
	}

	'outer: for pos_start_i in &pos_start {
		for pos_stop_i in &pos_stop {
			if (pos_stop_i > pos_start_i) && (pos_stop_i-pos_start_i)%3==0 {
				let prot_i = translate(&seq[*pos_start_i..*pos_stop_i]);
				orfs.push(prot_i);
				continue 'outer;
			}
		}
	}
	orfs
}