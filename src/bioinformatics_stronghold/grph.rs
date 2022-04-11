use std::error::Error;
use needletail::{parse_fastx_file};

#[allow(dead_code)]
pub fn grph(filename :&str) -> Result<(), Box<dyn Error>> {
	let mut reader = parse_fastx_file(filename)?;

	while let Some(record) = reader.next() {
        let seqrec = record?;
		let seq = seqrec.seq();
		let tail_3 = &seq[(seq.len()-3)..(seq.len())];

		let mut reader_j = parse_fastx_file(filename)?;
		while let Some(record_j) = reader_j.next() {
			let seqrec_j = record_j?;
			if seqrec.id() == seqrec_j.id() {
				continue;
			}
			let seq_j = seqrec_j.seq();
			let head_3 = &seq_j[0..3];
			if tail_3 == head_3 {
				println!("{} {}", std::str::from_utf8(seqrec.id()).unwrap(), std::str::from_utf8(seqrec_j.id()).unwrap())
			}
		}
    }
	
	Ok(())
}