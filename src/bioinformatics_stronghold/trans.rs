use std::error::Error;
// use std::fs::read_to_string;
use needletail::{parse_fastx_file, Sequence};

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>>{
	let mut records = parse_fastx_file(file_path)?;
	let seq1 = records.next().unwrap()?.normalize(false).to_vec();
	let seq2 = records.next().unwrap()?.normalize(false).to_vec();

	let result = trans(seq1, seq2);
	println!("{}", result);

	Ok(())
}

fn trans(seq1:Vec<u8>, seq2:Vec<u8>) -> f64 {
	let mut num_transition = 0_usize;
	let mut num_transversion = 0_usize;
	let mut seq_zip = seq1.into_iter().zip(seq2.into_iter());

	while let Some(x) = seq_zip.next(){
		// dbg!(&x);
		match x {
			(b1, b2 ) if b1==b2 => continue,
			(b1, b2 ) if b1==b'T' && b2==b'C' => num_transition +=1,
			(b1, b2 ) if b1==b'C' && b2==b'T' => num_transition +=1,
			(b1, b2 ) if b1==b'A' && b2==b'G' => num_transition +=1,
			(b1, b2 ) if b1==b'G' && b2==b'A' => num_transition +=1,
			_ => num_transversion += 1,
		}
	}
	// dbg!(&num_transition);
	// dbg!(&num_transversion);

	num_transition as f64 /num_transversion as f64 
}